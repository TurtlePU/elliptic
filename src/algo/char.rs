use crate::traits::Field;

pub fn char_is<T>(n: usize) -> bool where T: Field {
    let mut x = T::one();
    for _ in 1..n {
        x = x + T::one();
    }
    x.is_zero()
}
