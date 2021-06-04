use std::{
    array::TryFromSliceError,
    convert::TryInto,
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

use num_bigint::BigUint;
use num_traits::{Inv, One, Pow, Zero};
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler},
        Standard,
    },
    prelude::Distribution,
    Rng,
};

use crate::{
    algebra::{
        algo::{extended_gcd, is_prime, repeat_monoid},
        traits::{Field, FinGroup, Group, Ring, Sqrt},
    },
    bytes::{Decoding, Deserialize, Encoding, Serialize},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zn<const N: usize>(usize);

impl<const N: usize> Group for Zn<N> {}

impl<const N: usize> FinGroup for Zn<N> {
    fn order() -> BigUint {
        BigUint::from(N)
    }
}

impl<const N: usize> Ring for Zn<N> {}

impl<const N: usize> Field for Zn<N> {}

impl<const N: usize> From<usize> for Zn<N> {
    fn from(n: usize) -> Self {
        Self(n % N)
    }
}

impl<const N: usize> From<Zn<N>> for usize {
    fn from(zn: Zn<N>) -> Self {
        zn.0
    }
}

impl<const N: usize> Distribution<Zn<N>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Zn<N> {
        Zn::from(rng.gen::<usize>())
    }
}

impl<const N: usize> Sqrt for Zn<N> {
    fn sqrt(self) -> Option<Self> {
        assert!(is_prime(Self::order()));
        assert!((N + 1) % 4 == 0);
        let sqrt = self.clone().pow((N + 1) / 4);
        if sqrt.clone().pow(2) == self {
            Some(sqrt)
        } else {
            None
        }
    }
}

impl<const N: usize> Add for Zn<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl<const N: usize> Neg for Zn<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from(N - self.0)
    }
}

impl<const N: usize> Sub for Zn<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const N: usize> Mul<isize> for Zn<N> {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        if rhs < 0 {
            -self * -rhs
        } else {
            Self::from(self.0 * rhs as usize)
        }
    }
}

impl<const N: usize> Sum for Zn<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::from(iter.map(|x| x.0).sum::<usize>())
    }
}

impl<const N: usize> Mul for Zn<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from(self.0 * rhs.0)
    }
}

impl<const N: usize> Pow<usize> for Zn<N> {
    type Output = Self;

    fn pow(self, rhs: usize) -> Self::Output {
        Self::from(repeat_monoid(usize::mul, rhs, self.0, 1))
    }
}

impl<const N: usize> Inv for Zn<N> {
    type Output = Self;

    fn inv(self) -> Self::Output {
        let n: isize = N.try_into().unwrap();
        let (gcd, mut inv, _) = extended_gcd(self.0.try_into().unwrap(), n);
        assert!(gcd.is_one());
        while inv < 0 {
            inv += n;
        }
        Self(inv.try_into().unwrap())
    }
}

impl<const N: usize> Div for Zn<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<const N: usize> Zero for Zn<N> {
    fn zero() -> Self {
        Self::from(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl<const N: usize> One for Zn<N> {
    fn one() -> Self {
        Self::from(1)
    }
}

impl<const N: usize> Encoding for Zn<N> {
    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Self> {
        assert!(N > u8::MAX as usize);
        stream.next().map(|x| usize::from(x).into())
    }
}

impl<const N: usize> Decoding for Zn<N> {
    type Error = <usize as TryInto<u8>>::Error;

    fn decode(self) -> Result<Vec<u8>, Self::Error> {
        self.0.try_into().map(|x| vec![x])
    }
}

impl<const N: usize> Serialize for Zn<N> {
    fn serialize(self) -> Vec<u8> {
        self.0.to_be_bytes().into()
    }
}

impl<const N: usize> Deserialize for Zn<N> {
    type Error = TryFromSliceError;

    fn deserialize(
        stream: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<Self>, Self::Error> {
        const BYTES: usize = std::mem::size_of::<usize>();
        let vec: Vec<_> = (0..BYTES).filter_map(|_| stream.next()).collect();
        if vec.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Self::from(usize::from_be_bytes(vec[..].try_into()?))))
        }
    }
}

pub struct UniformZn<const N: usize>(UniformInt<usize>);

impl<const N: usize> UniformSampler for UniformZn<N> {
    type X = Zn<N>;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self(UniformInt::new(
            usize::from(*low.borrow()),
            usize::from(*high.borrow()),
        ))
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        Self(UniformInt::new_inclusive(
            usize::from(*low.borrow()),
            usize::from(*high.borrow()),
        ))
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Zn::from(self.0.sample(rng))
    }
}

impl<const N: usize> SampleUniform for Zn<N> {
    type Sampler = UniformZn<N>;
}

#[cfg(test)]
mod tests {
    use num_traits::{Inv, Pow, Zero};

    use crate::algebra::{fields::Zn, traits::Sqrt};

    #[test]
    fn add() {
        assert!((Zn::<74>::from(69) + Zn::from(5)).is_zero());
        assert!(Zn::<180>::from(174) + Zn::from(389) == Zn::from(23));
        assert!(-Zn::<47>::from(111) == Zn::from(30));
    }

    #[test]
    fn mul() {
        assert!(Zn::<14>::from(19) * Zn::from(5) == Zn::from(11));
        assert!((Zn::<9>::from(81) * Zn::from(12326234)).is_zero());
    }

    #[test]
    fn inv() {
        assert!(Zn::<18>::from(5).inv() == Zn::from(11));
        assert!(Zn::<17>::from(8).inv() == Zn::from(15));
    }

    #[test]
    fn sqrt() {
        let a = Zn::<19>::from(11);
        let sqrt = a.clone().sqrt().unwrap();
        assert!(sqrt.pow(2) == a);
    }
}
