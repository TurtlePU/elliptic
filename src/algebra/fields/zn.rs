use std::{
    convert::TryInto,
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_bigint::BigUint;
use num_traits::{Inv, One, Pow, Zero};
use rand::{
    distributions::{
        uniform::{SampleUniform, UniformInt, UniformSampler},
        Standard,
    },
    prelude::Distribution,
};

use crate::algebra::{
    algo::extended_gcd,
    traits::{Field, FinGroup, Group, Ring},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zn<const N: usize>(usize);

impl<const N: usize> Group for Zn<N> {}

impl<const N: usize> FinGroup for Zn<N> {
    fn order() -> BigUint {
        BigUint::from(N)
    }
}

impl<const N: usize> Ring for Zn<N> {}

impl<const N: usize> Field for Zn<N> {}

impl<const N: usize> From<usize> for Zn<N> {
    fn from(n: usize) -> Self {
        Self(n % N)
    }
}

impl<'a, const N: usize> From<&'a Zn<N>> for usize {
    fn from(zn: &'a Zn<N>) -> Self {
        zn.0
    }
}

impl<const N: usize> Distribution<Zn<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Zn<N> {
        Zn::from(rng.gen::<usize>())
    }
}

impl<const N: usize> Add for Zn<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl<const N: usize> Neg for Zn<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from(N - self.0)
    }
}

impl<const N: usize> Sub for Zn<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const N: usize> Mul<isize> for Zn<N> {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        if rhs < 0 {
            -self * -rhs
        } else {
            Self::from(self.0 * rhs as usize)
        }
    }
}

impl<const N: usize> Sum for Zn<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::from(iter.map(|x| x.0).sum::<usize>())
    }
}

impl<const N: usize> Mul for Zn<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.0 * rhs.0)
    }
}

impl<const N: usize> Pow<u32> for Zn<N> {
    type Output = Self;

    fn pow(self, rhs: u32) -> Self::Output {
        Self::from(self.0.pow(rhs))
    }
}

impl<const N: usize> Inv for Zn<N> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        let n: isize = N.try_into().unwrap();
        let (gcd, mut inv, _) = extended_gcd(self.0.try_into().unwrap(), n);
        assert!(gcd.is_one());
        while inv < 0 {
            inv += n;
        }
        Self(inv.try_into().unwrap())
    }
}

impl<const N: usize> Div for Zn<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const N: usize> Zero for Zn<N> {
    fn zero() -> Self {
        Self::from(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl<const N: usize> One for Zn<N> {
    fn one() -> Self {
        Self::from(1)
    }
}

pub struct UniformZn<const N: usize>(UniformInt<usize>);

impl<const N: usize> UniformSampler for UniformZn<N> {
    type X = Zn<N>;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let (low, high): (usize, usize) =
            (low.borrow().into(), high.borrow().into());
        Self(UniformInt::new(low, high))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
        B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
    {
        let (low, high): (usize, usize) =
            (low.borrow().into(), high.borrow().into());
        Self(UniformInt::new_inclusive(low, high))
    }

    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Zn::from(self.0.sample(rng))
    }
}

impl<const N: usize> SampleUniform for Zn<N> {
    type Sampler = UniformZn<N>;
}
