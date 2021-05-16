use std::string::FromUtf8Error;

use hex::FromHexError;
use rand::Rng;

use crate::bytes::{FromBytes, FromBytesInfallible, ToBytes};

use super::schemes::{
    Decryptor, Enc, Encryptor, PrivateKey, PrivateKeyEncryption,
    PublicKeyEncryption,
};

pub struct StringEncryption<S>(S);

impl<S> From<S> for StringEncryption<S> {
    fn from(scheme: S) -> Self {
        Self(scheme)
    }
}

impl<S> Enc for StringEncryption<S> {
    type Message = String;
    type Cipher = String;
}

impl<S> PublicKeyEncryption for StringEncryption<S>
where
    S: PublicKeyEncryption,
    S::Message: FromBytesInfallible + ToBytes,
    S::Cipher: FromBytes + ToBytes,
{
    type PublicKey = StringEncryption<S::PublicKey>;
    type Secret = StringEncryption<S::Secret>;

    fn generate_keys(
        &self,
        rng: &mut impl Rng,
    ) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_keys(rng);
        (StringEncryption(enc), StringEncryption(dec))
    }
}

impl<S, K> PrivateKeyEncryption for StringEncryption<S>
where
    S: PrivateKeyEncryption<Secret = K>,
    K: PrivateKey,
    K::Message: FromBytesInfallible + ToBytes,
    K::Cipher: FromBytes + ToBytes,
{
    type Secret = StringEncryption<K>;

    fn generate_key(&self, rng: &mut impl Rng) -> Self::Secret {
        StringEncryption(self.0.generate_key(rng))
    }
}

impl<E> Encryptor for StringEncryption<E>
where
    E: Encryptor,
    E::Message: FromBytesInfallible,
    E::Cipher: ToBytes,
{
    fn encrypt(
        &self,
        rng: &mut impl Rng,
        message: Self::Message,
    ) -> Self::Cipher {
        let message = E::Message::from_bytes(message.as_bytes());
        hex::encode(self.0.encrypt(rng, message).to_bytes())
    }
}

impl<D> Decryptor for StringEncryption<D>
where
    D: Decryptor,
    D::Cipher: FromBytes,
    D::Message: ToBytes,
{
    type Error =
        StringDecryptionError<<D::Cipher as FromBytes>::Error, D::Error>;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        use StringDecryptionError::*;
        let cipher = hex::decode(cipher).map_err(NotAHex)?;
        let cipher = D::Cipher::from_bytes(&cipher[..]).map_err(FromBytes)?;
        let message = self.0.decrypt(cipher).map_err(Decryption)?;
        String::from_utf8(message.to_bytes()).map_err(FromUTF8)
    }
}

impl<K> PrivateKey for StringEncryption<K>
where
    K: PrivateKey,
    K::Message: FromBytesInfallible + ToBytes,
    K::Cipher: FromBytes + ToBytes,
{
}

pub enum StringDecryptionError<B, D> {
    NotAHex(FromHexError),
    FromBytes(B),
    Decryption(D),
    FromUTF8(FromUtf8Error),
}
