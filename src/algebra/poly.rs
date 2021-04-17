use std::{
    iter::{repeat_with, Sum},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use itertools::{
    EitherOrBoth::{self, *},
    Itertools,
};
use num_traits::{One, Pow, Zero};

use super::{
    algo::repeat_monoid,
    traits::{Field, Group, Ring},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly<T>(Vec<T>);

impl<T> Group for Poly<T> where T: Group {}

impl<T> Ring for Poly<T> where T: Field {}

#[macro_export]
macro_rules! poly {
    ($($e:expr),*) => {
        Poly::from(vec![$(Zn::from($e)),*])
    };
}

impl<T> Poly<T> {
    pub fn degree(&self) -> usize {
        self.0.len().max(1) - 1
    }

    pub fn eldest_monome(&self) -> Option<Monome<T>>
    where
        T: Clone,
    {
        self.0.last().map(|coeff| Monome {
            coeff: coeff.clone(),
            degree: self.degree(),
        })
    }

    pub fn apply_binop<F>(self, rhs: Self, op: F) -> Self
    where
        T: Zero,
        F: Fn(EitherOrBoth<T, T>) -> T,
    {
        let mut reversed_trimmed = self
            .0
            .into_iter()
            .zip_longest(rhs.0.into_iter())
            .map(op)
            .rev()
            .skip_while(T::is_zero)
            .collect::<Vec<_>>();
        reversed_trimmed.reverse();
        Poly(reversed_trimmed)
    }

    fn div_rem(mut self, rhs: Self) -> (Self, Self)
    where
        T: Field,
    {
        let mut ans_monomes = Vec::new();
        while self.degree() >= rhs.degree() {
            let monome =
                self.eldest_monome().unwrap() / rhs.eldest_monome().unwrap();
            ans_monomes.push(monome.clone());
            self = self - monome * rhs.clone();
        }
        (self, Self::from(ans_monomes))
    }
}

impl<T: Group> Default for Poly<T> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T: Group> From<T> for Poly<T> {
    fn from(coeff: T) -> Self {
        if coeff.is_zero() {
            Self::zero()
        } else {
            Self(vec![coeff])
        }
    }
}

impl<T: Zero> From<Vec<T>> for Poly<T> {
    fn from(mut coeffs: Vec<T>) -> Self {
        let mut n = coeffs.len();
        while n > 0 {
            if !coeffs[n - 1].is_zero() {
                break;
            }
            n -= 1;
        }
        coeffs.truncate(n);
        Self(coeffs)
    }
}

impl<T: Zero> From<Vec<Monome<T>>> for Poly<T> {
    fn from(monomes: Vec<Monome<T>>) -> Self {
        let mut container = Vec::with_capacity(monomes.len());
        for Monome { coeff, degree } in monomes {
            if container.len() <= degree {
                container.resize_with(degree + 1, T::zero);
            }
            container[degree] = coeff;
        }
        Self(container)
    }
}

impl<T: Group> Add for Poly<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply_binop(rhs, |x| match x {
            Both(x, y) => x + y,
            Left(x) | Right(x) => x,
        })
    }
}

impl<T: Group> Sub for Poly<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply_binop(rhs, |x| match x {
            Both(x, y) => x - y,
            Left(x) => x,
            Right(y) => T::zero() - y,
        })
    }
}

impl<T: Neg> Neg for Poly<T> {
    type Output = Poly<T::Output>;

    fn neg(self) -> Self::Output {
        Poly(self.0.into_iter().map(T::neg).collect())
    }
}

impl<T: Group> Mul<isize> for Poly<T> {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0.into_iter().map(|x| x * rhs).collect())
    }
}

impl<T: Group> Sum for Poly<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), Self::add)
    }
}

impl<T: Field> Mul for Poly<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let ans_deg = self.degree() + rhs.degree();
        let mut addends_by_deg: Vec<_> = (0..=ans_deg)
            .map(|monome_deg| monome_deg.min(ans_deg - monome_deg) + 1)
            .map(|addends_count| Vec::with_capacity(addends_count))
            .collect();

        for (i, x) in self.0.iter().enumerate() {
            for (j, y) in rhs.0.iter().cloned().enumerate() {
                addends_by_deg[i + j].push(x.clone() * y);
            }
        }

        let result = addends_by_deg
            .into_iter()
            .map(|addends| addends.into_iter().sum())
            .collect();
        Poly(result)
    }
}

impl<T: Field> Pow<u32> for Poly<T> {
    type Output = Self;

    fn pow(self, rhs: u32) -> Self::Output {
        repeat_monoid(Self::mul, rhs as usize, self, Self::one())
    }
}

impl<T: Field> Div for Poly<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl<T: Field> Rem for Poly<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl<T: Group> Zero for Poly<T> {
    fn zero() -> Self {
        Self(Vec::default())
    }

    fn is_zero(&self) -> bool {
        self.0.len() == 0
    }
}

impl<T: Field> One for Poly<T> {
    fn one() -> Self {
        Self(vec![T::one()])
    }
}

#[derive(Clone)]
pub struct Monome<T> {
    coeff: T,
    degree: usize,
}

impl<T: Field> Div for Monome<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(self.degree >= rhs.degree);
        Monome {
            coeff: self.coeff / rhs.coeff,
            degree: self.degree - rhs.degree,
        }
    }
}

impl<T: Field> Mul<Poly<T>> for Monome<T> {
    type Output = Poly<T>;

    fn mul(self, rhs: Poly<T>) -> Self::Output {
        let head = repeat_with(T::zero).take(self.degree);
        let tail = rhs.0.into_iter().map(|x| self.coeff.clone() * x);
        Poly(head.chain(tail).collect())
    }
}
