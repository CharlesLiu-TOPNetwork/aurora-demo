use engine_types::types::{Address, EthGas};
use engine_types::{Borrowed, PhantomData, Vec};
use crate::{Byzantium, EvmPrecompileResult, HardFork, Istanbul, Precompile, PrecompileOutput};
use evm::{Context, ExitError};

/// bn128 costs.
mod costs {
    use engine_types::types::EthGas;

    /// Cost of the Byzantium alt_bn128_add operation.
    pub(super) const BYZANTIUM_ADD: EthGas = EthGas::new(500);

    /// Cost of the Byzantium alt_bn128_mul operation.
    pub(super) const BYZANTIUM_MUL: EthGas = EthGas::new(40_000);

    /// Cost of the alt_bn128_pair per point.
    pub(super) const BYZANTIUM_PAIR_PER_POINT: EthGas = EthGas::new(80_000);

    /// Cost of the alt_bn128_pair operation.
    pub(super) const BYZANTIUM_PAIR_BASE: EthGas = EthGas::new(100_000);

    /// Cost of the Istanbul alt_bn128_add operation.
    pub(super) const ISTANBUL_ADD: EthGas = EthGas::new(150);

    /// Cost of the Istanbul alt_bn128_mul operation.
    pub(super) const ISTANBUL_MUL: EthGas = EthGas::new(6_000);

    /// Cost of the Istanbul alt_bn128_pair per point.
    pub(super) const ISTANBUL_PAIR_PER_POINT: EthGas = EthGas::new(34_000);

    /// Cost of the Istanbul alt_bn128_pair operation.
    pub(super) const ISTANBUL_PAIR_BASE: EthGas = EthGas::new(45_000);
}

/// bn128 constants.
mod consts {
    /// Input length for the add operation.
    pub(super) const ADD_INPUT_LEN: usize = 128;

    /// Input length for the multiplication operation.
    pub(super) const MUL_INPUT_LEN: usize = 128;

    /// Pair element length.
    pub(super) const PAIR_ELEMENT_LEN: usize = 192;
}

/// Reads the `x` and `y` points from an input at a given position.
fn read_point(input: &[u8], pos: usize) -> Result<bn::G1, ExitError> {
    use bn::{AffineG1, Fq, Group, G1};

    let mut px_buf = [0u8; 32];
    px_buf.copy_from_slice(&input[pos..(pos + 32)]);
    let px =
        Fq::interpret(&px_buf).map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_X")))?;

    let mut py_buf = [0u8; 32];
    py_buf.copy_from_slice(&input[(pos + 32)..(pos + 64)]);
    let py =
        Fq::interpret(&py_buf).map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_Y")))?;

    Ok(if px == Fq::zero() && py == bn::Fq::zero() {
        G1::zero()
    } else {
        AffineG1::new(px, py)
            .map_err(|_| ExitError::Other(Borrowed("ERR_BN128_INVALID_POINT")))?
            .into()
    })
}

pub(super) struct Bn128Add<HF: HardFork>(PhantomData<HF>);

impl<HF: HardFork> Bn128Add<HF> {
    pub(super) const ADDRESS: Address = super::make_address(0, 6);

    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<HF: HardFork> Bn128Add<HF> {
    fn run_inner(input: &[u8], _context: &Context) -> Result<Vec<u8>, ExitError> {
        use bn::AffineG1;

        let mut input = input.to_vec();
        input.resize(consts::ADD_INPUT_LEN, 0);

        let p1 = read_point(&input, 0)?;
        let p2 = read_point(&input, 64)?;

        let mut output = [0u8; 64];
        if let Some(sum) = AffineG1::from_jacobian(p1 + p2) {
            let x = sum.x().into_u256().to_big_endian();
            let y = sum.y().into_u256().to_big_endian();
            output[0..32].copy_from_slice(&x);
            output[32..64].copy_from_slice(&y);
        }

        Ok(output.to_vec())
    }
}

impl Precompile for Bn128Add<Byzantium> {
    fn required_gas(_input: &[u8]) -> Result<EthGas, ExitError> {
        Ok(costs::BYZANTIUM_ADD)
    }

    /// Takes in two points on the elliptic curve alt_bn128 and calculates the sum
    /// of them.
    ///
    /// See: https://eips.ethereum.org/EIPS/eip-196
    /// See: https://etherscan.io/address/0000000000000000000000000000000000000006
    fn run(
        &self,
        input: &[u8],
        target_gas: Option<EthGas>,
        context: &Context,
        _is_static: bool,
    ) -> EvmPrecompileResult {
        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }

