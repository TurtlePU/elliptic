pub trait Enc {
    type Message;
    type Cipher;
}

pub trait PublicKeyEncryption: Enc {
    type PublicKey: Encryptor<Message = Self::Message, Cipher = Self::Cipher>;
    type Secret: Decryptor<Message = Self::Message, Cipher = Self::Cipher>;

    fn generate_keys(&mut self, n: usize) -> (Self::PublicKey, Self::Secret);
}

pub trait Encryptor: Enc {
    fn encrypt(&mut self, message: Self::Message) -> Self::Cipher;
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

    fn generate_key(&mut self, n: usize) -> Self::Secret;
}

pub trait PrivateKey: Encryptor + Decryptor {}
