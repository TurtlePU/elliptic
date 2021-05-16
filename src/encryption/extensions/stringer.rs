use std::{error::Error, string::FromUtf8Error};

use hex::FromHexError;
use rand::RngCore;
use thiserror::Error;

use crate::{bytes::*, encryption::base::encryption::*};

pub struct Stringer<X>(pub X);

impl<X> Enc for Stringer<X> {
    type Message = String;
    type Cipher = String;
}

impl<X> PublicKeyEncryption for Stringer<X>
where
    X: PublicKeyEncryption,
    X::Message: Encoding + Decoding,
    X::Cipher: Serialize + Deserialize,
{
    type PublicKey = Stringer<X::PublicKey>;
    type Secret = Stringer<X::Secret>;

    fn generate_keys(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_keys(rng);
        (Stringer(enc), Stringer(dec))
    }
}

impl<X, K> PrivateKeyEncryption for Stringer<X>
where
    X: PrivateKeyEncryption<Secret = K>,
    K: PrivateKey,
    K::Message: Encoding + Decoding,
    K::Cipher: Serialize + Deserialize,
{
    type Secret = Stringer<K>;

    fn generate_key(&self, rng: &mut dyn RngCore) -> Self::Secret {
        Stringer(self.0.generate_key(rng))
    }
}

impl<X> Encryptor for Stringer<X>
where
    X: Encryptor,
    X::Message: Encoding,
    X::Cipher: Serialize,
{
    fn encrypt(
        &self,
        rng: &mut dyn RngCore,
        message: Self::Message,
    ) -> Self::Cipher {
        let message = match consume_enc(message.bytes()) {
            Ok(message) => message,
            Err(ConsumeEncError::EmptyStream) => return String::default(),
            Err(ConsumeEncError::HugeStream) => {
                let type_name = std::any::type_name::<X::Message>();
                panic!("String is too long to be encoded by {}", type_name);
            }
        };
        let encryptor = &self.0;
        let cipher = encryptor.encrypt(rng, message).serialize();
        hex::encode(cipher)
    }
}

impl<X> Decryptor for Stringer<X>
where
    X: Decryptor,
    X::Message: Decoding,
    X::Cipher: Deserialize,
{
    type Error = StringDecryptionError<
        ConsumeDeserError<<X::Cipher as Deserialize>::Error>,
        X::Error,
        <X::Message as Decoding>::Error,
    >;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        let cipher = consume_deser(hex::decode(cipher)?)
            .map_err(Self::Error::Deserialization)?;
        let decryptor = &self.0;
        let message_bytes = decryptor
            .decrypt(cipher)
            .map_err(Self::Error::Decryption)?
            .decode()
            .map_err(Self::Error::Decoding)?;
        Ok(String::from_utf8(message_bytes)?)
    }
}

#[derive(Debug, Error)]
pub enum StringDecryptionError<
    S: Error + 'static,
    R: Error + 'static,
    C: Error + 'static,
> {
    #[error("Cipher is not a hex string")]
    NotAHex(#[from] FromHexError),
    #[error("Deserialization error")]
    Deserialization(#[source] S),
    #[error("Decryption error")]
    Decryption(#[source] R),
    #[error("Decoding error")]
    Decoding(#[source] C),
    #[error("Message is not UTF-8")]
    NotUtf8(#[from] FromUtf8Error),
}
