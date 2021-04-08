use std::{iter::Sum, ops::{Add, Div, Mul, Sub}};

use num_traits::{One, Zero};

use crate::poly::Poly;

use super::extended_gcd;

#[derive(Debug)]
pub struct NotInvertible;

pub fn modular_inverse<T>(arg: T, modulo: T) -> Result<T, NotInvertible>
where T: Zero + One + Clone + Ord +
         Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
{
    let (mut x, y) = extended_gcd(arg.clone(), modulo.clone());
    let gcd = arg * x.clone() + modulo.clone() * y;
    if !gcd.is_one() {
        Err(NotInvertible)
    } else {
        while x < T::zero() {
            x = x + modulo.clone();
        }
        Ok(x)
    }
}

pub enum Proportional {
    ReducibleModulo,
    ArgIsAMultiple,
}

pub fn poly_inverse<T>(arg: Poly<T>, modulo: Poly<T>)
    -> Result<Poly<T>, Proportional>
where T: Zero + One + Clone + Sum +
         Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>
{
    let (x, y) = extended_gcd(arg.clone(), modulo.clone());
    let gcd = arg * x.clone() + modulo * y;
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
