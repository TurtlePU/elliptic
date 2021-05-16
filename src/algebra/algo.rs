use num_bigint::BigUint;
use num_traits::{FromPrimitive, One, Zero};

use super::traits::Integral;

/// Returns (g, x, y) such that a * x + b * y = g = gcd(a, b).
pub fn extended_gcd<T: Integral>(a: T, b: T) -> (T, T, T) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (T::one(), T::zero());
    let (mut old_t, mut t) = (T::zero(), T::one());

    while !r.is_zero() {
        let quot = old_r.clone() / r.clone();
        old_r = replace(old_r - quot.clone() * r.clone(), &mut r);
        old_s = replace(old_s - quot.clone() * s.clone(), &mut s);
        old_t = replace(old_t - quot.clone() * t.clone(), &mut t);
    }

    (old_r, old_s, old_t)
}

fn replace<T>(src: T, dest: &mut T) -> T {
    std::mem::replace(dest, src)
}

/// `app(result, value x cnt)`
pub fn repeat_monoid<T, F>(app: F, cnt: usize, value: T, result: T) -> T
where
    T: Clone,
    F: Clone + Fn(T, T) -> T,
{
    if cnt == 0 {
        result
    } else if cnt % 2 == 0 {
        repeat_monoid(app.clone(), cnt / 2, app(value.clone(), value), result)
    } else {
        repeat_monoid(app.clone(), cnt - 1, value.clone(), app(result, value))
    }
}

pub fn is_prime(value: BigUint) -> bool {
    if value.is_one() {
        return false;
    }
    let mut x = BigUint::from_i8(2).unwrap();
    loop {
        if &x * &x > value {
            break true;
        }
        if (&value % &x).is_zero() {
            return false;
        }
        x += BigUint::one();
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    use super::replace;
    use std::ops::Add;

    #[test]
    fn repeat_monoid() {
        for n in 0..100000 {
            assert_eq!(super::repeat_monoid(usize::add, n, 1, 0), n);
        }
    }

    fn gcd(mut a: isize, mut b: isize) -> isize {
        while b != 0 {
            a = replace(a % b, &mut b);
        }
        a
    }

    #[test]
    fn extended_gcd() {
        for a in 1..800 {
            for b in 1..=a {
                let (g, x, y) = super::extended_gcd(a, b);
                assert_eq!(g, a * x + b * y);
                assert_eq!(g, gcd(a, b));
            }
        }
    }

    fn is_prime(value: usize) -> bool {
        super::is_prime(BigUint::from_usize(value).unwrap())
    }

    #[test]
    fn is_prime_test() {
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(is_prime(79));
        assert!(is_prime(113));
        assert!(!is_prime(79 * 113));
    }
}
