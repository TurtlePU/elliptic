use crate::{algebra::traits::FinGroup, encryption::base::encapsulation::Caps};
use crate::encryption::base::encapsulation::{
    Decapsulator, Encapsulator, KeyEncapsulation,
};
use rand::Rng;
use std::marker::PhantomData;

pub struct ElGamalKem<Gen, F, R> {
    get_generator: Gen,
    get_convert: F,
    random: R,
}

pub struct ElGamalEncaps<G, H, R> {
    public: G,
    generator: G,
    convert: H,
    random: R,
}

pub struct ElGamalDecaps<G, H> {
    secret: isize,
    convert: H,
    _phantom: PhantomData<G>,
}

impl<Gen, F, R, G, H, K> Caps for ElGamalKem<Gen, F, R>
where
    Gen: Fn(&mut R) -> G,
    F: Fn(&mut R, usize) -> H,
    H: Clone + Fn(G) -> K,
{
    type Key = K;
    type Cipher = G;
}

impl<Gen, F, R, G, H, K> KeyEncapsulation for ElGamalKem<Gen, F, R>
where
    Gen: Fn(&mut R) -> G,
    F: Fn(&mut R, usize) -> H,
    R: Rng + Clone,
    G: FinGroup,
    H: Clone + Fn(G) -> K,
{
    type Encaps = ElGamalEncaps<G, H, R>;
    type Decaps = ElGamalDecaps<G, H>;

    fn generate_caps(&mut self, n: usize) -> (Self::Encaps, Self::Decaps) {
        let generator = (self.get_generator)(&mut self.random);
        let convert = (self.get_convert)(&mut self.random, n);
        let secret: isize = self.random.gen();
        let public = generator.clone() * secret;
        (
            ElGamalEncaps {
                public,
                generator,
                convert: convert.clone(),
                random: self.random.clone(),
            },
            ElGamalDecaps {
                secret,
                convert,
                _phantom: PhantomData,
            },
        )
    }
}

impl<G, H, R, K> Caps for ElGamalEncaps<G, H, R> where H: Fn(G) -> K {
    type Key = K;
    type Cipher = G;
}

impl<G, H, R, K> Encapsulator for ElGamalEncaps<G, H, R>
where
    G: FinGroup,
    R: Rng,
    H: Fn(G) -> K,
{
    fn encapsulate(&mut self, _: usize) -> (Self::Key, Self::Cipher) {
        let y: isize = self.random.gen();
        (
            (self.convert)(self.public.clone() * y),
            self.generator.clone() * y,
        )
    }
}

impl<G, H, K> Caps for ElGamalDecaps<G, H> where H: Fn(G) -> K {
    type Key = K;
    type Cipher = G;
}

impl<G, H, K> Decapsulator for ElGamalDecaps<G, H>
where
    G: FinGroup,
    H: Fn(G) -> K,
{
    type Error = ();

    fn decapsulate(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Key, Self::Error> {
        Ok((self.convert)(cipher * self.secret))
    }
}
