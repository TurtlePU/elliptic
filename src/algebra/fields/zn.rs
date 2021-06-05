use std::{
    array::TryFromSliceError,
    convert::TryFrom,
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_bigint::{BigInt, BigUint};
use num_traits::{Inv, One, Pow, Zero};
use rand::{distributions::Standard, prelude::Distribution};

use crate::{
    algebra::{
        algo::extended_gcd,
        traits::{Field, FinGroup, Group, Ring, Sqrt},
    },
    bytes::{Deserialize, Serialize},
};

pub trait BigPrime {
    fn value() -> BigUint;
    fn bytes() -> usize {
        Self::value().to_bytes_le().len()
    }
}

#[derive(Debug)]
pub struct Zn<N>(BigUint, PhantomData<N>);

impl<N: BigPrime> Group for Zn<N> {}

impl<N: BigPrime> FinGroup for Zn<N> {
    fn order() -> BigUint {
        N::value()
    }
}

impl<N: BigPrime> Ring for Zn<N> {}

impl<N: BigPrime> Field for Zn<N> {}

impl<N: BigPrime> From<BigUint> for Zn<N> {
    fn from(n: BigUint) -> Self {
        Self(n % N::value(), PhantomData)
    }
}

impl<N: BigPrime> From<usize> for Zn<N> {
    fn from(x: usize) -> Self {
        Self::from(BigUint::from(x))
    }
}

impl<N: BigPrime> From<Zn<N>> for BigUint {
    fn from(zn: Zn<N>) -> Self {
        zn.0
    }
}

impl<N: BigPrime> Distribution<Zn<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Zn<N> {
        Zn::from(rng.gen::<usize>())
    }
}

impl<N> Clone for Zn<N> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<N> PartialEq for Zn<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<N> Eq for Zn<N> where BigUint: Eq {}

impl<N: BigPrime> Sqrt for Zn<N> {
    fn sqrt(self) -> Option<Self> {
        let deg = (N::value() + BigUint::one()) >> 2;
        let sqrt = self.clone().pow(deg);
        if sqrt.clone() * sqrt.clone() == self {
            Some(Self::from(sqrt))
        } else {
            None
        }
    }
}

impl<N: BigPrime> Add for Zn<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl<N: BigPrime> Neg for Zn<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from(N::value() - self.0)
    }
}

impl<N: BigPrime> Sub for Zn<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<N: BigPrime> Mul<BigInt> for Zn<N> {
    type Output = Self;

    fn mul(self, rhs: BigInt) -> Self::Output {
        match BigUint::try_from(rhs) {
            Ok(rhs) => Self::from(self.0 * rhs),
            Err(err) => -self * -err.into_original(),
        }
    }
}

impl<N: BigPrime> Sum for Zn<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::from(iter.map(|x| x.0).sum::<BigUint>())
    }
}

impl<N: BigPrime> Mul for Zn<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.0 * rhs.0)
    }
}

impl<N: BigPrime> Pow<BigUint> for Zn<N> {
    type Output = Self;

    fn pow(self, rhs: BigUint) -> Self::Output {
        Self(self.0.modpow(&rhs, &N::value()), PhantomData)
    }
}

impl<N: BigPrime> Product for Zn<N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().fold(Self::one(), Self::mul)
    }
}

impl<N: BigPrime> Inv for Zn<N> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        let n: BigInt = N::value().into();
        let (gcd, mut inv, _) = extended_gcd(self.0.into(), n.clone());
        assert!(gcd.is_one());
        inv = inv % &n;
        if inv < BigInt::zero() {
            inv += n;
        }
        Self::from(BigUint::try_from(inv).unwrap())
    }
}

impl<N: BigPrime> Div for Zn<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<N: BigPrime> Zero for Zn<N> {
    fn zero() -> Self {
        Self::from(0)
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<N: BigPrime> One for Zn<N> {
    fn one() -> Self {
        Self::from(1)
    }
}

impl<N: BigPrime> Serialize for Zn<N> {
    fn serialize(self) -> Vec<u8> {
        let mut result = self.0.to_bytes_le();
        result.resize(N::bytes(), 0);
        result
    }
}

impl<N: BigPrime> Deserialize for Zn<N> {
    type Error = TryFromSliceError;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        let vec: Vec<_> =
            (0..N::bytes()).filter_map(|_| stream.next()).collect();
        if vec.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self::from(BigUint::from_bytes_le(&vec[..]))))
        }
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::{Inv, Pow, Zero};

    use crate::algebra::{
        fields::zn::{BigPrime, Zn},
        traits::Sqrt,
    };

    #[test]
    fn add() {
        assert!((Zn::<Z74>::from(69) + Zn::from(5)).is_zero());
        assert!(Zn::<Z180>::from(174) + Zn::from(389) == Zn::from(23));
        assert!(-Zn::<Z47>::from(111) == Zn::from(30));
    }

    #[test]
    fn mul() {
        assert!(Zn::<Z14>::from(19) * Zn::from(5) == Zn::from(11));
        assert!((Zn::<Z9>::from(81) * Zn::from(12326234)).is_zero());
    }

    #[test]
    fn inv() {
        assert!(Zn::<Z18>::from(5).inv() == Zn::from(11));
        assert!(Zn::<Z17>::from(8).inv() == Zn::from(15));
    }

    #[test]
    fn sqrt() {
        let a = Zn::<Z19>::from(11);
        let sqrt = a.clone().sqrt().unwrap();
        assert!(sqrt.pow(BigUint::from(2usize)) == a);
    }

    pub struct Z74;
    pub struct Z180;
    pub struct Z47;
    pub struct Z14;
    pub struct Z9;
    pub struct Z18;
    pub struct Z17;
    pub struct Z19;

    impl BigPrime for Z74 {
        fn value() -> BigUint {
            BigUint::from(74usize)
        }
    }

    impl BigPrime for Z180 {
        fn value() -> BigUint {
            BigUint::from(180usize)
        }
    }

    impl BigPrime for Z47 {
        fn value() -> BigUint {
            BigUint::from(47usize)
        }
    }

    impl BigPrime for Z14 {
        fn value() -> BigUint {
            BigUint::from(14usize)
        }
    }

    impl BigPrime for Z9 {
        fn value() -> BigUint {
            BigUint::from(9usize)
        }
    }

    impl BigPrime for Z18 {
        fn value() -> BigUint {
            BigUint::from(18usize)
        }
    }

    impl BigPrime for Z17 {
        fn value() -> BigUint {
            BigUint::from(17usize)
        }
    }

    impl BigPrime for Z19 {
        fn value() -> BigUint {
            BigUint::from(19usize)
        }
    }
}
