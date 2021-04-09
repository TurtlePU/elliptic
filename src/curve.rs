use std::{
    fmt::Debug,
    iter::Sum,
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use num_traits::Zero;

use crate::{
    algo::repeat_monoid,
    traits::{Field, Group},
};

pub trait Curve<F> {
    fn a() -> F;
    fn b() -> F;
}

pub fn check_char<F: Debug + Field>() {
    assert_ne!(F::one() * 2, F::zero());
    assert_ne!(F::one() * 3, F::zero());
}

fn check_solution<F: Field, C: Curve<F>>(x: F, y: F) -> bool {
    y.pow(2) == x.clone().pow(3) + C::a() * x + C::b()
}

#[derive(Debug)]
pub struct NotOnCurve;

pub trait Points<F, C> {
    fn projected(x: F, y: F) -> Result<EllipticPoint<F, C>, NotOnCurve>;

    fn spatial(x: F, y: F, z: F) -> Result<EllipticPoint<F, C>, NotOnCurve>;
}

impl<F: Field, C: Curve<F>> Points<F, C> for C {
    fn projected(x: F, y: F) -> Result<EllipticPoint<F, C>, NotOnCurve> {
        if check_solution::<F, C>(x.clone(), y.clone()) {
            Ok(EllipticPoint::new(x, y))
        } else {
            Err(NotOnCurve)
        }
    }

    fn spatial(x: F, y: F, z: F) -> Result<EllipticPoint<F, C>, NotOnCurve> {
        let i = z.inv();
        let x = x * i.clone();
        let y = y * i;
        Self::projected(x, y)
    }
}

pub struct EllipticPoint<F, C> {
    coords: Option<(F, F)>,
    curve: PhantomData<C>,
}

impl<F, C> EllipticPoint<F, C> {
    fn new(x: F, y: F) -> Self {
        Self::from_coords(Some((x, y)))
    }

    fn from_coords(coords: Option<(F, F)>) -> Self {
        Self {
            coords,
            curve: PhantomData,
        }
    }
}

impl<F: Field, C: Curve<F>> Group for EllipticPoint<F, C> {}

impl<F: Clone, C> Clone for EllipticPoint<F, C> {
    fn clone(&self) -> Self {
        Self::from_coords(self.coords.clone())
    }
}

impl<F: PartialEq, C> PartialEq for EllipticPoint<F, C> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl<F: Eq, C> Eq for EllipticPoint<F, C> {}

impl<F: Field, C: Curve<F>> Add for EllipticPoint<F, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let c = |x: &F| x.clone();
        match (self.coords, rhs.coords) {
            (Some((xp, yp)), Some((xq, yq))) => {
                if xp == xq && yp == -c(&yq) {
                    return Self::zero();
                }
                let s = if xp != xq {
                    (c(&yp) - yq) / (c(&xp) - c(&xq))
                } else {
                    (c(&xp).pow(2) * 3 + C::a()) / (c(&yp) + yq)
                };
                let xr = c(&s).pow(2) - c(&xp) - xq;
                let yr = -yp + s * (xp - c(&xr));
                Self::new(xr, yr)
            }
            (p, None) | (None, p) => Self::from_coords(p),
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

    fn neg(self) -> Self::Output {
        match self.coords {
            Some((x, y)) => Self::new(x, -y),
            None => self,
        }
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
        Self::from_coords(None)
    }

    fn is_zero(&self) -> bool {
        self.coords.is_none()
    }
}
