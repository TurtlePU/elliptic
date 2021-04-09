use crate::{poly::Poly, traits::Integral};

use super::extended_gcd;

#[derive(Debug)]
pub struct NotInvertible;

pub fn modular_inverse<T>(arg: T, modulo: T) -> Result<T, NotInvertible>
where T: Integral {
    let (gcd, x, _) = extended_gcd(arg, modulo);
    if !gcd.is_one() {
        Err(NotInvertible)
    } else {
        Ok(x)
    }
}

#[derive(Debug)]
pub enum Proportional {
    ReducibleModulo,
    ArgIsAMultiple,
}

pub fn poly_inverse<T>(arg: Poly<T>, modulo: Poly<T>)
    -> Result<Poly<T>, Proportional>
where Poly<T>: Integral
{
    let (gcd, x, _) = extended_gcd(arg, modulo);
    if gcd.degree() > 0 {
        if x.degree() > 0 {
            Err(Proportional::ReducibleModulo)
        } else {
            Err(Proportional::ArgIsAMultiple)
        }
    } else {
        Ok(x / gcd)
    }
}
