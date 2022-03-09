use borsh::{BorshDeserialize, BorshSerialize};
pub enum VersionPrefix {
    V1 = 0x1,
}

#[allow(dead_code)]
#[derive(Clone, Copy, BorshSerialize, BorshDeserialize)]
pub enum KeyPrefix {
    Config = 0x0,
}

pub fn bytes_to_key(prefix: KeyPrefix, bytes: &[u8]) -> Vec<u8> {
    [&[VersionPrefix::V1 as u8], &[prefix as u8], bytes].concat()
}
