use std::{iter::repeat_with, ops::{Div, Mul}};

use num_traits::Zero;

use super::Poly;

#[derive(Clone)]
pub struct Monome<T> {
    pub coeff: T,
    pub degree: usize,
}

impl<T> Div for Monome<T> where T: Div<Output = T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(self.degree >= rhs.degree);
        Monome {
            coeff: self.coeff / rhs.coeff,
            degree: self.degree - rhs.degree,
        }
    }
}

impl<T> Mul<Poly<T>> for Monome<T> where T: Mul<Output = T> + Zero + Clone {
    type Output = Poly<T>;

    fn mul(self, rhs: Poly<T>) -> Self::Output {
        let head = repeat_with(T::zero).take(self.degree);
        let tail = rhs.0.into_iter().map(|x| self.coeff.clone() * x);
        Poly(head.chain(tail).collect())
    }
}
