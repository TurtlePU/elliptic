use itertools::Itertools;
use num_bigint::{BigInt, BigUint};
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

/// From https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186-draft.pdf
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

impl Encoder<Point> for P224 {
    type Error = NotFound;

    fn encode(stream: &mut impl Iterator<Item = u8>) -> Option<Point> {
        stream.next().map(|x| generator() * BigInt::from(x))
    }

    fn decode(item: Point) -> Result<Vec<u8>, Self::Error> {
        for x in u8::MIN..=u8::MAX {
            if item == generator() * BigInt::from(x) {
                return Ok(vec![x]);
            }
        }
        Err(NotFound)
    }
}

#[derive(Debug, Error)]
#[error("inverse not found, order is too big")]
pub struct NotFound;

pub fn ec_encryptor() -> PublicEncObject {
    make_dyn(public_encryption(ElGamal {
        get_group_generator: |_: &mut dyn RngCore| generator(),
    }))
}

pub fn generator() -> Point {
    P224::affine(
        z224("b70e0cbd6bb4bf7f321390b94a03c1d356c21122343280d6115c1d21"),
        z224("bd376388b5f723fb4c22dfe6cd4375a05a07476444d5819985007e34"),
    )
    .unwrap()
}

pub fn hex(string: &str) -> BigUint {
    let string = string.split_whitespace().collect_vec().join("");
    BigUint::from_str_radix(&string, 16).unwrap()
}

fn z224(string: &str) -> Z224 {
    hex(string).into()
}
