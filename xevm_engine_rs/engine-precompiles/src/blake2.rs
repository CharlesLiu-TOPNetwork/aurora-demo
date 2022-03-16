use crate::{EvmPrecompileResult, Precompile, PrecompileOutput};
use engine_sdk as sdk;
use engine_types::{mem, Borrowed, types::Address, types::EthGas};
use evm::{Context, ExitError};

/// Blake2 costs.
mod costs {
    use engine_types::types::EthGas;

    /// Cost per round of Blake2 F.
    pub(super) const F_ROUND: EthGas = EthGas::new(1);
}

/// Blake2 constants.
mod consts {
    pub(super) const INPUT_LENGTH: usize = 213;
}

pub(super) struct Blake2F;

impl Blake2F {
    pub(super) const ADDRESS: Address = crate::make_address(0, 9);
}

impl Precompile for Blake2F {
    fn required_gas(input: &[u8]) -> Result<EthGas, ExitError> {
        let (int_bytes, _) = input.split_at(mem::size_of::<u32>());
        let num_rounds = u32::from_be_bytes(
            // Unwrap is fine here as it can not fail
            int_bytes.try_into().unwrap(),
        );
        Ok(num_rounds * costs::F_ROUND)
    }

    /// The compression function of the blake2 algorithm.
    ///
    /// Takes as an argument the state vector `h`, message block vector `m` (the last block is padded
    /// with zeros to full block size, if required), 2w-bit offset counter `t`, and final block
    /// indicator flag `f`. Local vector v[0..15] is used in processing. F returns a new state vector.
    /// The number of rounds, `r`, is 12 for BLAKE2b and 10 for BLAKE2s. Rounds are numbered from 0 to
    /// r - 1.
    ///
    /// See: https://eips.ethereum.org/EIPS/eip-152
    /// See: https://etherscan.io/address/0000000000000000000000000000000000000009
    fn run(
        &self,
        input: &[u8],
        target_gas: Option<EthGas>,
        _context: &Context,
        _is_static: bool,
    ) -> EvmPrecompileResult {
        if input.len() != consts::INPUT_LENGTH {
            return Err(ExitError::Other(Borrowed("ERR_BLAKE2F_INVALID_LEN")));
        }

        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }

        return Err(ExitError::OutOfGas);

        // let mut rounds_bytes = [0u8; 4];
        // rounds_bytes.copy_from_slice(&input[0..4]);
        // let rounds = u32::from_be_bytes(rounds_bytes);

        // let mut h = [0u64; 8];
        // for (mut x, value) in h.iter_mut().enumerate() {
        //     let mut word: [u8; 8] = [0u8; 8];
        //     x = x * 8 + 4;
        //     word.copy_from_slice(&input[x..(x + 8)]);
        //     *value = u64::from_le_bytes(word);
        // }

        // let mut m = [0u64; 16];
        // for (mut x, value) in m.iter_mut().enumerate() {
        //     let mut word: [u8; 8] = [0u8; 8];
        //     x = x * 8 + 68;
        //     word.copy_from_slice(&input[x..(x + 8)]);
        //     *value = u64::from_le_bytes(word);
        // }

        // let mut t: [u64; 2] = [0u64; 2];
        // for (mut x, value) in t.iter_mut().enumerate() {
        //     let mut word: [u8; 8] = [0u8; 8];
        //     x = x * 8 + 196;
        //     word.copy_from_slice(&input[x..(x + 8)]);
        //     *value = u64::from_le_bytes(word);
        // }

        // // TODO: need add blake2 algorithm!!!
        // if input[212] != 0 && input[212] != 1 {
        //     return Err(ExitError::Other(Borrowed("ERR_BLAKE2F_FINAL_FLAG")));
        // }
        // let finished = input[212] != 0;

        // let output = near_blake2::blake2b_f(rounds, h, m, t, finished).to_vec();

        // Ok(PrecompileOutput::without_logs(cost, output).into())
    }
}
