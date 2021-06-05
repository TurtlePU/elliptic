use std::{
    convert::TryInto,
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
    traits::{FinGroup, Group},
};

pub trait Generator {
    fn generator() -> BigUint;
    fn order() -> BigUint;
}

pub struct Zp<N>(Zn<N>);

impl<N: BigPrime> From<BigUint> for Zp<N> {
    fn from(x: BigUint) -> Self {
        Self(Zn::from(x))
    }
}

impl<N: BigPrime + Generator> Distribution<Zp<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Zp<N> {
        let degree = rng.gen_range(BigUint::one()..N::order());
        N::generator().pow(degree).into()
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

impl<N: BigPrime + Generator> FinGroup for Zp<N> {
    fn order() -> BigUint {
        N::order()
    }
}

impl<N: BigPrime> Add for Zp<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<N: BigPrime> Zero for Zp<N> {
    fn zero() -> Self {
        Self(Zn::one())
    }

    fn is_zero(&self) -> bool {
        self.0.is_one()
    }
}

impl<N: BigPrime> Neg for Zp<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.inv())
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
        Self(iter.into_iter().map(|x| x.0).product())
    }
}

impl<N: BigPrime> Mul<BigInt> for Zp<N> {
    type Output = Self;

    fn mul(self, rhs: BigInt) -> Self::Output {
        match rhs.try_into() {
            Ok(rhs) => Self(self.0.pow(rhs)),
            Err(err) => -self * -err.into_original(),
        }
    }
}

impl<N: BigPrime + Generator> Encoding for Zp<N> {
    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Self> {
        stream.next().map(|x| (N::generator() * BigUint::from(x)).into())
    }
}

impl<N: BigPrime + Generator> Decoding for Zp<N> {
    type Error = TooBig;

    fn decode(self) -> Result<Vec<u8>, Self::Error> {
        for x in u8::MIN..=u8::MAX {
            if Self::from(N::generator() * BigUint::from(x)) == self {
                return Ok(vec![x]);
            }
        }
        Err(TooBig(self.0.into()))
    }
}

impl<N: BigPrime> Serialize for Zp<N> {
    fn serialize(self) -> Vec<u8> {
        self.0.serialize()
    }
}

impl<N: BigPrime> Deserialize for Zp<N> {
    type Error = <Zn<N> as Deserialize>::Error;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        Zn::<N>::deserialize(stream).map(|x| x.map(Self))
    }
}

#[derive(Debug, Error)]
#[error("{0} is too big for decoding")]
pub struct TooBig(BigUint);
