use std::{
    iter::Sum,
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_bigint::BigUint;
use num_traits::{Inv, One, Pow, Zero};
use rand::{distributions::Standard, prelude::Distribution, Fill};

use crate::algebra::{algo::extended_gcd, poly::Poly, traits::*};

pub trait DenseBytes {}

pub trait Irreducible<T>: Sized
where
    Poly<T>: Integral,
{
    fn modulo() -> Poly<T>;

    fn into_field(poly: Poly<T>) -> PolyField<T, Self> {
        PolyField(poly % Self::modulo(), PhantomData)
    }
}

#[derive(Debug)]
pub struct PolyField<T, I>(Poly<T>, PhantomData<I>);

impl<T, I> Distribution<PolyField<T, I>> for Standard
where
    I: Irreducible<T>,
    Poly<T>: Integral + Fill,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PolyField<T, I> {
        let mut result = I::modulo();
        result.try_fill(rng).unwrap();
        I::into_field(result)
    }
}

impl<T, I> Group for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
}

impl<T, I> FinGroup for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
    T: FinGroup,
{
    fn order() -> BigUint {
        T::order().pow(I::modulo().degree())
    }
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

impl<T, I> From<Poly<T>> for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    fn from(poly: Poly<T>) -> Self {
        I::into_field(poly)
    }
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

impl<T, I> Pow<usize> for PolyField<T, I>
where
    I: Irreducible<T>,
    Poly<T>: Integral,
{
    type Output = Self;

    fn pow(self, rhs: usize) -> Self::Output {
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

#[cfg(test)]
mod tests {
    use crate::{algebra::{fields::Zn, poly::Poly}, poly};
    use super::{Irreducible, PolyField};

    pub struct XcubePlus2;

    impl Irreducible<Zn<5>> for XcubePlus2 {
        fn modulo() -> Poly<Zn<5>> {
            poly![2, 0, 0, 1]
        }
    }

    type F = PolyField<Zn<5>, XcubePlus2>;
}
