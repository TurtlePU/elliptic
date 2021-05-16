use std::error::Error;

use rand::RngCore;
use thiserror::Error;

use crate::encryption::base::{encapsulation::*, encryption::*};

pub struct Hybrid<X>(pub X);

impl<X, K> Enc for Hybrid<X>
where
    X: Caps<Key = K>,
    K: Enc,
{
    type Message = K::Message;
    type Cipher = (X::Cipher, K::Cipher);
}

impl<X, K> PublicKeyEncryption for Hybrid<X>
where
    X: KeyEncapsulation<Key = K>,
    K: PrivateKey,
{
    type PublicKey = Hybrid<X::Encaps>;
    type Secret = Hybrid<X::Decaps>;

    fn generate_keys(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_caps(rng);
        (Hybrid(enc), Hybrid(dec))
    }
}

impl<X, K> Encryptor for Hybrid<X>
where
    X: Encapsulator<Key = K>,
    K: Encryptor,
{
    fn encrypt(
        &self,
        rng: &mut dyn RngCore,
        message: Self::Message,
    ) -> Self::Cipher {
        let (enc, c1) = self.0.encapsulate(rng);
        (c1, enc.encrypt(rng, message))
    }
}

impl<X, K> Decryptor for Hybrid<X>
where
    X: Decapsulator<Key = K>,
    K: Decryptor,
{
    type Error = HybridError<X::Error, K::Error>;

    fn decrypt(
        &self,
        (c1, c2): Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        self.0
            .decapsulate(c1)
            .map_err(HybridError::Decapsulation)?
            .decrypt(c2)
            .map_err(HybridError::Decryption)
    }
}

#[derive(Debug, Error)]
pub enum HybridError<C: Error + 'static, R: Error + 'static> {
    #[error("Decapsulation error")]
    Decapsulation(#[source] C),
    #[error("Decryption error")]
    Decryption(#[source] R),
}
