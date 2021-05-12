pub trait ToBytes: Sized {
    fn to_bytes(self) -> Vec<u8>;
}

pub trait FromBytes: Sized {
    type Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>;
}

pub trait FromBytesInfallible {
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait ByteCnt {
    const BYTE_CNT: usize;
}

impl<T: ToBytes> ToBytes for Vec<T> {
    fn to_bytes(self) -> Vec<u8> {
        self.into_iter().map(T::to_bytes).flatten().collect()
    }
}

impl<T: FromBytes + ByteCnt> FromBytes for Vec<T> {
    type Error = T::Error;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        bytes.chunks(T::BYTE_CNT).map(T::from_bytes).collect()
    }
}

impl<T: FromBytesInfallible + ByteCnt> FromBytesInfallible for Vec<T> {
    fn from_bytes(bytes: &[u8]) -> Self {
        bytes.chunks(T::BYTE_CNT).map(T::from_bytes).collect()
    }
}
