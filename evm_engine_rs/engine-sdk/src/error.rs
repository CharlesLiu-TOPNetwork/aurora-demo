#[derive(Debug)]
pub struct BorshDeserializeError;

impl AsRef<[u8]> for BorshDeserializeError {
    fn as_ref(&self) -> &[u8] {
        b"ERR_ARG_PARSE"
    }
}