        let output = Self::run_inner(input, context)?;
        Ok(PrecompileOutput::without_logs(cost, output).into())
    }
}

impl Precompile for Bn128Add<Istanbul> {
    fn required_gas(_input: &[u8]) -> Result<EthGas, ExitError> {
        Ok(costs::ISTANBUL_ADD)
    }

    /// Takes in two points on the elliptic curve alt_bn128 and calculates the sum
    /// of them.
    ///
    /// See: https://eips.ethereum.org/EIPS/eip-196
    /// See: https://etherscan.io/address/0000000000000000000000000000000000000006
    fn run(
        &self,
        input: &[u8],
        target_gas: Option<EthGas>,
        context: &Context,
        _is_static: bool,
    ) -> EvmPrecompileResult {
        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }
        let output = Self::run_inner(input, context)?;
        Ok(PrecompileOutput::without_logs(cost, output).into())
    }
}

pub(super) struct Bn128Mul<HF: HardFork>(PhantomData<HF>);

impl<HF: HardFork> Bn128Mul<HF> {
    pub(super) const ADDRESS: Address = super::make_address(0, 7);

    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<HF: HardFork> Bn128Mul<HF> {
    fn run_inner(input: &[u8], _context: &Context) -> Result<Vec<u8>, ExitError> {
        use bn::AffineG1;

        let mut input = input.to_vec();
        input.resize(consts::MUL_INPUT_LEN, 0);

        let p = read_point(&input, 0)?;
        let mut fr_buf = [0u8; 32];
        fr_buf.copy_from_slice(&input[64..96]);
        let fr = bn::Fr::interpret(&fr_buf)
            .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_FE")))?;

        let mut output = [0u8; 64];
        if let Some(mul) = AffineG1::from_jacobian(p * fr) {
            let x = mul.x().into_u256().to_big_endian();
            let y = mul.y().into_u256().to_big_endian();
            output[0..32].copy_from_slice(&x);
            output[32..64].copy_from_slice(&y);
        }

        Ok(output.to_vec())
    }
}

impl Precompile for Bn128Mul<Byzantium> {
    fn required_gas(_input: &[u8]) -> Result<EthGas, ExitError> {
        Ok(costs::BYZANTIUM_MUL)
    }

    /// Takes in two points on the elliptic curve alt_bn128 and multiples them.
    ///
    /// See: https://eips.ethereum.org/EIPS/eip-196
    /// See: https://etherscan.io/address/0000000000000000000000000000000000000007
    fn run(
        &self,
        input: &[u8],
        target_gas: Option<EthGas>,
        context: &Context,
        _is_static: bool,
    ) -> EvmPrecompileResult {
        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }

        let output = Self::run_inner(input, context)?;
        Ok(PrecompileOutput::without_logs(cost, output).into())
    }
}

impl Precompile for Bn128Mul<Istanbul> {
    fn required_gas(_input: &[u8]) -> Result<EthGas, ExitError> {
        Ok(costs::ISTANBUL_MUL)
    }

    /// Takes in two points on the elliptic curve alt_bn128 and multiples them.
    ///
    /// See: https://eips.ethereum.org/EIPS/eip-196
    /// See: https://etherscan.io/address/0000000000000000000000000000000000000007
    fn run(
        &self,
        input: &[u8],
        target_gas: Option<EthGas>,
        context: &Context,
        _is_static: bool,
    ) -> EvmPrecompileResult {
        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }

        let output = Self::run_inner(input, context)?;
        Ok(PrecompileOutput::without_logs(cost, output).into())
    }
}

pub(super) struct Bn128Pair<HF: HardFork>(PhantomData<HF>);

