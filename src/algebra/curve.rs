use std::{
    error::Error,
    fmt::Debug,
    iter::Sum,
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num_bigint::BigUint;
use num_traits::Zero;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use thiserror::Error;

use crate::bytes::{Decoding, Deserialize, Encoding, Serialize};

use super::{
    algo::repeat_monoid,
    traits::{Field, FinGroup, Group, Sqrt},
};

pub trait Curve<F: Field>: Sized {
    fn group_order() -> BigUint;
    fn a() -> F;
    fn b() -> F;

    fn affine(x: F, y: F) -> Result<EllipticPoint<F, Self>, NotOnCurve> {
        if check_solution::<F, Self>(x.clone(), y.clone()) {
            Ok(EllipticPoint::affine(x, y))
        } else {
            Err(NotOnCurve)
        }
    }
}

pub trait Encoder<T> {
    type Error: Error + 'static;

    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<T>;
    fn decode(item: T) -> Result<Vec<u8>, Self::Error>;
}

#[derive(Debug, Error)]
#[error("Point is not on curve.")]
pub struct NotOnCurve;

pub struct EllipticPoint<F, C> {
    coords: (F, F, F),
    curve: PhantomData<C>,
}

impl<F, C> EllipticPoint<F, C> {
    fn new(x: F, y: F, z: F) -> Self {
        Self::from_coords((x, y, z))
    }

    fn from_coords(coords: (F, F, F)) -> Self {
        Self {
            coords,
            curve: PhantomData,
        }
    }
}

impl<F: Field, C> EllipticPoint<F, C> {
    fn affine(x: F, y: F) -> Self {
        Self::new(x, y, F::one())
    }
}

impl<F: Field + Sqrt, C: Curve<F>> Distribution<EllipticPoint<F, C>>
    for Standard
where
    Standard: Distribution<F>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EllipticPoint<F, C> {
        loop {
            let x = rng.gen();
            if let Some(y) = solve::<F, C>(x.clone()) {
                break EllipticPoint::affine(x, y);
            }
        }
    }
}

impl<F: Clone, C> EllipticPoint<F, C> {
    fn x(&self) -> F {
        self.coords.0.clone()
    }

    fn y(&self) -> F {
        self.coords.1.clone()
    }

    fn z(&self) -> F {
        self.coords.2.clone()
    }
}

impl<F: Field, C> EllipticPoint<F, C> {
    fn is_infinite(&self) -> bool {
        self.coords.2.is_zero()
    }
}

impl<F: Field, C> From<EllipticPoint<F, C>> for (F, F) {
    fn from(point: EllipticPoint<F, C>) -> Self {
        let (x, y, z) = point.coords;
        let i = z.inv();
        (x * i.clone(), y * i)
    }
}

impl<F: Field, C> From<EllipticPoint<F, C>> for Option<(F, F)> {
    fn from(point: EllipticPoint<F, C>) -> Self {
        if point.is_infinite() {
            None
        } else {
            Some(point.into())
        }
    }
}

impl<F: Field, C: Curve<F>> Group for EllipticPoint<F, C> {}

impl<F: Field, C: Curve<F>> FinGroup for EllipticPoint<F, C> {
    fn order() -> BigUint {
        C::group_order()
    }
}

impl<F: Clone, C> Clone for EllipticPoint<F, C> {
    fn clone(&self) -> Self {
        Self::from_coords(self.coords.clone())
    }
}

impl<F: Field, C: Curve<F>> PartialEq for EllipticPoint<F, C> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() {
            other.is_zero()
        } else {
            self.x() * other.z() == other.x() * self.z()
                && self.y() * other.z() == other.y() * self.z()
        }
    }
}

impl<F: Field, C: Curve<F>> Eq for EllipticPoint<F, C> {}

