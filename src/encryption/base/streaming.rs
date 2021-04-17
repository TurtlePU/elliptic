use super::schemes::*;

pub struct StreamEncryption<S>(S);
pub struct StreamMapper<T>(T);

impl<S> From<S> for StreamEncryption<S> {
    fn from(scheme: S) -> Self {
        Self(scheme)
    }
}

impl<S, M, C> PublicKeyEncryption<Vec<M>, Vec<C>> for StreamEncryption<S>
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

impl<S, M, C> PrivateKeyEncryption<Vec<M>, Vec<C>> for StreamEncryption<S>
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
