use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Inv, One, Zero};

pub trait Ring : Zero + One + Add<Output = Self> + Neg<Output = Self>
               + Sub<Output = Self> + Mul<Output = Self> + Clone
{}

pub trait Field : Ring + Inv<Output = Self> + Div<Output = Self> {}

pub trait Integral : Ring + Div<Output = Self> + Rem<Output = Self> {}

impl Ring for isize {}

impl Integral for isize {}
