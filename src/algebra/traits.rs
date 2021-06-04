use std::{
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_bigint::BigUint;
use num_traits::{Inv, One, Pow, Zero};

pub trait Value: Clone + Eq {}

pub trait BasicGroup:
    Value + Zero + Add<Output = Self> + Neg<Output = Self>
{
}

pub trait Group:
    BasicGroup + Sub<Output = Self> + Mul<isize, Output = Self> + Sum
{
}

pub trait FinGroup: Group {
    fn order() -> BigUint;
}

pub trait Ring:
    Group + One + Mul<Output = Self> + Pow<usize, Output = Self> + Product
{
}

pub trait Integral: Ring + Div<Output = Self> + Rem<Output = Self> {}

pub trait Field: Ring + Inv<Output = Self> + Div<Output = Self> {}

pub trait Sqrt: Sized {
    fn sqrt(self) -> Option<Self>;
}

impl<T> Value for T where T: Clone + Eq {}

impl<T> BasicGroup for T where
    T: Value + Zero + Add<Output = Self> + Neg<Output = Self>
{
}

impl<T> Integral for T where T: Ring + Div<Output = Self> + Rem<Output = Self> {}

impl Group for isize {}

impl Ring for isize {}
