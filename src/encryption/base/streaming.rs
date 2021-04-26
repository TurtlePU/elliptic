use super::{encapsulation::*, schemes::*};

pub struct StreamScheme<S>(S);
pub struct StreamMapper<T>(T);
pub struct StreamCapsule<T>(T);

impl<S> From<S> for StreamScheme<S> {
    fn from(scheme: S) -> Self {
        Self(scheme)
    }
}

impl<S, M, C> PublicKeyEncryption<Vec<M>, Vec<C>> for StreamScheme<S>
where
    S: PublicKeyEncryption<M, C>,
{
    type PublicKey = StreamMapper<S::PublicKey>;
    type Secret = StreamMapper<S::Secret>;

    fn generate_keys(&mut self, n: usize) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_keys(n);
        (StreamMapper(enc), StreamMapper(dec))
    }
}

impl<E, M, C> Encryptor<Vec<M>, Vec<C>> for StreamMapper<E>
where
    E: Encryptor<M, C>,
{
    fn encrypt(&mut self, message: Vec<M>) -> Vec<C> {
        message.into_iter().map(|x| self.0.encrypt(x)).collect()
    }
}

impl<D, M, C> Decryptor<Vec<M>, Vec<C>> for StreamMapper<D>
where
    D: Decryptor<M, C>,
{
    type Error = D::Error;

    fn decrypt(&self, cipher: Vec<C>) -> Result<Vec<M>, Self::Error> {
        cipher.into_iter().map(|x| self.0.decrypt(x)).collect()
    }
}

impl<S, M, C> PrivateKeyEncryption<Vec<M>, Vec<C>> for StreamScheme<S>
where
    S: PrivateKeyEncryption<M, C>,
{
    type Secret = StreamMapper<S::Secret>;

    fn generate_key(&mut self, n: usize) -> Self::Secret {
        StreamMapper(self.0.generate_key(n))
    }
}

impl<P, M, C> PrivateKey<Vec<M>, Vec<C>> for StreamMapper<P> where
    P: PrivateKey<M, C>
{
}

impl<E, C> KeyEncapsulation<C> for StreamScheme<E>
where
    E: KeyEncapsulation<C>,
{
    type Key = StreamMapper<E::Key>;
    type Encaps = StreamCapsule<E::Encaps>;
    type Decaps = StreamCapsule<E::Decaps>;

    fn generate_caps(&mut self, n: usize) -> (Self::Encaps, Self::Decaps) {
        let (enc, dec) = self.0.generate_caps(n);
        (StreamCapsule(enc), StreamCapsule(dec))
    }
}

impl<E, C> Encapsulator<C> for StreamCapsule<E>
where
    E: Encapsulator<C>,
{
    type Key = StreamMapper<E::Key>;

    fn encapsulate(&mut self, n: usize) -> (Self::Key, C) {
        let (key, c) = self.0.encapsulate(n);
        (StreamMapper(key), c)
    }
}

impl<E, C> Decapsulator<C> for StreamCapsule<E>
where
    E: Decapsulator<C>,
{
    type Key = StreamMapper<E::Key>;
    type Error = E::Error;

    fn decapsulate(&self, cipher: C) -> Result<Self::Key, Self::Error> {
        self.0.decapsulate(cipher).map(StreamMapper)
    }
}
