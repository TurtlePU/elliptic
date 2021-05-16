use std::string::FromUtf8Error;

use hex::FromHexError;
use rand::Rng;

use super::encryption::*;

pub struct Stringer<X>(pub X);

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub enum StringDecryptionError<S, R, C> {
    NotAHex(FromHexError),
    Deserialization(S),
    Decryption(R),
    Decoding(C),
    NotUtf8(FromUtf8Error),
}

pub enum ConsumeError<E> {
    Deserialization(E),
    EmptyStream,
    HugeStream,
}

pub trait Encoding: Sized {
    fn encode(stream: impl Iterator<Item = u8>) -> Self;
}

pub trait Decoding {
    type Error;

    fn decode(self) -> Result<Vec<u8>, Self::Error>;
}

pub trait Serialize {
    fn serialize(self) -> Vec<u8>;
}

pub trait Deserialize: Sized {
    type Error;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error>;
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize,
{
    type Error = T::Error;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        let mut result = vec![];
        while let Some(item) = T::deserialize(stream)? {
            result.push(item);
        }
        Ok(Some(result))
    }
}

impl<T, U> Deserialize for (T, U)
where
    T: Deserialize,
    U: Deserialize,
{
    type Error = Either<T::Error, U::Error>;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        let left = match T::deserialize(stream).map_err(Either::Left)? {
            Some(left) => left,
            None => return Ok(None),
        };
        let right = match U::deserialize(stream).map_err(Either::Right)? {
            Some(right) => right,
            None => return Ok(None),
        };
        Ok(Some((left, right)))
    }
}

pub fn consume<T: Deserialize>(
    iter: impl IntoIterator<Item = u8>,
) -> Result<T, ConsumeError<T::Error>> {
    let mut stream = iter.into_iter();
    let result = T::deserialize(&mut stream)
        .map_err(ConsumeError::Deserialization)?
        .ok_or(ConsumeError::EmptyStream)?;
    if stream.next().is_some() {
        Err(ConsumeError::HugeStream)
    } else {
        Ok(result)
    }
}

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
        rng: &mut impl Rng,
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

    fn generate_key(&self, rng: &mut impl Rng) -> Self::Secret {
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
        rng: &mut impl Rng,
        message: Self::Message,
    ) -> Self::Cipher {
        let encryptor = &self.0;
        let cipher = encryptor
            .encrypt(rng, X::Message::encode(message.bytes()))
            .serialize();
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
        ConsumeError<<X::Cipher as Deserialize>::Error>,
        X::Error,
        <X::Message as Decoding>::Error,
    >;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        let cipher = consume(hex::decode(cipher)?)
            .map_err(Self::Error::Deserialization)?;
        let decryptor = &self.0;
        let message_bytes = decryptor
            .decrypt(cipher)
            .map_err(Self::Error::Decryption)?
            .decode()
            .map_err(Self::Error::Decoding)?;
        String::from_utf8(message_bytes).map_err(Self::Error::NotUtf8)
    }
}

impl<S, R, C> From<FromHexError> for StringDecryptionError<S, R, C> {
    fn from(err: FromHexError) -> Self {
        Self::NotAHex(err)
    }
}
