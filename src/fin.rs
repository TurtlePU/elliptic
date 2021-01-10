use super::{group::Group, poly::Poly, ring::Ring};

struct Fin<R, T>(R, Poly<T>);

impl<R, T> Ring<T> for Fin<R, T> {
    // TODO: implement Ring for Fin
}
