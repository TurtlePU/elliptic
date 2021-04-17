pub trait KeyEncapsulation<Cipher> {
    type Key;
    type Encaps: Encapsulator<Cipher, Key = Self::Key>;
    type Decaps: Decapsulator<Cipher, Key = Self::Key>;

    fn generate_caps(&mut self, n: usize) -> (Self::Encaps, Self::Decaps);
}

pub trait Encapsulator<Cipher> {
    type Key;

    fn encapsulate(&mut self, n: usize) -> (Self::Key, Cipher);
}

pub trait Decapsulator<Cipher> {
    type Key;
    type Error;

    fn decapsulate(&self, cipher: Cipher) -> Result<Self::Key, Self::Error>;
}
