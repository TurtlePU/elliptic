use std::{iter::repeat_with, ops::{Div, Mul}};

use num_traits::Zero;

use super::Poly;

#[derive(Clone)]
pub struct Monome<T> {
    pub coeff: T,
    pub degree: usize,
}

impl<T, U, V> Div<Monome<U>> for Monome<T> where T: Div<U, Output = V> {
    type Output = Monome<V>;

    fn div(self, rhs: Monome<U>) -> Self::Output {
        assert!(self.degree >= rhs.degree);
        Monome {
            coeff: self.coeff / rhs.coeff,
            degree: self.degree - rhs.degree,
        }
    }
}

impl<T, U, V> Mul<Poly<U>> for Monome<T>
where T: Mul<U, Output = V> + Clone, V: Zero {
    type Output = Poly<V>;

    fn mul(self, rhs: Poly<U>) -> Self::Output {
        let head = repeat_with(V::zero).take(self.degree);
        let tail = rhs.0.into_iter().map(|x| self.coeff.clone() * x);
        Poly(head.chain(tail).collect())
    }
}
