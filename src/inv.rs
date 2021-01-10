use std::ops::{Sub, Mul, Div, Rem};
use num_traits::identities::{Zero, One};

pub fn inv<T>(x: T, n: T) -> Option<T>
    where T: Zero + One + Clone + Eq
    + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>
{
    let (ref mut r, ref mut r1, ref mut t, ref mut t1)
        = (n, x, T::zero(), T::one());
    while r1.clone() != T::zero() {
        let q = r.clone() / r1.clone();
        shift(t, t1, t.clone() - q * t1.clone());
        shift(r, r1, r.clone() - q * r1.clone());
    }
    if *r == T::one() {
        Some(*t)
    } else {
        None
    }
}

fn shift<T>(prev: &mut T, cur: &mut T, new: T) {
    *prev = *cur;
    *cur = new;
}
