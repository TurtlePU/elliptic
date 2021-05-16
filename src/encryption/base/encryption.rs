use std::{error::Error, ops::Deref};

use rand::RngCore;

pub trait Enc {
    type Message: 'static;
    type Cipher: 'static;
}

pub trait PublicKeyEncryption: Enc {
    type PublicKey: Encryptor<Message = Self::Message, Cipher = Self::Cipher>
        + 'static;
    type Secret: Decryptor<Message = Self::Message, Cipher = Self::Cipher>
        + 'static;

    fn generate_keys(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::PublicKey, Self::Secret);
}

pub trait Encryptor: Enc {
    fn encrypt(
        &self,
        rng: &mut dyn RngCore,
        message: Self::Message,
    ) -> Self::Cipher;
}

pub trait Decryptor: Enc {
    type Error: Error + 'static;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error>;
}

pub trait PrivateKeyEncryption {
    type Secret: PrivateKey;

    fn generate_key(&self, rng: &mut dyn RngCore) -> Self::Secret;
}

pub trait PrivateKey: Encryptor + Decryptor {}

impl<E: Enc + ?Sized> Enc for Box<E> {
    type Message = E::Message;
    type Cipher = E::Cipher;
}

impl<E: Encryptor + ?Sized> Encryptor for Box<E> {
    fn encrypt(
        &self,
        rng: &mut dyn RngCore,
        message: Self::Message,
    ) -> Self::Cipher {
        self.deref().encrypt(rng, message)
    }
}

impl<E: Decryptor + ?Sized> Decryptor for Box<E> {
    type Error = E::Error;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        self.deref().decrypt(cipher)
    }
}

impl<X> PrivateKey for X where X: Encryptor + Decryptor {}
