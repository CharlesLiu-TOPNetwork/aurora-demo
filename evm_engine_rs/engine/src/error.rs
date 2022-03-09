#[derive(Debug)]
pub enum EngineStateError {
    NotFound,
    DeserializationFailed,
}

impl AsRef<[u8]> for EngineStateError {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::NotFound => b"ERR_STATE_NOT_FOUND",
            Self::DeserializationFailed => b"ERR_STATE_CORRUPTED",
        }
    }
}
