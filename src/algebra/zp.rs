use std::{
    array::TryFromSliceError,
    convert::{TryFrom, TryInto},
    iter::Sum,
    ops::{Add, Mul, Neg, Sub},
};

use num_bigint::{BigInt, BigUint};
use num_traits::{Inv, One, Pow, Zero};
use rand::{distributions::Standard, prelude::Distribution};
use thiserror::Error;

use crate::bytes::{Decoding, Deserialize, Encoding, Serialize};

use super::{
    fields::zn::{BigPrime, Zn},
    traits::Group,
};

pub struct Zp<N>(Zn<N>);

#[derive(Debug, Error)]
#[error("Zp cannot be 0")]
pub struct IsZero;

impl<N: BigPrime> TryFrom<usize> for Zp<N> {
    type Error = IsZero;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Zn::from(value).try_into()
    }
}

impl<N: BigPrime> TryFrom<Zn<N>> for Zp<N> {
    type Error = IsZero;

    fn try_from(value: Zn<N>) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(IsZero)
        } else {
            Ok(Self(value))
        }
    }
}

impl<N: BigPrime> Distribution<Zp<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Zp<N> {
        loop {
            let zn: Zn<N> = self.sample(rng);
            if let Ok(zp) = zn.try_into() {
                break zp;
            }
        }
    }
}

impl<N> Clone for Zp<N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<N> PartialEq for Zp<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<N> Eq for Zp<N> where Zn<N>: Eq {}

impl<N: BigPrime> Group for Zp<N> {}

impl<N: BigPrime> Add for Zp<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::try_from(self.0 * rhs.0).unwrap()
    }
}

impl<N: BigPrime> Zero for Zp<N> {
    fn zero() -> Self {
        Zn::one().try_into().unwrap()
    }

    fn is_zero(&self) -> bool {
        self.0.is_one()
    }
}

impl<N: BigPrime> Neg for Zp<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.0.inv().try_into().unwrap()
    }
}

impl<N: BigPrime> Sub for Zp<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<N: BigPrime> Sum for Zp<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter()
            .map(|x| x.0)
            .product::<Zn<N>>()
            .try_into()
            .unwrap()
    }
}

impl<N: BigPrime> Mul<BigInt> for Zp<N> {
    type Output = Self;

    fn mul(self, rhs: BigInt) -> Self::Output {
        match rhs.try_into() {
            Ok(rhs) => self.0.pow(rhs).try_into().unwrap(),
            Err(err) => -self * -err.into_original(),
        }
    }
}

impl<N: BigPrime> Encoding for Zp<N> {
    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let len = bytevec_len::<N>();
        let bytes: Vec<_> = (0..len).filter_map(|_| stream.next()).collect();
        if bytes.len() == 0 {
            None
        } else {
            let n = BigUint::from_bytes_le(&bytes) + BigUint::one();
            Some(Self(Zn::from(n)))
        }
    }
}

impl<N: BigPrime> Decoding for Zp<N> {
    type Error = TooBig;

    fn decode(self) -> Result<Vec<u8>, Self::Error> {
        let len = bytevec_len::<N>();
        let mut bytes = (BigUint::from(self.0) - BigUint::one()).to_bytes_le();
        if bytes.len() > len {
            Err(TooBig(BigUint::from_bytes_le(&bytes)))
        } else {
            bytes.resize(len, 0);
            Ok(bytes)
        }
    }
}

fn bytevec_len<N: BigPrime>() -> usize {
    (N::value() - BigUint::one()).to_bytes_le().len() - 1
}

impl<N: BigPrime> Serialize for Zp<N> {
    fn serialize(self) -> Vec<u8> {
        self.0.serialize()
    }
}

impl<N: BigPrime> Deserialize for Zp<N> {
    type Error = ZpDeserError;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        match Zn::<N>::deserialize(stream) {
            Ok(Some(x)) => Ok(Some(x.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}

#[derive(Debug, Error)]
#[error("{0} is too big for decoding")]
pub struct TooBig(BigUint);

#[derive(Debug, Error)]
pub enum ZpDeserError {
    #[error("Not enough bytes")]
    NotEnoughBytes(#[from] TryFromSliceError),
    #[error(transparent)]
    IsZero(#[from] IsZero),
}
