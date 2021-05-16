use crate::bytes::{Decoding, Deserialize, Encoding, Serialize};

use self::{
    hybrid::Hybrid,
    object::{DynEncryption, DynError},
    stringer::Stringer,
    vectorized::Vectorized,
};

use super::base::{
    encapsulation::KeyEncapsulation,
    encryption::{Decryptor, Encryptor, PrivateKey, PublicKeyEncryption},
};

pub mod hybrid;
pub mod object;
pub mod stringer;
pub mod vectorized;

pub type EncryptorObject =
    Box<dyn Encryptor<Message = String, Cipher = String>>;

pub type DecryptorObject =
    Box<dyn Decryptor<Message = String, Cipher = String, Error = DynError>>;

pub type PublicEncObject = Box<
    dyn PublicKeyEncryption<
        Message = String,
        Cipher = String,
        PublicKey = EncryptorObject,
        Secret = DecryptorObject,
    >,
>;

pub fn public_encryption<E>(
    encryption: E,
) -> impl PublicKeyEncryption<Message = String, Cipher = String>
where
    E: PublicKeyEncryption + 'static,
    E::Message: Encoding + Decoding,
    E::Cipher: Serialize + Deserialize,
{
    Stringer(Vectorized(encryption))
}

pub fn hybrid_encryption<E, K>(
    encapsulation: E,
) -> impl PublicKeyEncryption<Message = String, Cipher = String>
where
    E: KeyEncapsulation<Key = K> + 'static,
    E::Cipher: Serialize + Deserialize,
    K: PrivateKey,
    K::Cipher: Serialize + Deserialize,
    K::Message: Encoding + Decoding,
{
    Stringer(Hybrid(Vectorized(encapsulation)))
}

pub fn make_dyn(
    encryption: impl PublicKeyEncryption<Message = String, Cipher = String>
        + 'static,
) -> PublicEncObject {
    Box::new(DynEncryption(encryption))
}
