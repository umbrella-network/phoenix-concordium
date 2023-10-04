#![cfg_attr(not(feature = "std"), no_std)]

//! # Staking Bank
use concordium_std::*;
use core::fmt::Debug;

#[cfg(feature = "production")]
mod production_constants;
#[cfg(feature = "production")]
use production_constants::*;

#[cfg(feature = "development")]
mod development_constants;
#[cfg(feature = "development")]
use development_constants::*;

#[cfg(feature = "sandbox")]
mod sandbox_constants;
#[cfg(feature = "sandbox")]
use sandbox_constants::*;

/// one = 1 * 10^18.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
const ONE: u64 = 1000000000000000000u64;

#[derive(Serial, Deserial)]
pub struct State {}

/// All smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum CustomContractError {
    /// Failed to parse the parameter.
    #[from(ParseError)]
    ParseParams, // -1
    /// Failed to log because the log is full.
    LogFull, // -2
    /// Failed to log because the log is malformed.
    LogMalformed, // -3
    /// Failed to invoke a contract.
    InvokeContractError, // -4
    /// Failed because the address is not a validator.
    #[allow(dead_code)]
    NotValidator, // -5
}

/// Mapping errors related to logging to CustomContractError.
impl From<LogError> for CustomContractError {
    fn from(le: LogError) -> Self {
        match le {
            LogError::Full => Self::LogFull,
            LogError::Malformed => Self::LogMalformed,
        }
    }
}

/// Mapping errors related to contract invocations to CustomContractError.
impl<T> From<CallContractError<T>> for CustomContractError {
    fn from(_cce: CallContractError<T>) -> Self {
        Self::InvokeContractError
    }
}

/// Init function that creates a new smart contract.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[init(contract = "staking_bank")]
fn init<S: HasStateApi>(
    _ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    Ok(State {})
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `NUMBER_OF_VALIDATORS`.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "NUMBER_OF_VALIDATORS",
    return_value = "u8"
)]
fn number_of_validators<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u8> {
    Ok(NUMBER_OF_VALIDATORS)
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `TOTAL_SUPPLY`.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(contract = "staking_bank", name = "TOTAL_SUPPLY", return_value = "u64")]
fn total_supply_1<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    Ok(TOTAL_SUPPLY)
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `ONE`.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(contract = "staking_bank", name = "ONE", return_value = "u64")]
fn one<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    Ok(ONE)
}

/// View function that returns the balance of an validator.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "balances",
    parameter = "PublicKeyEd25519",
    return_value = "u64"
)]
fn balances<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    let account: PublicKeyEd25519 = ctx.parameter_cursor().get()?;

    if is_validator(account) {
        Ok(ONE)
    } else {
        Ok(0u64)
    }
}

/// View function that returns a true, if all of the provided account addresses are validators, otherwise a false.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "verifyValidators",
    parameter = "Vec<PublicKeyEd25519>",
    return_value = "bool"
)]
fn verify_validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
    let accounts: Vec<PublicKeyEd25519> = ctx.parameter_cursor().get()?;

    for validator in accounts {
        if !is_validator(validator) {
            return Ok(false);
        }
    }

    Ok(true)
}

/// View function that returns the number of validtors.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "getNumberOfValidators",
    return_value = "u8"
)]
fn get_number_of_validators<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u8> {
    Ok(NUMBER_OF_VALIDATORS)
}

/// View function that returns the balances of validators.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "getBalances",
    return_value = "Vec<u64>"
)]
fn get_balances<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<Vec<u64>> {
    let mut balances = Vec::with_capacity(NUMBER_OF_VALIDATORS as usize);
    for _i in 0..NUMBER_OF_VALIDATORS {
        balances.push(ONE)
    }

    Ok(balances)
}

/// View function that returns the address of a validator from an index.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "addresses",
    parameter = "u8",
    return_value = "PublicKeyEd25519"
)]
fn addresses_external<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<PublicKeyEd25519> {
    let index: u8 = ctx.parameter_cursor().get()?;
    Ok(addresses()[usize::from(index)])
}

/// View function that returns the balance of an validator. This is to follow ERC20 interface.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "balanceOf",
    parameter = "PublicKeyEd25519",
    return_value = "u64"
)]
fn balance_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    let account: PublicKeyEd25519 = ctx.parameter_cursor().get()?;

    if is_validator(account) {
        Ok(ONE)
    } else {
        Ok(0u64)
    }
}

/// View function that returns the total supply value. This is to follow ERC20 interface.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(contract = "staking_bank", name = "totalSupply", return_value = "u64")]
fn total_supply_2<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    Ok(TOTAL_SUPPLY)
}

/// View function that returns the key hash of this contract.
#[receive(contract = "staking_bank", name = "getName", return_value = "String")]
fn get_name<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<String> {
    Ok(String::from("StakingBank"))
}

/// The parameter type for the contract function `upgrade`.
#[derive(Debug, Serialize, SchemaType)]
pub struct UpgradeParams {
    /// The new module reference.
    pub module: ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

/// Hook function to enable `atomicUpdate` via the registry contract.
#[receive(
    contract = "staking_bank",
    name = "upgradeNatively",
    parameter = "UpgradeParams",
    error = "CustomContractError",
    low_level
)]
fn upgrade_natively<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<S>,
) -> Result<(), CustomContractError> {
    // There are no requirements atm

    Ok(())
}

/// Hook function to enable `atomicUpdate` via the registry contract.
#[receive(contract = "staking_bank", name = "unregister")]
fn unregister<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<()> {
    // There are no requirements atm

    Ok(())
}
