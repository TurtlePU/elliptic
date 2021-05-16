use rand::Rng;

pub trait Enc {
    type Message;
    type Cipher;
}

pub trait PublicKeyEncryption: Enc {
    type PublicKey: Encryptor<Message = Self::Message, Cipher = Self::Cipher>;
    type Secret: Decryptor<Message = Self::Message, Cipher = Self::Cipher>;

    fn generate_keys(
        &self,
        rng: &mut impl Rng,
    ) -> (Self::PublicKey, Self::Secret);
}

pub trait Encryptor: Enc {
    fn encrypt(
        &self,
        rng: &mut impl Rng,
        message: Self::Message,
    ) -> Self::Cipher;
}

pub trait Decryptor: Enc {
    type Error;

    fn decrypt(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Message, Self::Error>;
}

pub trait PrivateKeyEncryption {
    type Secret: PrivateKey;

    fn generate_key(&self, rng: &mut impl Rng) -> Self::Secret;
}

pub trait PrivateKey: Encryptor + Decryptor {}

impl<X> PrivateKey for X where X: Encryptor + Decryptor {}
