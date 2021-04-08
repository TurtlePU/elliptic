use crate::traits::Integral;

/// Returns (g, x, y) such that a * x + b * y = g = gcd(a, b).
pub fn extended_gcd<T>(a: T, b: T) -> (T, T, T) where T: Integral {
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

#[cfg(test)]
mod tests {
    use super::{replace, extended_gcd};

    fn gcd(mut a: isize, mut b: isize) -> isize {
        while b != 0 {
            a = replace(a % b, &mut b);
        }
        a
    }

    #[test]
    fn counts_gcd() {
        let (a, b) = (4, 18);
        let (g, x, y) = extended_gcd(a, b);
        assert_eq!(g, a * x + b * y);
        assert_eq!(g, gcd(a, b));
    }
}
