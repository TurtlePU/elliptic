use itertools::Itertools;
use num_traits::{One, Zero};

use crate::algebra::fields::Zn;

pub trait ToBytes: Sized {
    fn to_bytes(self) -> Vec<u8>;
}

pub trait FromBytes: Sized {
    type Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
}

pub trait FromBytesInfallible {
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait ByteCnt {
    const BYTE_CNT: usize;
}

impl<T: ToBytes> ToBytes for Vec<T> {
    fn to_bytes(self) -> Vec<u8> {
        self.into_iter().map(T::to_bytes).flatten().collect()
    }
}

impl<T: FromBytes + ByteCnt> FromBytes for Vec<T> {
    type Error = T::Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        bytes.chunks(T::BYTE_CNT).map(T::from_bytes).collect()
    }
}

impl<T: FromBytesInfallible + ByteCnt> FromBytesInfallible for Vec<T> {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.chunks(T::BYTE_CNT).map(T::from_bytes).collect()
    }
}

impl<T: FromBytes + ByteCnt, U: FromBytes> FromBytes for (T, U) {
    type Error = Either<T::Error, U::Error>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        let (left, right) = bytes.split_at(T::BYTE_CNT);
        Ok((
            T::from_bytes(left).map_err(Either::Left)?,
            U::from_bytes(right).map_err(Either::Right)?,
        ))
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<T, U> FromBytesInfallible for (T, U)
where
    T: FromBytesInfallible + ByteCnt,
    U: FromBytesInfallible,
{
    fn from_bytes(bytes: &[u8]) -> Self {
        let (left, right) = bytes.split_at(T::BYTE_CNT);
        (T::from_bytes(left), U::from_bytes(right))
    }
}

impl<T: ToBytes, U: ToBytes> ToBytes for (T, U) {
    fn to_bytes(self) -> Vec<u8> {
        let mut ans = self.0.to_bytes();
        ans.extend(self.1.to_bytes());
        ans
    }
}

impl<T: ByteCnt, U: ByteCnt> ByteCnt for (T, U) {
    const BYTE_CNT: usize = T::BYTE_CNT + U::BYTE_CNT;
}

const BYTE_BIT: usize = 8;

pub fn bitvec_from_bytes(bytes: &[u8]) -> Vec<Zn<2>> {
    bytes
        .iter()
        .map(|&x| {
            (0..BYTE_BIT).map(move |i| {
                if x & (1 << (BYTE_BIT - i)) != 0 {
                    Zn::one()
                } else {
                    Zn::zero()
                }
            })
        })
        .flatten()
        .collect()
}

pub fn bitvec_to_bytes(vec: Vec<Zn<2>>) -> Vec<u8> {
    vec.into_iter()
        .map(usize::from)
        .chunks(BYTE_BIT)
        .into_iter()
        .map(|chunk| chunk.fold(0, |sum, bit| (sum << 1) & bit) as u8)
        .collect()
}
