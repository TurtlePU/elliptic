use std::{
    iter::Sum,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_traits::{Inv, One, Pow, Zero};

use crate::{algo::extended_gcd, poly::Poly, traits::*};

pub trait Irreducible<T> {
    fn modulo() -> Poly<T>;
}

pub trait IntoField<T, I> {
    fn into_field(poly: Poly<T>) -> PolyField<T, I>;
}

impl<T, I> IntoField<T, I> for I
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    fn into_field(poly: Poly<T>) -> PolyField<T, I> {
        PolyField(poly % Self::modulo(), PhantomData)
    }
}

#[derive(Debug)]
pub struct PolyField<T, I>(Poly<T>, PhantomData<I>);

impl<T, I> Group for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
}

impl<T, I> Ring for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
}

impl<T, I> Field for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
}

impl<T, I> Clone for PolyField<T, I>
where
    Poly<T>: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<T, I> PartialEq for PolyField<T, I>
where
    Poly<T>: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T, I> Eq for PolyField<T, I> where Self: PartialEq {}

impl<T, I> Add for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        I::into_field(self.0 + rhs.0)
    }
}

impl<T, I> Neg for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        I::into_field(-self.0)
    }
}

impl<T, I> Sub for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        I::into_field(self.0 - rhs.0)
    }
}

impl<T, I> Mul<isize> for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        I::into_field(self.0 * rhs)
    }
}

impl<T, I> Sum for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    fn sum<J: Iterator<Item = Self>>(iter: J) -> Self {
        I::into_field(iter.map(|x| x.0).sum())
    }
}

impl<T, I> Mul for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        I::into_field(self.0 * rhs.0)
    }
}

impl<T, I> Pow<u32> for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn pow(self, rhs: u32) -> Self::Output {
        I::into_field(self.0.pow(rhs))
    }
}

impl<T, I> Zero for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    fn zero() -> Self {
        Self(Poly::zero(), PhantomData)
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T, I> One for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    fn one() -> Self {
        I::into_field(Poly::one())
    }
}

impl<T, I> Inv for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn inv(self) -> Self::Output {
        let (gcd, x, _) = extended_gcd(self.0, I::modulo());
        assert_eq!(gcd.degree(), 0);
        I::into_field(x / gcd)
    }
}

impl<T, I> Div for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}
