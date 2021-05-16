use std::error::Error;

use thiserror::Error;

pub trait Encoding: Sized {
    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Self>;
}

impl<T: Encoding> Encoding for Vec<T> {
    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let mut result = vec![];
        while let Some(value) = T::encode(stream) {
            result.push(value);
        }
        Some(result)
    }
}

pub enum ConsumeEncError {
    EmptyStream,
    HugeStream,
}

pub fn consume_enc<T: Encoding>(
    iter: impl IntoIterator<Item = u8>,
) -> Result<T, ConsumeEncError> {
    let mut stream = iter.into_iter();
    let result = T::encode(&mut stream).ok_or(ConsumeEncError::EmptyStream)?;
    if stream.next().is_some() {
        Err(ConsumeEncError::HugeStream)
    } else {
        Ok(result)
    }
}

pub trait Decoding {
    type Error: Error + 'static;

    fn decode(self) -> Result<Vec<u8>, Self::Error>;
}

impl<T: Decoding> Decoding for Vec<T> {
    type Error = T::Error;

    fn decode(self) -> Result<Vec<u8>, Self::Error> {
        let nested = self
            .into_iter()
            .map(T::decode)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(nested.into_iter().flatten().collect())
    }
}

pub trait Serialize {
    fn serialize(self) -> Vec<u8>;
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(self) -> Vec<u8> {
        self.into_iter().map(T::serialize).flatten().collect()
    }
}

impl<T: Serialize, U: Serialize> Serialize for (T, U) {
    fn serialize(self) -> Vec<u8> {
        let (left, right) = self;
        let mut left = left.serialize();
        left.append(&mut right.serialize());
        left
    }
}

pub trait Deserialize: Sized {
    type Error: Error + 'static;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error>;
}

impl<T: Deserialize> Deserialize for Vec<T> {
    type Error = T::Error;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        let mut result = vec![];
        while let Some(value) = T::deserialize(stream)? {
            result.push(value);
        }
        Ok(Some(result))
    }
}

impl<T: Deserialize, U: Deserialize> Deserialize for (T, U) {
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

#[derive(Debug, Error)]
pub enum Either<L: Error + 'static, R: Error + 'static> {
    #[error(transparent)]
    Left(L),
    #[error(transparent)]
    Right(R),
}

#[derive(Debug, Error)]
pub enum ConsumeDeserError<E: Error + 'static> {
    #[error(transparent)]
    Deserialization(#[from] E),
    #[error("Stream is empty")]
    EmptyStream,
    #[error("Stream contains more than one item")]
    HugeStream,
}

pub fn consume_deser<T: Deserialize>(
    iter: impl IntoIterator<Item = u8>,
) -> Result<T, ConsumeDeserError<T::Error>> {
    let ref mut stream = iter.into_iter();
    let result =
        T::deserialize(stream)?.ok_or(ConsumeDeserError::EmptyStream)?;
    if stream.next().is_some() {
        Err(ConsumeDeserError::HugeStream)
    } else {
        Ok(result)
    }
}
