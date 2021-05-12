use self::{
    encapsulation::KeyEncapsulation,
    hybrid::HybridEncryption,
    schemes::{PrivateKey, PublicKeyEncryption},
    streaming::StreamScheme,
};

pub mod encapsulation;
pub mod hybrid;
pub mod schemes;
pub mod streaming;

pub fn hybrid_streaming<K>(
    scheme: impl KeyEncapsulation<Key = K>,
) -> impl PublicKeyEncryption
where
    K: PrivateKey,
{
    HybridEncryption::from(StreamScheme::from(scheme))
}
