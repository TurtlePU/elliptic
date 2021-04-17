pub trait PublicKeyEncryption<Message, Cipher> {
    type PublicKey: Encryptor<Message, Cipher>;
    type Secret: Decryptor<Message, Cipher>;

    fn generate_keys(&mut self, n: usize) -> (Self::PublicKey, Self::Secret);
}

pub trait Encryptor<Message, Cipher> {
    fn encrypt(&mut self, message: Message) -> Cipher;
}

pub trait Decryptor<Message, Cipher> {
    type Error;

    fn decrypt(&self, cipher: Cipher) -> Result<Message, Self::Error>;
}

pub trait PrivateKeyEncryption<Message, Cipher> {
    type Secret: PrivateKey<Message, Cipher>;

    fn generate_key(&mut self, n: usize) -> Self::Secret;
}

pub trait PrivateKey<Message, Cipher>:
    Encryptor<Message, Cipher> + Decryptor<Message, Cipher>
{
}
