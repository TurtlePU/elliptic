use rand::RngCore;

use self::el_gamal::ElGamal;

pub mod el_gamal;
pub mod el_gamal_kem;

pub fn el_gamal_const<T>(
    f: impl Fn() -> T,
) -> ElGamal<impl Fn(&mut dyn RngCore) -> T> {
    ElGamal {
        get_group_generator: move |_: &mut dyn RngCore| f(),
    }
}
