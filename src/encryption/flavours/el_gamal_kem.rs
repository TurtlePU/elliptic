use crate::{algebra::traits::FinGroup, encryption::base::encapsulation::*};
use rand::{Rng, RngCore};
use std::marker::PhantomData;

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
    pub secret: isize,
    pub key_from_group: F,
    pub group: PhantomData<G>,
}

impl<G0, F0, G, F, K> Caps for ElGamalKem<G0, F0>
where
    G0: Fn(&mut dyn RngCore) -> G,
    F0: Fn(&mut dyn RngCore) -> F,
    F: Fn(G) -> K,
{
    type Key = K;
    type Cipher = G;
}

impl<G0, F0, G, F, K> KeyEncapsulation for ElGamalKem<G0, F0>
where
    G0: Fn(&mut dyn RngCore) -> G,
    F0: Fn(&mut dyn RngCore) -> F,
    G: FinGroup,
    F: Clone + Fn(G) -> K,
{
    type Encaps = ElGamalEncaps<G, F>;
    type Decaps = ElGamalDecaps<G, F>;

    fn generate_caps(
        &self,
        rng: &mut impl Rng,
    ) -> (Self::Encaps, Self::Decaps) {
        let group_generator = (self.group_generator_gen)(rng);
        let key_from_group = (self.key_from_group_gen)(rng);
        let secret: isize = rng.gen();
        let group_key = group_generator.clone() * secret;
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

impl<G, F, K> Caps for ElGamalEncaps<G, F>
where
    F: Fn(G) -> K,
{
    type Key = K;
    type Cipher = G;
}

impl<G, F, K> Encapsulator for ElGamalEncaps<G, F>
where
    G: FinGroup,
    F: Fn(G) -> K,
{
    fn encapsulate(&self, rng: &mut impl Rng) -> (Self::Key, Self::Cipher) {
        let y: isize = rng.gen();
        (
            (self.key_from_group)(self.group_key.clone() * y),
            self.group_generator.clone() * y,
        )
    }
}

impl<G, F, K> Caps for ElGamalDecaps<G, F>
where
    F: Fn(G) -> K,
{
    type Key = K;
    type Cipher = G;
}

impl<G, F, K> Decapsulator for ElGamalDecaps<G, F>
where
    G: FinGroup,
    F: Fn(G) -> K,
{
    type Error = ();

    fn decapsulate(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Key, Self::Error> {
        Ok((self.key_from_group)(cipher * self.secret))
    }
}
