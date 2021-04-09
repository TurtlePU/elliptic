mod char;
mod extended_gcd;
pub mod inverse;

pub use extended_gcd::extended_gcd;
pub use self::char::char_is;

pub fn repeat_monoid<T, F>(app: F, cnt: usize, value: T, result: T) -> T
where T: Clone, F: Clone + Fn(T, T) -> T
{
    if cnt == 0 {
        result
    } else if cnt % 2 == 0 {
        repeat_monoid(app.clone(), cnt / 2, app(value.clone(), value), result)
    } else {
        repeat_monoid(app.clone(), cnt - 1, value.clone(), app(result, value))
    }
}

#[cfg(test)]
mod tests {
    use super::repeat_monoid;
    use std::ops::Add;

    #[test]
    fn repeats_correctly() {
        for n in 0..100000 {
            assert_eq!(repeat_monoid(usize::add, n, 1, 0), n);
        }
    }
}
