use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::{One, Zero};

use crate::algo::inverse::modular_inverse;

#[derive(Clone, Copy)]
pub struct Zn<const N: usize>(usize);

impl<const N: usize> From<usize> for Zn<N> {
    fn from(n: usize) -> Self {
        Self(n % N)
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
        Self(N - self.0)
    }
}

impl<const N: usize> Sub for Zn<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const N: usize> Mul for Zn<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.0 * rhs.0)
    }
}

impl<const N: usize> Zn<N> {
    pub fn inv(self) -> Self {
        Self(modular_inverse(self.0, N).unwrap())
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
