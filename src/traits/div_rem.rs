pub trait DivRem<Rhs = Self> {
    type Output;

    fn div_rem(&mut self, rhs: Rhs) -> Self::Output;
}
