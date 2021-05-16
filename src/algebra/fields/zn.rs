use std::{
    convert::TryInto,
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_bigint::BigUint;
use num_traits::{Inv, One, Pow, Zero};
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler},
        Standard,
    },
    prelude::Distribution,
    Rng,
};

use crate::algebra::{
    algo::{extended_gcd, is_prime, repeat_monoid},
    traits::{Field, FinGroup, Group, Ring, Sqrt},
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

impl<const N: usize> From<Zn<N>> for usize {
    fn from(zn: Zn<N>) -> Self {
        zn.0
    }
}

impl<const N: usize> Distribution<Zn<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Zn<N> {
        Zn::from(rng.gen::<usize>())
    }
}

impl<const N: usize> Sqrt for Zn<N> {
    fn sqrt(self) -> Option<Self> {
        assert!(is_prime(Self::order()));
        assert!((N + 1) % 4 == 0);
        let sqrt = self.clone().pow((N + 1) / 4);
        if sqrt.clone().pow(2) == self {
            Some(sqrt)
        } else {
            None
        }
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

impl<const N: usize> Pow<usize> for Zn<N> {
    type Output = Self;

    fn pow(self, rhs: usize) -> Self::Output {
        Self::from(repeat_monoid(usize::mul, rhs, self.0, 1))
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
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self(UniformInt::new(
            usize::from(*low.borrow()),
            usize::from(*high.borrow()),
        ))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self(UniformInt::new_inclusive(
            usize::from(*low.borrow()),
            usize::from(*high.borrow()),
        ))
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Zn::from(self.0.sample(rng))
    }
}

impl<const N: usize> SampleUniform for Zn<N> {
    type Sampler = UniformZn<N>;
}
