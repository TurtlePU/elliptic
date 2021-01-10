use super::poly::Poly;
use itertools::{Itertools, EitherOrBoth::*};

pub trait Group<T> {
    fn zero(&self) -> T;
    fn eq(&self, a: &T, b: &T) -> bool;
    fn add(&self, a: T, b: T) -> T;
    fn neg(&self, x: T) -> T;
}

impl<T, G> Group<Poly<T>> for G where G: Group<T> {
    fn zero(&self) -> Poly<T> {
        Poly::default()
    }

    fn eq(&self, a: &Poly<T>, b: &Poly<T>) -> bool {
        a.iter().zip_longest(b).all(|x| match x {
            Both(a, b) => self.eq(a, b),
            _ => false,
        })
    }

    fn add(&self, a: Poly<T>, b: Poly<T>) -> Poly<T> {
        let iter = a.into_iter().zip_longest(b).map(|x| match x {
            Both(a, b) => self.add(a, b),
            Left(a) => a,
            Right(b) => b,
        });
        let ref zero = self.zero();
        Poly::trimmed_with(iter, |x| self.eq(x, zero))
    }

    fn neg(&self, x: Poly<T>) -> Poly<T> {
        x.into_iter().map(|x| self.neg(x)).collect()
    }
}
