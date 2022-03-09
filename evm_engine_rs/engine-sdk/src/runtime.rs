pub(crate) mod exports {
    #[allow(unused)]
    extern "C" {

        pub(crate) fn read_register(register_id: u64, ptr: u64);
        pub(crate) fn keccak256(value_len: u64, value_ptr: u64, register_id: u64);
    }
}
