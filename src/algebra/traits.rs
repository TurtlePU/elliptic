use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_traits::{Inv, One, Pow, Zero};

pub trait Value: Clone + Eq {}

pub trait BasicGroup: Value + Zero + Add<Output = Self> + Neg<Output = Self> {}

pub trait Group: BasicGroup + Sub<Output = Self> + Mul<isize, Output = Self> + Sum {}

pub trait FinGroup: Group {
    const ORDER: usize;
}

pub trait Ring: Group + One + Mul<Output = Self> + Pow<u32, Output = Self> {}

pub trait Integral: Ring + Div<Output = Self> + Rem<Output = Self> {}

pub trait Field: Ring + Inv<Output = Self> + Div<Output = Self> {}

impl<T> Value for T where T: Clone + Eq {}

impl<T> BasicGroup for T where T: Value + Zero + Add<Output = Self> + Neg<Output = Self> {}

impl<T> Integral for T where T: Ring + Div<Output = Self> + Rem<Output = Self> {}

impl Group for isize {}

impl Ring for isize {}
