use std::error::Error;

use rand::RngCore;
use thiserror::Error;

use crate::encryption::base::encryption::*;

pub struct DynEncryption<X>(pub X);

impl<X: Enc> Enc for DynEncryption<X> {
    type Message = X::Message;
    type Cipher = X::Cipher;
}

impl<X: PublicKeyEncryption> PublicKeyEncryption for DynEncryption<X> {
    type PublicKey =
        Box<dyn Encryptor<Message = Self::Message, Cipher = Self::Cipher>>;
    type Secret = Box<
        dyn Decryptor<
            Message = Self::Message,
            Cipher = Self::Cipher,
            Error = DynError,
        >,
    >;

    fn generate_keys(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_keys(rng);
        (Box::new(enc), Box::new(DynEncryption(dec)))
    }
}

impl<X: Encryptor> Encryptor for DynEncryption<X> {
    fn encrypt(
        &self,
        rng: &mut dyn RngCore,
        message: Self::Message,
    ) -> Self::Cipher {
        self.0.encrypt(rng, message)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct DynError(Box<dyn Error>);

impl<X: Decryptor> Decryptor for DynEncryption<X> {
    type Error = DynError;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        self.0
            .decrypt(cipher)
            .map_err(|err| DynError(Box::new(err)))
    }
}
