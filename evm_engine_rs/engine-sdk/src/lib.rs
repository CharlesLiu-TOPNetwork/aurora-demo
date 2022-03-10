#![allow(unused)]

use crate::prelude::Address;
use crate::prelude::H256;

pub mod dup_cache;
pub mod env;
pub mod error;
pub mod io;
mod prelude;
pub mod runtime;
pub mod types;

use runtime::exports;
pub use types::keccak;

pub fn panic_utf8(bytes: &[u8]) -> ! {
    unsafe {
        exports::log_utf8(bytes.len() as u64, bytes.as_ptr() as u64);
    }
    unreachable!()
}

pub fn log_utf8(bytes: &[u8]) {
    unsafe {
        exports::log_utf8(bytes.len() as u64, bytes.as_ptr() as u64);
    }
}

pub fn log(data: &str) {
    log_utf8(data.as_bytes())
}

#[macro_export]
macro_rules! log {
    ($e: expr) => {
        #[cfg(feature = "log")]
        $crate::log($e)
    };
}