impl<HF: HardFork> Bn128Pair<HF> {
    pub(super) const ADDRESS: Address = super::make_address(0, 8);

    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl<HF: HardFork> Bn128Pair<HF> {
    fn run_inner(input: &[u8], _context: &Context) -> Result<Vec<u8>, ExitError> {
        use bn::{arith::U256, AffineG1, AffineG2, Fq, Fq2, Group, Gt, G1, G2};

        if input.len() % consts::PAIR_ELEMENT_LEN != 0 {
            return Err(ExitError::Other(Borrowed("ERR_BN128_INVALID_LEN")));
        }

        let output = if input.is_empty() {
            U256::one()
        } else {
            let elements = input.len() / consts::PAIR_ELEMENT_LEN;
            let mut vals = Vec::with_capacity(elements);

            for idx in 0..elements {
                let mut buf = [0u8; 32];

                buf.copy_from_slice(
                    &input[(idx * consts::PAIR_ELEMENT_LEN)..(idx * consts::PAIR_ELEMENT_LEN + 32)],
                );
                let ax = Fq::interpret(&buf)
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_AX")))?;
                buf.copy_from_slice(
                    &input[(idx * consts::PAIR_ELEMENT_LEN + 32)
                        ..(idx * consts::PAIR_ELEMENT_LEN + 64)],
                );
                let ay = Fq::interpret(&buf)
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_AY")))?;
                buf.copy_from_slice(
                    &input[(idx * consts::PAIR_ELEMENT_LEN + 64)
                        ..(idx * consts::PAIR_ELEMENT_LEN + 96)],
                );
                let bay = Fq::interpret(&buf)
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_B_AY")))?;
                buf.copy_from_slice(
                    &input[(idx * consts::PAIR_ELEMENT_LEN + 96)
                        ..(idx * consts::PAIR_ELEMENT_LEN + 128)],
                );
                let bax = Fq::interpret(&buf)
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_B_AX")))?;
                buf.copy_from_slice(
                    &input[(idx * consts::PAIR_ELEMENT_LEN + 128)
                        ..(idx * consts::PAIR_ELEMENT_LEN + 160)],
                );
                let bby = Fq::interpret(&buf)
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_B_BY")))?;
                buf.copy_from_slice(
                    &input[(idx * consts::PAIR_ELEMENT_LEN + 160)
                        ..(idx * consts::PAIR_ELEMENT_LEN + 192)],
                );
                let bbx = Fq::interpret(&buf)
                    .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_B_BX")))?;

                let a = {
                    if ax.is_zero() && ay.is_zero() {
                        G1::zero()
                    } else {
                        G1::from(
                            AffineG1::new(ax, ay)
                                .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_A")))?,
                        )
                    }
                };
                let b = {
                    let ba = Fq2::new(bax, bay);
                    let bb = Fq2::new(bbx, bby);

                    if ba.is_zero() && bb.is_zero() {
                        G2::zero()
                    } else {
                        G2::from(
                            AffineG2::new(ba, bb)
                                .map_err(|_e| ExitError::Other(Borrowed("ERR_BN128_INVALID_B")))?,
                        )
                    }
                };
                vals.push((a, b))
            }

            let mul = vals
                .into_iter()
                .fold(Gt::one(), |s, (a, b)| s * bn::pairing(a, b));

            if mul == Gt::one() {
                U256::one()
            } else {
                U256::zero()
            }
        };

        Ok(output.to_big_endian().to_vec())
    }
}

impl Precompile for Bn128Pair<Byzantium> {
    fn required_gas(input: &[u8]) -> Result<EthGas, ExitError> {
        Ok(
            costs::BYZANTIUM_PAIR_PER_POINT * input.len() / consts::PAIR_ELEMENT_LEN
                + costs::BYZANTIUM_PAIR_BASE,
        )
    }

    /// Takes in elements and calculates the pair.
    ///
    /// See: https://eips.ethereum.org/EIPS/eip-197
    /// See: https://etherscan.io/address/0000000000000000000000000000000000000008
    fn run(
        &self,
        input: &[u8],
        target_gas: Option<EthGas>,
        context: &Context,
        _is_static: bool,
    ) -> EvmPrecompileResult {
        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }

        let output = Self::run_inner(input, context)?;
        Ok(PrecompileOutput::without_logs(cost, output).into())
    }
}

impl Precompile for Bn128Pair<Istanbul> {
    fn required_gas(input: &[u8]) -> Result<EthGas, ExitError> {
        Ok(
            costs::ISTANBUL_PAIR_PER_POINT * input.len() / consts::PAIR_ELEMENT_LEN
                + costs::ISTANBUL_PAIR_BASE,
        )
    }

    /// Takes in elements and calculates the pair.
    ///
    /// See: https://eips.ethereum.org/EIPS/eip-197
    /// See: https://etherscan.io/address/0000000000000000000000000000000000000008
    fn run(
        &self,
        input: &[u8],
        target_gas: Option<EthGas>,
        context: &Context,
        _is_static: bool,
    ) -> EvmPrecompileResult {
        let cost = Self::required_gas(input)?;
        if let Some(target_gas) = target_gas {
            if cost > target_gas {
                return Err(ExitError::OutOfGas);
            }
        }

        let output = Self::run_inner(input, context)?;
        Ok(PrecompileOutput::without_logs(cost, output).into())
    }
}

