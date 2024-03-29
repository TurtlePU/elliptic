use crate::{algebra::traits::FinGroup, encryption::base::encapsulation::*};
use num_bigint::BigInt;
use num_traits::One;
use rand::{Rng, RngCore};
use std::{convert::Infallible, marker::PhantomData};

pub struct ElGamalKem<G, F> {
    pub group_generator_gen: G,
    pub key_from_group_gen: F,
}

pub struct ElGamalEncaps<G, F> {
    pub group_key: G,
    pub group_generator: G,
    pub key_from_group: F,
}

pub struct ElGamalDecaps<G, F> {
    pub secret: BigInt,
    pub key_from_group: F,
    pub group: PhantomData<G>,
}

impl<G0, F0, G, F, K> Caps for ElGamalKem<G0, F0>
where
    G0: Fn(&mut dyn RngCore) -> G,
    F0: Fn(&mut dyn RngCore) -> F,
    G: 'static,
    F: Fn(G) -> K,
{
    type Key = K;
    type Cipher = G;
}

impl<G0, F0, G, F, K> KeyEncapsulation for ElGamalKem<G0, F0>
where
    G0: Fn(&mut dyn RngCore) -> G,
    F0: Fn(&mut dyn RngCore) -> F,
    G: FinGroup + 'static,
    F: Clone + Fn(G) -> K + 'static,
{
    type Encaps = ElGamalEncaps<G, F>;
    type Decaps = ElGamalDecaps<G, F>;

    fn generate_caps(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::Encaps, Self::Decaps) {
        let group_generator = (self.group_generator_gen)(rng);
        let key_from_group = (self.key_from_group_gen)(rng);
        let secret = rng.gen_range(BigInt::one()..G::order().into());
        let group_key = group_generator.clone() * secret.clone();
        (
            ElGamalEncaps {
                group_key,
                group_generator,
                key_from_group: key_from_group.clone(),
            },
            ElGamalDecaps {
                secret,
                key_from_group,
                group: PhantomData,
            },
        )
    }
}

impl<G, F, K> Encapsulator for ElGamalEncaps<G, F>
where
    G: FinGroup + 'static,
    F: Fn(G) -> K,
{
    fn encapsulate(&self, rng: &mut dyn RngCore) -> (Self::Key, Self::Cipher) {
        let y = rng.gen_range(BigInt::one()..G::order().into());
        (
            (self.key_from_group)(self.group_key.clone() * y.clone()),
            self.group_generator.clone() * y,
        )
    }
}

impl<G, F, K> Decapsulator for ElGamalDecaps<G, F>
where
    G: FinGroup + 'static,
    F: Fn(G) -> K,
{
    type Error = Infallible;

    fn decapsulate(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Key, Self::Error> {
        Ok((self.key_from_group)(cipher * self.secret.clone()))
    }
}

impl<G, F, K> Caps for ElGamalEncaps<G, F>
where
    G: 'static,
    F: Fn(G) -> K,
{
    type Key = K;
    type Cipher = G;
}

impl<G, F, K> Caps for ElGamalDecaps<G, F>
where
    G: 'static,
    F: Fn(G) -> K,
{
    type Key = K;
    type Cipher = G;
}
