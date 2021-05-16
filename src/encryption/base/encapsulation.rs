use rand::Rng;

pub trait Caps {
    type Cipher;
    type Key;
}

pub trait KeyEncapsulation: Caps {
    type Encaps: Encapsulator<Cipher = Self::Cipher, Key = Self::Key>;
    type Decaps: Decapsulator<Cipher = Self::Cipher, Key = Self::Key>;

    fn generate_caps(&self, rng: &mut impl Rng) -> (Self::Encaps, Self::Decaps);
}

pub trait Encapsulator: Caps {
    fn encapsulate(&self, rng: &mut impl Rng) -> (Self::Key, Self::Cipher);
}

pub trait Decapsulator: Caps {
    type Error;

    fn decapsulate(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Key, Self::Error>;
}
