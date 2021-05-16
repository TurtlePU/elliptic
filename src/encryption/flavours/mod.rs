use rand::{distributions::Standard, prelude::Distribution, Rng, RngCore};

use crate::algebra::{
    algo::is_prime,
    curve::{Curve, EllipticPoint},
    traits::{Field, Sqrt},
};

use self::el_gamal::ElGamal;

mod el_gamal;
mod el_gamal_kem;

pub fn el_gamal_prime_curve<F, C>(
) -> ElGamal<impl Fn(&mut dyn RngCore) -> EllipticPoint<F, C>>
where
    F: Field + Sqrt,
    C: Curve<F>,
    Standard: Distribution<F>,
{
    assert!(is_prime(C::group_order()));
    ElGamal::from(|rng: &mut dyn RngCore| rng.gen::<EllipticPoint<F, C>>())
}
