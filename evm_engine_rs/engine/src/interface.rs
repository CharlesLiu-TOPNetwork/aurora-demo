#![allow(unused)]

use crate::prelude::*;

mod interface {
    use crate::engine::Engine;
    use crate::prelude::*;
    use crate::util;
    use engine_sdk::{
        env::Env,
        io::{StorageIntermediate, IO},
        runtime::Runtime,
    };

    fn predecessor_address(predecessor_account_id: &AccountId) -> Address {
        top_account_to_evm_address(predecessor_account_id.as_bytes())
    }

    #[no_mangle]
    pub extern "C" fn deploy_code() {
        let io = Runtime;
        let input = io.read_input().to_vec();
        let current_account_id = io.current_account_id();
        let mut engine = Engine::new(
            predecessor_address(&io.predecessor_account_id()),
            current_account_id,
            io,
            &io,
        )
        .sdk_unwrap();
        Engine::deploy_code_with_input(&mut engine, input)
            .map(|res| {
                println!("res: {:?}", res);
                res.try_to_vec().sdk_expect("ERR_SERIALIZE")
            })
            .sdk_process();
        // TODO: charge for storage
    }
    #[no_mangle]
    pub extern "C" fn call_contract() {
        let io = Runtime;
        let current_account_id = io.current_account_id();
        let eth_address = "fb29cba9b146786da16733f89982f7481effb094";
        let input = util::build_input("init()", &[]);
        let args = CallArgs::V2(FunctionCallArgsV2 {
            contract: Address::new(H160::from_slice(&hex::decode(eth_address).unwrap()[..])),
            value: [0; 32],
            input,
        });
        // let bytes = io.read_input().to_vec();
        // let args = CallArgs::deserialize(&bytes).sdk_expect("ERR_BORSH_DESERIALIZE");
        // let mut ser: Vec<u8> = Vec::new();
        // args.serialize(&mut ser).unwrap();
        // let ser_len = ser.len();
        // println!("args: {:?}", ser);
        let mut engine = Engine::new(
            predecessor_address(&io.predecessor_account_id()),
            current_account_id,
            io,
            &io,
        )
        .sdk_unwrap();
        Engine::call_with_args(&mut engine, args)
            .map(|res| {
                println!("res: {:?}", res);
                res.try_to_vec().sdk_expect("ERR_SERIALIZE")
            })
            .sdk_process();
    }
}