impl<F: Field, C: Curve<F>> Add for EllipticPoint<F, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_zero() {
            rhs
        } else if rhs.is_zero() {
            self
        } else if self == -rhs.clone() {
            Self::zero()
        } else if self == rhs {
            let q = self.y() * self.z() * 2;
            let n = self.x().pow(2) * 3 + C::a() * self.z().pow(2);
            let p = self.x() * self.y().pow(2) * self.z() * 4;
            let u = n.clone().pow(2) - p.clone() * 2;

            let x = u.clone() * q.clone();
            let z = q.pow(3);
            let y = n * (p - u) - self.y().pow(4) * self.z().pow(2) * 8;

            Self::new(x, y, z)
        } else {
            let u = rhs.y() * self.z() - self.y() * rhs.z();
            let v = rhs.x() * self.z() - self.x() * rhs.z();
            let w = u.clone().pow(2) * self.z() * rhs.z()
                - v.clone().pow(3)
                - v.clone().pow(2) * 2 * self.x() * rhs.z();
            let q = v.clone().pow(3) * self.y() * rhs.z();

            let x = v.clone() * w.clone();
            let z = self.z() * rhs.z() * v.clone().pow(3);
            let y = u * (v.pow(2) * self.x() * rhs.z() - w) - q;

            Self::new(x, y, z)
        }
    }
}

impl<F: Field, C: Curve<F>> Sub for EllipticPoint<F, C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<F: Field, C: Curve<F>> Neg for EllipticPoint<F, C> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.coords.1 = -self.coords.1;
        self
    }
}

impl<F: Field, C: Curve<F>> Mul<isize> for EllipticPoint<F, C> {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        if rhs < 0 {
            -self * -rhs
        } else {
            repeat_monoid(Self::add, rhs as usize, self, Self::zero())
        }
    }
}

impl<F: Field, C: Curve<F>> Sum for EllipticPoint<F, C>
where
    Self: Zero,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Self::add)
    }
}

impl<F: Field, C: Curve<F>> Zero for EllipticPoint<F, C> {
    fn zero() -> Self {
        Self::new(F::zero(), F::one(), F::zero())
    }

    fn is_zero(&self) -> bool {
        self.is_infinite()
    }
}

pub fn check_char<F: Debug + Field>() {
    assert_ne!(F::one() * 2, F::zero());
    assert_ne!(F::one() * 3, F::zero());
}

pub fn check_curve<F: Debug + Field, C: Curve<F>>() {
    check_char::<F>();
    let prop = C::a().pow(3) * 4 + C::b().pow(2) * 27;
    assert_ne!(prop, F::zero());
}

fn check_solution<F: Field, C: Curve<F>>(x: F, y: F) -> bool {
    y.pow(2) == right_side::<F, C>(x)
}

fn solve<F: Field + Sqrt, C: Curve<F>>(x: F) -> Option<F> {
    right_side::<F, C>(x).sqrt()
}

fn right_side<F: Field, C: Curve<F>>(x: F) -> F {
    x.clone().pow(3) + C::a() * x + C::b()
}

impl<F, C> Encoding for EllipticPoint<F, C>
where
    C: Encoder<EllipticPoint<F, C>>,
{
    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Self> {
        C::encode(stream)
    }
}

impl<F, C> Decoding for EllipticPoint<F, C>
where
    C: Encoder<EllipticPoint<F, C>>,
{
    type Error = C::Error;

    fn decode(self) -> Result<Vec<u8>, Self::Error> {
        C::decode(self)
    }
}

impl<F, C> Serialize for EllipticPoint<F, C>
where
    F: Field + Serialize,
{
    fn serialize(self) -> Vec<u8> {
        match Option::<(F, F)>::from(self) {
            Some(points) => {
                let mut result = vec![1];
                result.append(&mut points.serialize());
                result
            }
            None => vec![0],
        }
    }
}

impl<F, C> Deserialize for EllipticPoint<F, C>
where
    F: Field + Deserialize,
    C: Curve<F>,
{
    type Error = PointDeserError<F::Error>;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        use PointDeserError::NotEnoughBytes;
        match stream.next() {
            Some(0) => Ok(Some(EllipticPoint::zero())),
            Some(_) => {
                let x = F::deserialize(stream)?.ok_or(NotEnoughBytes)?;
                let y = F::deserialize(stream)?.ok_or(NotEnoughBytes)?;
                C::affine(x, y)
                    .map(Some)
                    .map_err(PointDeserError::NotOnCurve)
            }
            None => Ok(None),
        }
    }
}

#[derive(Debug, Error)]
pub enum PointDeserError<E: Debug + Error + 'static> {
    #[error(transparent)]
    FieldDeser(#[from] E),
    #[error("Not enough bytes")]
    NotEnoughBytes,
    #[error(transparent)]
    NotOnCurve(NotOnCurve),
}
