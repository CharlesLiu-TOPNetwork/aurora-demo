#![allow(unused)]
use crate::error::EngineStateError;
use crate::parameters::NewCallArgs;
use crate::prelude::*;
use core::cell::RefCell;
use engine_sdk::dup_cache::{DupCache, PairDupCache};
use engine_sdk::env::Env;
use engine_sdk::io::{StorageIntermediate, IO};
use evm::backend::Basic;

#[derive(BorshSerialize, BorshDeserialize, Default, Clone)]
pub struct EngineState {
    /// Chain id, according to the EIP-155 / ethereum-lists spec.
    pub chain_id: [u8; 32],
    /// Account which can upgrade this contract.
    /// Use empty to disable updatability.
    pub owner_id: AccountId,
    /// Account of the bridge prover.
    /// Use empty to not use base token as bridged asset.
    pub bridge_prover_id: AccountId,
    /// How many blocks after staging upgrade can deploy it.
    pub upgrade_delay_blocks: u64,
}

impl From<NewCallArgs> for EngineState {
    fn from(args: NewCallArgs) -> Self {
        EngineState {
            chain_id: args.chain_id,
            owner_id: args.owner_id,
            bridge_prover_id: args.bridge_prover_id,
            upgrade_delay_blocks: args.upgrade_delay_blocks,
        }
    }
}

pub struct Engine<'env, I: IO, E: Env> {
    state: EngineState,
    origin: Address,
    gas_price: U256,
    current_account_id: AccountId,
    io: I,
    env: &'env E,
    generation_cache: RefCell<BTreeMap<Address, u32>>,
    account_info_cache: RefCell<DupCache<Address, Basic>>,
    contract_storage_cache: RefCell<PairDupCache<Address, H256, H256>>,
}

const STATE_KEY: &[u8; 5] = b"STATE";

impl<'env, I: IO + Copy, E: Env> Engine<'env, I, E> {
    pub fn new(
        origin: Address,
        current_account_id: AccountId,
        io: I,
        env: &'env E,
    ) -> Result<Self, EngineStateError> {
        get_state(&io).map(|state| Self::new_with_state(state, origin, current_account_id, io, env))
    }

    pub fn new_with_state(
        state: EngineState,
        origin: Address,
        current_account_id: AccountId,
        io: I,
        env: &'env E,
    ) -> Self {
        Self {
            state,
            origin,
            gas_price: U256::zero(),
            current_account_id,
            io,
            env,
            generation_cache: RefCell::new(BTreeMap::new()),
            account_info_cache: RefCell::new(DupCache::default()),
            contract_storage_cache: RefCell::new(PairDupCache::default()),
        }
    }
}

pub fn get_state<I: IO>(io: &I) -> Result<EngineState, EngineStateError> {
    match io.read_storage(&bytes_to_key(KeyPrefix::Config, STATE_KEY)) {
        None => Err(EngineStateError::NotFound),
        Some(bytes) => EngineState::try_from_slice(&bytes.to_vec())
            .map_err(|_| EngineStateError::DeserializationFailed),
    }
}
