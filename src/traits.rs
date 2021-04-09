use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::{Inv, One, Zero};

pub trait Group : Clone + Eq + Zero + Add<Output = Self>
                + Neg<Output = Self> + Sub<Output = Self> {}

pub trait Ring : Group + One + Mul<Output = Self> {}

pub trait Field : Ring + Inv<Output = Self> + Div<Output = Self> {}

pub trait Integral : Ring + Div<Output = Self> + Rem<Output = Self> {}

impl Group for isize {}

impl Ring for isize {}

impl Integral for isize {}
