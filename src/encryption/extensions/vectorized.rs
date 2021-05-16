use rand::RngCore;

use crate::encryption::base::{encapsulation::*, encryption::*};

pub struct Vectorized<X>(pub X);

impl<X: Enc> Enc for Vectorized<X> {
    type Message = Vec<X::Message>;
    type Cipher = Vec<X::Cipher>;
}

impl<X: PublicKeyEncryption> PublicKeyEncryption for Vectorized<X> {
    type PublicKey = Vectorized<X::PublicKey>;
    type Secret = Vectorized<X::Secret>;

    fn generate_keys(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_keys(rng);
        (Vectorized(enc), Vectorized(dec))
    }
}

impl<X: Encryptor> Encryptor for Vectorized<X> {
    fn encrypt(
        &self,
        rng: &mut dyn RngCore,
        message: Self::Message,
    ) -> Self::Cipher {
        message
            .into_iter()
            .map(|x| self.0.encrypt(rng, x))
            .collect()
    }
}

impl<X: Decryptor> Decryptor for Vectorized<X> {
    type Error = X::Error;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        cipher.into_iter().map(|x| self.0.decrypt(x)).collect()
    }
}

impl<X: PrivateKeyEncryption> PrivateKeyEncryption for Vectorized<X> {
    type Secret = Vectorized<X::Secret>;

    fn generate_key(&self, rng: &mut dyn RngCore) -> Self::Secret {
        Vectorized(self.0.generate_key(rng))
    }
}

impl<X: Caps> Caps for Vectorized<X> {
    type Key = Vectorized<X::Key>;
    type Cipher = X::Cipher;
}

impl<X: KeyEncapsulation> KeyEncapsulation for Vectorized<X> {
    type Encaps = Vectorized<X::Encaps>;
    type Decaps = Vectorized<X::Decaps>;

    fn generate_caps(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::Encaps, Self::Decaps) {
        let (enc, dec) = self.0.generate_caps(rng);
        (Vectorized(enc), Vectorized(dec))
    }
}

impl<X: Encapsulator> Encapsulator for Vectorized<X> {
    fn encapsulate(&self, rng: &mut dyn RngCore) -> (Self::Key, Self::Cipher) {
        let (key, c) = self.0.encapsulate(rng);
        (Vectorized(key), c)
    }
}

impl<X: Decapsulator> Decapsulator for Vectorized<X> {
    type Error = X::Error;

    fn decapsulate(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Key, Self::Error> {
        self.0.decapsulate(cipher).map(Vectorized)
    }
}
