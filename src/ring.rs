use super::{group::Group, poly::Poly};

pub trait Ring<T> : Group<T> {
    fn one(&self) -> T;
    fn mul(&self, a: T, b: T) -> T;
    fn inv(&self, x: T) -> Option<T>;

    fn div(&self, x: T, y: T) -> Option<T> {
        Some(self.mul(x, self.inv(y)?))
    }
}

pub fn div_poly<T>(
    ring: impl Ring<T>,
    a: Poly<T>,
    b: Poly<T>,
) -> Option<Poly<T>> {
    todo!()
}
