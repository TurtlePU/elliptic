use rand::Rng;

use super::{encapsulation::*, schemes::*};

pub struct StreamScheme<S>(S);
pub struct StreamMapper<T>(T);
pub struct StreamCapsule<T>(T);

impl<S> From<S> for StreamScheme<S> {
    fn from(scheme: S) -> Self {
        Self(scheme)
    }
}

impl<S: Enc> Enc for StreamScheme<S> {
    type Message = Vec<S::Message>;
    type Cipher = Vec<S::Cipher>;
}

impl<S: PublicKeyEncryption> PublicKeyEncryption for StreamScheme<S> {
    type PublicKey = StreamMapper<S::PublicKey>;
    type Secret = StreamMapper<S::Secret>;

    fn generate_keys(
        &self,
        rng: &mut impl Rng,
    ) -> (Self::PublicKey, Self::Secret) {
        let (enc, dec) = self.0.generate_keys(rng);
        (StreamMapper(enc), StreamMapper(dec))
    }
}

impl<E: Enc> Enc for StreamMapper<E> {
    type Message = Vec<E::Message>;
    type Cipher = Vec<E::Cipher>;
}

impl<E: Encryptor> Encryptor for StreamMapper<E> {
    fn encrypt(
        &self,
        rng: &mut impl Rng,
        message: Self::Message,
    ) -> Self::Cipher {
        message
            .into_iter()
            .map(|x| self.0.encrypt(rng, x))
            .collect()
    }
}

impl<D: Decryptor> Decryptor for StreamMapper<D> {
    type Error = D::Error;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error> {
        cipher.into_iter().map(|x| self.0.decrypt(x)).collect()
    }
}

impl<S: PrivateKeyEncryption> PrivateKeyEncryption for StreamScheme<S> {
    type Secret = StreamMapper<S::Secret>;

    fn generate_key(&self, rng: &mut impl Rng) -> Self::Secret {
        StreamMapper(self.0.generate_key(rng))
    }
}

impl<E: Caps> Caps for StreamScheme<E> {
    type Key = StreamMapper<E::Key>;
    type Cipher = E::Cipher;
}

impl<E: KeyEncapsulation> KeyEncapsulation for StreamScheme<E> {
    type Encaps = StreamCapsule<E::Encaps>;
    type Decaps = StreamCapsule<E::Decaps>;

    fn generate_caps(
        &self,
        rng: &mut impl Rng,
    ) -> (Self::Encaps, Self::Decaps) {
        let (enc, dec) = self.0.generate_caps(rng);
        (StreamCapsule(enc), StreamCapsule(dec))
    }
}

impl<E: Caps> Caps for StreamCapsule<E> {
    type Key = StreamMapper<E::Key>;
    type Cipher = E::Cipher;
}

impl<E: Encapsulator> Encapsulator for StreamCapsule<E> {
    fn encapsulate(&self, rng: &mut impl Rng) -> (Self::Key, Self::Cipher) {
        let (key, c) = self.0.encapsulate(rng);
        (StreamMapper(key), c)
    }
}

impl<E: Decapsulator> Decapsulator for StreamCapsule<E> {
    type Error = E::Error;

    fn decapsulate(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Key, Self::Error> {
        self.0.decapsulate(cipher).map(StreamMapper)
    }
}
