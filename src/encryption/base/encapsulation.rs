use std::error::Error;

use rand::RngCore;

pub trait Caps {
    type Cipher: 'static;
    type Key;
}

pub trait KeyEncapsulation: Caps {
    type Encaps: Encapsulator<Cipher = Self::Cipher, Key = Self::Key> + 'static;
    type Decaps: Decapsulator<Cipher = Self::Cipher, Key = Self::Key> + 'static;

    fn generate_caps(
        &self,
        rng: &mut dyn RngCore,
    ) -> (Self::Encaps, Self::Decaps);
}

pub trait Encapsulator: Caps {
    fn encapsulate(&self, rng: &mut dyn RngCore) -> (Self::Key, Self::Cipher);
}

pub trait Decapsulator: Caps {
    type Error: Error + 'static;

    fn decapsulate(
        &self,
        cipher: Self::Cipher,
    ) -> Result<Self::Key, Self::Error>;
}
