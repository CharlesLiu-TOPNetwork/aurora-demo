use crate::prelude::H256;

pub fn keccak(input: &[u8]) -> H256 {
    unsafe {
        super::exports::keccak256(input.len() as u64, input.as_ptr() as u64, 1);
        let bytes = H256::zero();
        super::exports::read_register(1, bytes.0.as_ptr() as *const u64 as u64);
        bytes
    }
}
