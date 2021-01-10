use super::{group::Group, ring::Ring};
use std::ops::{Add, Sub, Mul, Rem, Div};
use num_traits::identities::{Zero, One};

pub struct ZN<T>(pub T);

impl<T> ZN<T> where T: Clone + Rem<Output = T> {
    fn rem_into(&self, x: T) -> T {
        x % self.0.clone()
    }

    fn rem(&self, x: &T) -> T {
        self.rem_into(x.clone())
    }
}

impl<T> Group<T> for ZN<T> where T: Zero + Clone + Eq
    + Add<Output = T> + Sub<Output = T> + Rem<Output = T> {

    fn zero(&self) -> T {
        T::zero()
    }

    fn eq(&self, x: &T, y: &T) -> bool {
        self.rem(x) == self.rem(y)
    }

    fn add(&self, x: T, y: T) -> T {
        self.rem_into(x) + self.rem_into(y)
    }

    fn neg(&self, x: T) -> T {
        self.0.clone() - self.rem_into(x)
    }
}

impl<T> Ring<T> for ZN<T> where T: Zero + One + Clone + Eq + Add<Output = T>
    + Sub<Output = T> + Rem<Output = T> + Mul<Output = T> + Div<Output = T> {

    fn one(&self) -> T {
        T::one()
    }

    fn mul(&self, x: T, y: T) -> T {
        self.rem_into(x) * self.rem_into(y)
    }

    fn inv(&self, x: T) -> Option<T> {
        let (mut r, mut t) = (self.0.clone(), T::zero());
        let (ref mut r1, ref mut t1) = (x, T::one());
        while r1.clone() != T::zero() {
            let q = r.clone() / r1.clone();
            t = replace(t - q.clone() * t1.clone(), t1);
            r = replace(r - q * r1.clone(), r1);
        }
        if r == T::one() {
            Some(t)
        } else {
            None
        }
    }
}

fn replace<T>(new: T, old: &mut T) -> T {
    std::mem::replace(old, new)
}
