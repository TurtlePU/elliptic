use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Inv, One, Pow, Zero};

pub trait Group : Clone + Eq + Zero + Add<Output = Self>
                + Neg<Output = Self> + Sub<Output = Self>
                + Mul<isize, Output = Self>
{}

pub trait Ring : Group + One + Mul<Output = Self> + Pow<u32, Output = Self> {}

pub trait Field : Ring + Inv<Output = Self> + Div<Output = Self> {}

pub trait Integral : Ring + Div<Output = Self> + Rem<Output = Self> {}

impl Group for isize {}

impl Ring for isize {}

impl Integral for isize {}
