use std::{convert::Infallible, marker::PhantomData};

use rand::{Rng, RngCore};

use crate::{algebra::traits::FinGroup, encryption::base::encryption::*};

pub struct ElGamal<F> {
    pub get_group_generator: F,
}

pub struct ElGamalPublicKey<T> {
    pub group_generator: T,
    pub key: T,
}

pub struct ElGamalSecret<T> {
    pub secret: isize,
    pub group: PhantomData<T>,
}

impl<F, T> Enc for ElGamal<F>
where
    F: Fn(&mut dyn RngCore) -> T,
    T: 'static,
{
    type Message = T;
    type Cipher = (T, T);
}

impl<F, T> PublicKeyEncryption for ElGamal<F>
where
    F: Fn(&mut dyn RngCore) -> T,
    T: FinGroup + 'static,
{
    type PublicKey = ElGamalPublicKey<T>;
    type Secret = ElGamalSecret<T>;

    fn generate_keys(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::PublicKey, Self::Secret) {
        let group_generator = (self.get_group_generator)(rng);
        let secret: isize = rng.gen();
        let key = group_generator.clone() * secret;
        (
            ElGamalPublicKey {
                group_generator,
                key,
            },
            ElGamalSecret {
                secret,
                group: PhantomData,
            },
        )
    }
}

impl<T> Enc for ElGamalPublicKey<T>
where
    T: 'static,
{
    type Message = T;
    type Cipher = (T, T);
}

impl<T> Encryptor for ElGamalPublicKey<T>
where
    T: FinGroup + 'static,
{
    fn encrypt(&self, rng: &mut dyn RngCore, message: T) -> (T, T) {
        let y: isize = rng.gen();
        (
            self.group_generator.clone() * y,
            self.key.clone() * y + message,
        )
    }
}

impl<T> Enc for ElGamalSecret<T>
where
    T: 'static,
{
    type Message = T;
    type Cipher = (T, T);
}

impl<T> Decryptor for ElGamalSecret<T>
where
    T: FinGroup + 'static,
{
    type Error = Infallible;

    fn decrypt(&self, (salt, cipher): (T, T)) -> Result<T, Self::Error> {
        Ok(cipher - salt * self.secret)
    }
}
