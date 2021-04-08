use std::{iter::Sum, ops::{Add, Div, Mul, Neg, Rem, Sub}};

use itertools::{Itertools, EitherOrBoth::{self, *}};
use num_traits::{One, Zero};

use crate::traits::{Field, Integral, Ring};

use self::monome::Monome;

mod monome;

#[derive(Clone)]
pub struct Poly<T>(Vec<T>);

impl<T> Ring for Poly<T> where T: Field + Sum {}

impl<T> Integral for Poly<T> where T: Field + Sum {}

impl<T> Poly<T> {
    pub fn degree(&self) -> usize {
        self.0.len().max(1) - 1
    }

    pub fn eldest_monome(&self) -> Option<Monome<T>> where T: Clone {
        self.0.last().map(|coeff| Monome {
            coeff: coeff.clone(),
            degree: self.degree(),
        })
    }

    pub fn apply_binop<U, V, F>(self, rhs: Poly<U>, op: F) -> Poly<V>
    where F: Fn(EitherOrBoth<T, U>) -> V,
          V: Zero
    {
        let mut reversed_trimmed =
            self.0.into_iter()
                .zip_longest(rhs.0.into_iter())
                .map(op)
                .rev()
                .skip_while(V::is_zero)
                .collect::<Vec<_>>();
        reversed_trimmed.reverse();
        Poly(reversed_trimmed)
    }
}

impl<T> Default for Poly<T> where T: Zero {
    fn default() -> Self {
        Self::zero()
    }
}

impl<T> From<T> for Poly<T> where T: Zero {
    fn from(coeff: T) -> Self {
        if coeff.is_zero() {
            Self::zero()
        } else {
            Self(vec![coeff])
        }
    }
}

impl<T> From<Vec<Monome<T>>> for Poly<T> where T: Zero {
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

impl<T, U, V> Add<Poly<U>> for Poly<T>
where T: Add<U, Output = V> + Zero, U: Zero, V: Zero {
    type Output = Poly<V>;

    fn add(self, rhs: Poly<U>) -> Self::Output {
        self.apply_binop(rhs, |x| match x {
            Both(x, y) => x + y,
            Left(x) => x + U::zero(),
            Right(y) => T::zero() + y,
        })
    }
}

impl<T, U, V> Sub<Poly<U>> for Poly<T>
where T: Sub<U, Output = V> + Zero, U: Zero, V: Zero {
    type Output = Poly<V>;

    fn sub(self, rhs: Poly<U>) -> Self::Output {
        self.apply_binop(rhs, |x| match x {
            Both(x, y) => x - y,
            Left(x) => x - U::zero(),
            Right(y) => T::zero() - y,
        })
    }
}

impl<T> Neg for Poly<T> where T: Neg {
    type Output = Poly<T::Output>;

    fn neg(self) -> Self::Output {
        Poly(self.0.into_iter().map(T::neg).collect())
    }
}

impl<T, U, V> Mul<Poly<U>> for Poly<T>
where T: Mul<U, Output = V> + Clone, U: Clone, V: Sum {
    type Output = Poly<V>;

    fn mul(self, rhs: Poly<U>) -> Self::Output {
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

impl<T> Poly<T> where T: Field {
    fn div_rem(mut self, rhs: Self) -> (Self, Self) {
        let mut ans_monomes = Vec::new();
        while self.degree() >= rhs.degree() {
            let monome = self.eldest_monome().unwrap() /
                         rhs.eldest_monome().unwrap();
            ans_monomes.push(monome.clone());
            self = self - monome * rhs.clone();
        }
        (self, Self::from(ans_monomes))
    }
}

impl<T> Div for Poly<T> where T: Field {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl<T> Rem for Poly<T> where T: Field {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl<T> Zero for Poly<T> where T: Zero {
    fn zero() -> Self {
        Self(Vec::default())
    }

    fn is_zero(&self) -> bool {
        self.0.len() == 0
    }
}

impl<T> One for Poly<T> where T: One, Self: Mul<Output = Self> {
    fn one() -> Self {
        Self(vec![T::one()])
    }
}
