use std::marker::PhantomData;

use rand::Rng;

use crate::{algebra::traits::Group, encryption::base::schemes::*};

pub struct ElGamal<T, R> {
    generator: T,
    random: R,
}

pub struct ElGamalPublicKey<T, R> {
    generator: T,
    random: R,
    key: T,
}

pub struct ElGamalSecret<T>(isize, PhantomData<T>);

impl<T: Group, R: Rng + Clone> PublicKeyEncryption<T, (T, T)>
    for ElGamal<T, R>
{
    type PublicKey = ElGamalPublicKey<T, R>;
    type Secret = ElGamalSecret<T>;

    fn generate_keys(&mut self, _: usize) -> (Self::PublicKey, Self::Secret) {
        let x: isize = self.random.gen();
        let k = self.generator.clone() * x;
        (
            ElGamalPublicKey {
                generator: self.generator.clone(),
                random: self.random.clone(),
                key: k,
            },
            ElGamalSecret(x, PhantomData)
        )
    }
}

impl<T: Group, R: Rng> Encryptor<T, (T, T)> for ElGamalPublicKey<T, R> {
    fn encrypt(&mut self, message: T) -> (T, T) {
        let y: isize = self.random.gen();
        (self.generator.clone() * y, self.key.clone() * y + message)
    }
}

impl<T: Group> Decryptor<T, (T, T)> for ElGamalSecret<T> {
    type Error = ();

    fn decrypt(&self, (c1, c2): (T, T)) -> Result<T, Self::Error> {
        Ok(c2 - c1 * self.0)
    }
}
