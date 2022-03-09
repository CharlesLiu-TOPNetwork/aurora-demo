#![allow(unused)]

mod interface {
    use crate::prelude::*;

    #[no_mangle]
    pub extern "C" fn deploy_code() {}
    #[no_mangle]
    pub extern "C" fn call_contract() {}
}
