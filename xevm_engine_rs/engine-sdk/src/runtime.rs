use crate::io::StorageIntermediate;
use engine_types::account_id::AccountId;
use engine_types::H256;
pub struct RegisterIndex(u64);

impl StorageIntermediate for RegisterIndex {
    fn len(&self) -> usize {
        unsafe {
            let result = exports::register_len(self.0);
            // By convention, an unused register will return a length of U64::MAX
            // (see https://nomicon.io/RuntimeSpec/Components/BindingsSpec/RegistersAPI.html).
            if result < u64::MAX {
                result as usize
            } else {
                0
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn copy_to_slice(&self, buffer: &mut [u8]) {
        unsafe { exports::read_register(self.0, buffer.as_ptr() as u64) }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Runtime;

impl Runtime {
    const READ_STORAGE_REGISTER_ID: RegisterIndex = RegisterIndex(0);
    const INPUT_REGISTER_ID: RegisterIndex = RegisterIndex(1);
    const WRITE_REGISTER_ID: RegisterIndex = RegisterIndex(2);
    const EVICT_REGISTER_ID: RegisterIndex = RegisterIndex(3);
    const ENV_REGISTER_ID: RegisterIndex = RegisterIndex(4);
    // const PROMISE_REGISTER_ID: RegisterIndex = RegisterIndex(5);

    fn read_account_id() -> AccountId {
        let bytes = Self::ENV_REGISTER_ID.to_vec();
        match AccountId::try_from(bytes) {
            Ok(account_id) => account_id,
            // the environment must give us a valid Account ID.
            Err(_) => unreachable!(),
        }
    }
}

impl crate::io::IO for Runtime {
    type StorageValue = RegisterIndex;

    fn read_input(&self) -> Self::StorageValue {
        unsafe {
            exports::input(Runtime::INPUT_REGISTER_ID.0);
        }
        Runtime::INPUT_REGISTER_ID
    }

    fn return_output(&mut self, value: &[u8]) {
        unsafe {
            exports::value_return(value.len() as u64, value.as_ptr() as u64);
        }
    }

    fn read_storage(&self, key: &[u8]) -> Option<Self::StorageValue> {
        unsafe {
            if exports::storage_read(
                key.len() as u64,
                key.as_ptr() as u64,
                Runtime::READ_STORAGE_REGISTER_ID.0,
            ) == 1
            {
                Some(Runtime::READ_STORAGE_REGISTER_ID)
            } else {
                None
            }
        }
    }

    fn storage_has_key(&self, key: &[u8]) -> bool {
        unreachable!()
        // unsafe { exports::storage_has_key(key.len() as _, key.as_ptr() as _) == 1 }
    }

    fn write_storage(&mut self, key: &[u8], value: &[u8]) -> Option<Self::StorageValue> {
        unsafe {
            if exports::storage_write(
                key.len() as u64,
                key.as_ptr() as u64,
                value.len() as u64,
                value.as_ptr() as u64,
                Runtime::WRITE_REGISTER_ID.0,
            ) == 1
            {
                Some(Runtime::WRITE_REGISTER_ID)
            } else {
                None
            }
        }
    }

    fn write_storage_direct(
        &mut self,
        key: &[u8],
        value: Self::StorageValue,
    ) -> Option<Self::StorageValue> {
        unsafe {
            if exports::storage_write(
                key.len() as _,
                key.as_ptr() as _,
                u64::MAX,
                value.0,
                Runtime::WRITE_REGISTER_ID.0,
            ) == 1
            {
                Some(Runtime::WRITE_REGISTER_ID)
            } else {
                None
            }
        }
    }

    fn remove_storage(&mut self, key: &[u8]) -> Option<Self::StorageValue> {
        unsafe {
            if exports::storage_remove(
                key.len() as _,
                key.as_ptr() as _,
                Runtime::EVICT_REGISTER_ID.0,
            ) == 1
            {
                Some(Runtime::EVICT_REGISTER_ID)
            } else {
                None
            }
        }
    }
}

impl crate::env::Env for Runtime {
    fn signer_account_id(&self) -> engine_types::account_id::AccountId {
        unsafe {
            exports::signer_account_id(Self::ENV_REGISTER_ID.0);
        }
        Self::read_account_id()
    }

    // fn current_account_id(&self) -> engine_types::account_id::AccountId {
    //     unsafe {
    //         exports::current_account_id(Self::ENV_REGISTER_ID.0);
    //     }
    //     Self::read_account_id()
    // }

    fn predecessor_account_id(&self) -> engine_types::account_id::AccountId {
        unsafe {
            exports::predecessor_account_id(Self::ENV_REGISTER_ID.0);
        }
        Self::read_account_id()
    }

    fn block_height(&self) -> u64 {
        unreachable!()
        // unsafe { exports::block_index() }
    }

    fn block_timestamp(&self) -> crate::env::Timestamp {
        unreachable!()
        // let ns = unsafe { exports::block_timestamp() };
        // crate::env::Timestamp::new(ns)
    }

    fn random_seed(&self) -> engine_types::H256 {
        unsafe {
            exports::random_seed(0);
            let bytes = H256::zero();
            exports::read_register(0, bytes.0.as_ptr() as *const u64 as u64);
            bytes
        }
    }
}

pub(crate) mod exports {
    #[allow(unused)]
    extern "C" {
        // Register
        pub(crate) fn read_register(register_id: u64, ptr: u64);
        pub(crate) fn register_len(register_id: u64) -> u64;

        // Context
        // pub(crate) fn current_account_id(register_id: u64);
        pub(crate) fn signer_account_id(register_id: u64);

        pub(crate) fn predecessor_account_id(register_id: u64);
        pub(crate) fn input(register_id: u64);

        // Economics
        fn account_balance(balance_ptr: u64);

        // Math
        pub(crate) fn random_seed(register_id: u64);
        pub(crate) fn sha256(value_len: u64, value_ptr: u64, register_id: u64);
        pub(crate) fn keccak256(value_len: u64, value_ptr: u64, register_id: u64);
        pub(crate) fn ripemd160(value_len: u64, value_ptr: u64, register_id: u64);

        // Others
        pub(crate) fn value_return(value_len: u64, value_ptr: u64);
        pub(crate) fn log_utf8(len: u64, ptr: u64);

        // Storage
        pub(crate) fn storage_write(
            key_len: u64,
            key_ptr: u64,
            value_len: u64,
            value_ptr: u64,
            register_id: u64,
        ) -> u64;
        pub(crate) fn storage_read(key_len: u64, key_ptr: u64, register_id: u64) -> u64;
        pub(crate) fn storage_remove(key_len: u64, key_ptr: u64, register_id: u64) -> u64;
    }
}
