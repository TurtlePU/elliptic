use std::marker::PhantomData;

use rand::{Rng, RngCore};

use crate::{algebra::traits::FinGroup, encryption::base::schemes::*};

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
{
    type Message = T;
    type Cipher = (T, T);
}

impl<F, T> PublicKeyEncryption for ElGamal<F>
where
    F: Fn(&mut dyn RngCore) -> T,
    T: FinGroup,
{
    type PublicKey = ElGamalPublicKey<T>;
    type Secret = ElGamalSecret<T>;

    fn generate_keys(
        &self,
        rng: &mut impl Rng,
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

impl<T> Enc for ElGamalPublicKey<T> {
    type Message = T;
    type Cipher = (T, T);
}

impl<T: FinGroup> Encryptor for ElGamalPublicKey<T> {
    fn encrypt(&self, rng: &mut impl Rng, message: T) -> (T, T) {
        let y: isize = rng.gen();
        (
            self.group_generator.clone() * y,
            self.key.clone() * y + message,
        )
    }
}

impl<T> Enc for ElGamalSecret<T> {
    type Message = T;
    type Cipher = (T, T);
}

impl<T: FinGroup> Decryptor for ElGamalSecret<T> {
    type Error = ();

    fn decrypt(&self, (salt, cipher): (T, T)) -> Result<T, Self::Error> {
        Ok(cipher - salt * self.secret)
    }
}
