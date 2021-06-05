use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::{Num, One};
use rand::RngCore;
use thiserror::Error;

use crate::{
    algebra::{
        curve::{Curve, EllipticPoint, Encoder},
        fields::zn::{BigPrime, Zn},
    },
    encryption::{
        extensions::{make_dyn, public_encryption, PublicEncObject},
        flavours::el_gamal::ElGamal,
    },
};

pub struct N224;

impl BigPrime for N224 {
    fn value() -> num_bigint::BigUint {
        (BigUint::one() << 224) - (BigUint::one() << 96) + BigUint::one()
    }
}

pub type Z224 = Zn<N224>;

pub struct P224;

impl Curve<Z224> for P224 {
    fn group_order() -> BigUint {
        hex("ffffffffffffffffffffffffffff16a2e0b8f03e13dd29455c5c2a3d")
    }

    fn a() -> Z224 {
        -Z224::from(3)
    }

    fn b() -> Z224 {
        z224("b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4")
    }
}

pub type Point = EllipticPoint<Z224, P224>;

const BUCKET_ORDER: usize = 2;
const BUCKET: usize = 1 << (BUCKET_ORDER * 8);

impl Encoder<Point> for P224 {
    type Error = DecodingError;

    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Point> {
        let len = bytevec_len();
        let bytes = (0..len).filter_map(|_| stream.next()).collect_vec();
        if bytes.len() == 0 {
            return None;
        }
        let mut x =
            Z224::from(BigUint::from_bytes_le(&bytes)) * Z224::from(BUCKET);
        for _ in 0..BUCKET {
            if let Some(y) = P224::solve(x.clone()) {
                return Some(P224::affine(x, y).unwrap());
            }
            x = x + Z224::one();
        }
        panic!("bucket exhausted")
    }

    fn decode(item: Point) -> Result<Vec<u8>, Self::Error> {
        let (x, _) =
            Option::<(_, _)>::from(item).ok_or(DecodingError::IsZero)?;
        let x: BigUint = x.into();
        let bytes = (x / BigUint::from(BUCKET)).to_bytes_le();
        if bytes.len() > bytevec_len() {
            Err(DecodingError::TooBig)
        } else {
            Ok(bytes)
        }
    }
}

fn bytevec_len() -> usize {
    N224::bytes() - 1 - BUCKET_ORDER
}

#[derive(Debug, Error)]
pub enum DecodingError {
    #[error("tried to decode infinite point")]
    IsZero,
    #[error("tried to decode number which is too big")]
    TooBig,
}

pub fn ec_encryptor() -> PublicEncObject {
    make_dyn(public_encryption(ElGamal {
        get_group_generator: |_: &mut dyn RngCore| {
            P224::affine(
                z224(
                    "b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21",
                ),
                z224(
                    "bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34",
                ),
            )
            .unwrap()
        },
    }))
}

fn hex(string: &str) -> BigUint {
    BigUint::from_str_radix(string, 16).unwrap()
}

fn z224(string: &str) -> Z224 {
    hex(string).into()
}
