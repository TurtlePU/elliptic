use std::marker::PhantomData;

use rand::Rng;

use crate::{algebra::traits::FinGroup, encryption::base::schemes::*};

pub struct ElGamal<F, R> {
    get_generator: F,
    random: R,
}

pub struct ElGamalPublicKey<T, R> {
    generator: T,
    random: R,
    key: T,
}

pub struct ElGamalSecret<T>(isize, PhantomData<T>);

impl<F, T, R> PublicKeyEncryption<T, (T, T)> for ElGamal<F, R>
where
    F: Fn(&mut R) -> T,
    T: FinGroup,
    R: Rng + Clone,
{
    type PublicKey = ElGamalPublicKey<T, R>;
    type Secret = ElGamalSecret<T>;

    fn generate_keys(&mut self, _: usize) -> (Self::PublicKey, Self::Secret) {
        let generator = (self.get_generator)(&mut self.random);
        let x: isize = self.random.gen();
        let k = generator.clone() * x;
        (
            ElGamalPublicKey {
                generator,
                random: self.random.clone(),
                key: k,
            },
            ElGamalSecret(x, PhantomData),
        )
    }
}

impl<T: FinGroup, R: Rng> Encryptor<T, (T, T)> for ElGamalPublicKey<T, R> {
    fn encrypt(&mut self, message: T) -> (T, T) {
        let y: isize = self.random.gen();
        (self.generator.clone() * y, self.key.clone() * y + message)
    }
}

impl<T: FinGroup> Decryptor<T, (T, T)> for ElGamalSecret<T> {
    type Error = ();

    fn decrypt(&self, (c1, c2): (T, T)) -> Result<T, Self::Error> {
        Ok(c2 - c1 * self.0)
    }
}
