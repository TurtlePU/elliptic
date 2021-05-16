use rand::Rng;

use super::{encapsulation::*, encryption::*};

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
        rng: &mut impl Rng,
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
        rng: &mut impl Rng,
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

pub enum HybridError<E1, E2> {
    Decapsulation(E1),
    Decryption(E2),
}
