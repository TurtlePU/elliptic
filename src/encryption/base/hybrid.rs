use super::{schemes::*, encapsulation::*};

pub struct HybridEncryption<E>(E);
pub struct HybridEncryptor<E>(E, usize);
pub struct HybridDecryptor<D>(D);

impl<E> From<E> for HybridEncryption<E> {
    fn from(scheme: E) -> Self {
        Self(scheme)
    }
}

impl<E, K, M, C1, C2> PublicKeyEncryption<M, (C1, C2)> for HybridEncryption<E>
where
    E: KeyEncapsulation<C1, Key = K>,
    K: PrivateKey<M, C2>,
{
    type PublicKey = HybridEncryptor<E::Encaps>;
    type Secret = HybridDecryptor<E::Decaps>;

    fn generate_keys(&mut self, n: usize) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_caps(n);
        (HybridEncryptor(enc, n), HybridDecryptor(dec))
    }
}

impl<E, K, M, C1, C2> Encryptor<M, (C1, C2)> for HybridEncryptor<E>
where
    E: Encapsulator<C1, Key = K>,
    K: Encryptor<M, C2>,
{
    fn encrypt(&mut self, message: M) -> (C1, C2) {
        let (mut enc, c1) = self.0.encapsulate(self.1);
        (c1, enc.encrypt(message))
    }
}

impl<D, K, M, C1, C2> Decryptor<M, (C1, C2)> for HybridDecryptor<D>
where
    D: Decapsulator<C1, Key = K>,
    K: Decryptor<M, C2>,
{
    type Error = HybridError<D::Error, K::Error>;

    fn decrypt(&self, (c1, c2): (C1, C2)) -> Result<M, Self::Error> {
        self.0
            .decapsulate(c1)?
            .decrypt(c2)
            .map_err(HybridError::Decryption)
    }
}

pub enum HybridError<E1, E2> {
    Decapsulation(E1),
    Decryption(E2),
}

impl<E1, E2> From<E1> for HybridError<E1, E2> {
    fn from(err: E1) -> Self {
        Self::Decapsulation(err)
    }
}
