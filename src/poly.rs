#[derive(Clone, Debug)]
pub struct Poly<T>(Vec<T>);

impl<T> Poly<T> {
    pub fn trimmed_with(
        iter: impl DoubleEndedIterator<Item = T>,
        pred: impl Fn(&T) -> bool,
    ) -> Poly<T> {
        let mut vec: Vec<_> = iter.rev().skip_while(pred).collect();
        vec.reverse();
        Poly(vec)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}

impl<T> From<T> for Poly<T> {
    fn from(t: T) -> Self {
        Self(vec![t])
    }
}

impl<T> Default for Poly<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T> IntoIterator for Poly<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Poly<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> std::iter::FromIterator<T> for Poly<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T> std::ops::Index<usize> for Poly<T> where {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.0[index]
    }
}
