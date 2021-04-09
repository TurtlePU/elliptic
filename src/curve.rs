use std::{marker::PhantomData, ops::{Add, Neg, Sub}};

use num_traits::Zero;

use crate::traits::{Field, Group};

pub trait Curve<F> {
    fn a() -> F;
    fn b() -> F;
}

fn check_solution<F, C>(x: F, y: F) -> bool where F: Field, C: Curve<F> {
    let left = y.clone() * y;
    let x_cube = x.clone() * x.clone() * x.clone();
    let right = x_cube + C::a() * x + C::b();
    left == right
}

#[derive(Debug)]
pub struct NotOnCurve;

pub trait Points<F, C> {
    fn projected(x: F, y: F) -> Result<EllipticPoint<F, C>, NotOnCurve>;

    fn spatial(x: F, y: F, z: F) -> Result<EllipticPoint<F, C>, NotOnCurve>;
}

impl<F, C> Points<F, C> for C where F: Field, C: Curve<F> {
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
        Self { coords, curve: PhantomData }
    }
}

impl<F, C> Group for EllipticPoint<F, C> where F: Field, C: Curve<F> {}

impl<F, C> Clone for EllipticPoint<F, C> where F: Clone {
    fn clone(&self) -> Self {
        Self::from_coords(self.coords.clone())
    }
}

impl<F, C> PartialEq for EllipticPoint<F, C> where F: Field, C: Curve<F> {
    fn eq(&self, other: &Self) -> bool {
        self.coords == other.coords
    }
}

impl<F, C> Eq for EllipticPoint<F, C> where Self: PartialEq {}

impl<F, C> Add for EllipticPoint<F, C> where F: Field, C: Curve<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.coords, rhs.coords) {
            (Some((xp, yp)), Some((xq, yq))) => if xp != xq {
                let s = (yp.clone() - yq) / (xp.clone() - xq.clone());
                let xr = s.clone() * s.clone() - xp.clone() - xq;
                let yr = -yp + s * (xp - xr.clone());
                Self::new(xr, yr)
            } else if yp == yq && !yp.is_zero() {
                let xp_sq = xp.clone() * xp.clone();
                let s = (xp_sq.clone() + xp_sq.clone() + xp_sq + C::a())
                      / (yp.clone() + yp.clone());
                let xr = s.clone() * s.clone() - xp.clone() - xp.clone();
                let yr = -yp + s * (xp - xr.clone());
                Self::new(xr, yr)
            } else {
                Self::zero()
            },
            (p, None) | (None, p) => Self::from_coords(p),
        }
    }
}

impl<F, C> Sub for EllipticPoint<F, C> where F: Field, C: Curve<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<F, C> Neg for EllipticPoint<F, C> where F: Field, C: Curve<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.coords {
            Some((x, y)) => Self::new(x, -y),
            None => self,
        }
    }
}

impl<F, C> Zero for EllipticPoint<F, C> where F: Field, C: Curve<F> {
    fn zero() -> Self {
        Self::from_coords(None)
    }

    fn is_zero(&self) -> bool {
        self.coords.is_none()
    }
}
