use super::{encapsulation::*, schemes::*};

pub struct HybridEncryption<E>(E);
pub struct HybridEncryptor<E>(E, usize);
pub struct HybridDecryptor<D>(D);

impl<E> From<E> for HybridEncryption<E> {
    fn from(scheme: E) -> Self {
        Self(scheme)
    }
}

impl<E, K> Enc for HybridEncryption<E>
where
    E: Caps<Key = K>,
    K: Enc,
{
    type Message = K::Message;
    type Cipher = (E::Cipher, K::Cipher);
}

impl<E, K> PublicKeyEncryption for HybridEncryption<E>
where
    E: KeyEncapsulation<Key = K>,
    K: PrivateKey,
{
    type PublicKey = HybridEncryptor<E::Encaps>;
    type Secret = HybridDecryptor<E::Decaps>;

    fn generate_keys(&mut self, n: usize) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_caps(n);
        (HybridEncryptor(enc, n), HybridDecryptor(dec))
    }
}

impl<E, K> Enc for HybridEncryptor<E>
where
    E: Caps<Key = K>,
    K: Enc,
{
    type Message = K::Message;
    type Cipher = (E::Cipher, K::Cipher);
}

impl<E, K> Encryptor for HybridEncryptor<E>
where
    E: Encapsulator<Key = K>,
    K: Encryptor,
{
    fn encrypt(&mut self, message: Self::Message) -> Self::Cipher {
        let (mut enc, c1) = self.0.encapsulate(self.1);
        (c1, enc.encrypt(message))
    }
}

impl<D, K> Enc for HybridDecryptor<D>
where
    D: Caps<Key = K>,
    K: Enc,
{
    type Message = K::Message;
    type Cipher = (D::Cipher, K::Cipher);
}

impl<D, K> Decryptor for HybridDecryptor<D>
where
    D: Decapsulator<Key = K>,
    K: Decryptor,
{
    type Error = HybridError<D::Error, K::Error>;

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
