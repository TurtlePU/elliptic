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

pub fn hybrid_streaming<K, M, C1, C2>(
    scheme: impl KeyEncapsulation<C1, Key = K>,
) -> impl PublicKeyEncryption<Vec<M>, (C1, Vec<C2>)>
where
    K: PrivateKey<M, C2>,
{
    HybridEncryption::from(StreamScheme::from(scheme))
}
