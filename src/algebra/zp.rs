use std::{
    array::TryFromSliceError,
    convert::{TryFrom, TryInto},
    iter::Sum,
    ops::{Add, Mul, Neg, Sub},
};

use num_traits::{Inv, One, Pow, Zero};
use rand::{distributions::Standard, prelude::Distribution};
use thiserror::Error;

use crate::bytes::{Decoding, Deserialize, Encoding, Serialize};

use super::{fields::Zn, traits::Group};

#[derive(Clone, PartialEq, Eq)]
pub struct Zp<const N: usize>(Zn<N>);

#[derive(Debug, Error)]
#[error("Zp cannot be 0")]
pub struct IsZero;

impl<const N: usize> TryFrom<usize> for Zp<N> {
    type Error = IsZero;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Zn::from(value).try_into()
    }
}

impl<const N: usize> TryFrom<Zn<N>> for Zp<N> {
    type Error = IsZero;

    fn try_from(value: Zn<N>) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(IsZero)
        } else {
            Ok(Self(value))
        }
    }
}

impl<const N: usize> Distribution<Zp<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Zp<N> {
        loop {
            let zn: Zn<N> = self.sample(rng);
            if let Ok(zp) = zn.try_into() {
                break zp;
            }
        }
    }
}

impl<const N: usize> Group for Zp<N> {}

impl<const N: usize> Add for Zp<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::try_from(self.0 * rhs.0).unwrap()
    }
}

impl<const N: usize> Zero for Zp<N> {
    fn zero() -> Self {
        Zn::one().try_into().unwrap()
    }

    fn is_zero(&self) -> bool {
        self.0.is_one()
    }
}

impl<const N: usize> Neg for Zp<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.0.inv().try_into().unwrap()
    }
}

impl<const N: usize> Sub for Zp<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<const N: usize> Sum for Zp<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter()
            .map(|x| x.0)
            .product::<Zn<N>>()
            .try_into()
            .unwrap()
    }
}

impl<const N: usize> Mul<isize> for Zp<N> {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        match rhs.try_into() {
            Ok(rhs) => self.0.pow(rhs).try_into().unwrap(),
            Err(_) => (-self) * (-rhs),
        }
    }
}

impl<const N: usize> Encoding for Zp<N> {
    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Self> {
        assert!(N > 256);
        stream
            .next()
            .map(|x| usize::from(x + 1).try_into().unwrap())
    }
}

impl<const N: usize> Decoding for Zp<N> {
    type Error = <usize as TryInto<u8>>::Error;

    fn decode(self) -> Result<Vec<u8>, Self::Error> {
        usize::from(self.0).try_into().map(|x: u8| vec![x - 1])
    }
}

impl<const N: usize> Serialize for Zp<N> {
    fn serialize(self) -> Vec<u8> {
        self.0.serialize()
    }
}

impl<const N: usize> Deserialize for Zp<N> {
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
pub enum ZpDeserError {
    #[error("Not enough bytes")]
    NotEnoughBytes(#[from] TryFromSliceError),
    #[error(transparent)]
    IsZero(#[from] IsZero),
}
