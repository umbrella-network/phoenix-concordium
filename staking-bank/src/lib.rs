#![cfg_attr(not(feature = "std"), no_std)]

//! # Staking Bank
use concordium_std::*;
use core::fmt::Debug;

/// one = 1 * 10^18.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
const ONE: u64 = 1000000000000000000u64;

// Production constants and functions

#[cfg(feature = "production")]
const VALIDATOR_0: AccountAddress = AccountAddress([0u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_1: AccountAddress = AccountAddress([1u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_2: AccountAddress = AccountAddress([2u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_3: AccountAddress = AccountAddress([3u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_4: AccountAddress = AccountAddress([4u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_5: AccountAddress = AccountAddress([5u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_6: AccountAddress = AccountAddress([6u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_7: AccountAddress = AccountAddress([7u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_8: AccountAddress = AccountAddress([8u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_9: AccountAddress = AccountAddress([9u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_10: AccountAddress = AccountAddress([10u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_11: AccountAddress = AccountAddress([11u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_12: AccountAddress = AccountAddress([12u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_13: AccountAddress = AccountAddress([13u8; 32]);
#[cfg(feature = "production")]
const VALIDATOR_14: AccountAddress = AccountAddress([14u8; 32]);

/// The number of validators.
#[cfg(feature = "production")]
const NUMBER_OF_VALIDATORS: u8 = 15;
/// total supply = number_of_validators * ONE.
#[cfg(feature = "production")]
const TOTAL_SUPPLY: u64 = 15 * 1000000000000000000u64;

#[cfg(feature = "production")]
/// Internal function that returns a boolean if the given address is a validator.
fn _is_validator(_validator: AccountAddress) -> bool {
    _validator == VALIDATOR_0
        || _validator == VALIDATOR_1
        || _validator == VALIDATOR_2
        || _validator == VALIDATOR_3
        || _validator == VALIDATOR_4
        || _validator == VALIDATOR_5
        || _validator == VALIDATOR_6
        || _validator == VALIDATOR_7
        || _validator == VALIDATOR_8
        || _validator == VALIDATOR_9
        || _validator == VALIDATOR_10
        || _validator == VALIDATOR_11
        || _validator == VALIDATOR_12
        || _validator == VALIDATOR_13
        || _validator == VALIDATOR_14
}

#[cfg(feature = "production")]
/// Internal function that returns all validators.
fn _addresses() -> Vec<AccountAddress> {
    vec![
        VALIDATOR_0,
        VALIDATOR_1,
        VALIDATOR_2,
        VALIDATOR_3,
        VALIDATOR_4,
        VALIDATOR_5,
        VALIDATOR_6,
        VALIDATOR_7,
        VALIDATOR_8,
        VALIDATOR_9,
        VALIDATOR_10,
        VALIDATOR_11,
        VALIDATOR_12,
        VALIDATOR_13,
        VALIDATOR_14,
    ]
}

/// View function that returns validator's URL (as well as the inputted account address). The function throws an error if the address is not a validator.
#[cfg(feature = "production")]
#[receive(
    contract = "staking_bank",
    name = "validators",
    parameter = "AccountAddress",
    return_value = "(AccountAddress,String)"
)]
fn validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<(AccountAddress, String)> {
    let id: AccountAddress = ctx.parameter_cursor().get()?;

    match id {
        VALIDATOR_0 => Ok((id, "https://validator.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.umb.network".to_string())),
        VALIDATOR_2 => Ok((id, "https://umbrella.artemahr.tech".to_string())),
        VALIDATOR_3 => Ok((id, "https://umb.vtabsolutions.com:3030".to_string())),
        VALIDATOR_4 => Ok((id, "https://umb.stakers.world".to_string())),
        VALIDATOR_5 => Ok((id, "https://umbrella.crazywhale.es".to_string())),
        VALIDATOR_6 => Ok((id, "https://umbrella-node.gateomega.com".to_string())),
        VALIDATOR_7 => Ok((id, "https://umb.anorak.technology".to_string())),
        VALIDATOR_8 => Ok((id, "https://umbrella.infstones.io".to_string())),
        VALIDATOR_9 => Ok((id, "https://umb.hashquark.io".to_string())),
        VALIDATOR_10 => Ok((id, "http://umbrella.staking4all.org:3000".to_string())),
        VALIDATOR_11 => Ok((id, "https://umbrella-api.validatrium.club".to_string())),
        VALIDATOR_12 => Ok((id, "http://5.161.78.230:3000".to_string())),
        VALIDATOR_13 => Ok((id, "https://umbnode.blockchainliverpool.com".to_string())),
        VALIDATOR_14 => Ok((id, "https://umb-api.staking.rocks".to_string())),
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

// Development constants and functions

#[cfg(feature = "development")]
const VALIDATOR_0: AccountAddress = AccountAddress([0u8; 32]);
#[cfg(feature = "development")]
const VALIDATOR_1: AccountAddress = AccountAddress([1u8; 32]);

/// The number of validators.
#[cfg(feature = "development")]
const NUMBER_OF_VALIDATORS: u8 = 2;
/// total supply = number_of_validators * ONE.
#[cfg(feature = "development")]
const TOTAL_SUPPLY: u64 = 2 * 1000000000000000000u64;

#[cfg(feature = "development")]
/// Internal function that returns a boolean if the given address is a validator.
fn _is_validator(_validator: AccountAddress) -> bool {
    _validator == VALIDATOR_0 || _validator == VALIDATOR_1
}

#[cfg(feature = "development")]
/// Internal function that returns all validators.
fn _addresses() -> Vec<AccountAddress> {
    vec![VALIDATOR_0, VALIDATOR_1]
}

/// View function that returns validator's URL (as well as the inputted account address). The function throws an error if the address is not a validator.
#[cfg(feature = "development")]
#[receive(
    contract = "staking_bank",
    name = "validators",
    parameter = "AccountAddress",
    return_value = "(AccountAddress,String)"
)]
fn validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<(AccountAddress, String)> {
    let id: AccountAddress = ctx.parameter_cursor().get()?;

    match id {
        VALIDATOR_0 => Ok((id, "https://validator.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.umb.network".to_string())),
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

// Sandbox constants and functions

#[cfg(feature = "sandbox")]
const VALIDATOR_0: AccountAddress = AccountAddress([0u8; 32]);
#[cfg(feature = "sandbox")]
const VALIDATOR_1: AccountAddress = AccountAddress([1u8; 32]);

/// The number of validators.
#[cfg(feature = "sandbox")]
const NUMBER_OF_VALIDATORS: u8 = 2;
/// total supply = number_of_validators * ONE.
#[cfg(feature = "sandbox")]
const TOTAL_SUPPLY: u64 = 2 * 1000000000000000000u64;

#[cfg(feature = "sandbox")]
/// Internal function that returns a boolean if the given address is a validator.
fn _is_validator(_validator: AccountAddress) -> bool {
    _validator == VALIDATOR_0 || _validator == VALIDATOR_1
}

#[cfg(feature = "sandbox")]
/// Internal function that returns all validators.
fn _addresses() -> Vec<AccountAddress> {
    vec![VALIDATOR_0, VALIDATOR_1]
}

/// View function that returns validator's URL (as well as the inputted account address). The function throws an error if the address is not a validator.
#[cfg(feature = "sandbox")]
#[receive(
    contract = "staking_bank",
    name = "validators",
    parameter = "AccountAddress",
    return_value = "(AccountAddress,String)"
)]
fn validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<(AccountAddress, String)> {
    let id: AccountAddress = ctx.parameter_cursor().get()?;

    match id {
        VALIDATOR_0 => Ok((id, "https://validator.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.umb.network".to_string())),
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

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
    /// Failed because the validators count is not consistent.
    #[allow(dead_code)]
    ValidatorsCountMisMatch, // -5
    /// Failed because the address is not a validator.
    #[allow(dead_code)]
    NotValidator, // -6
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
    let list = _addresses();

    ensure_eq!(
        list.len(),
        NUMBER_OF_VALIDATORS as usize,
        CustomContractError::ValidatorsCountMisMatch.into()
    );

    for validator in list {
        ensure!(
            _is_validator(validator),
            CustomContractError::NotValidator.into()
        );
    }

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
    parameter = "AccountAddress",
    return_value = "u64"
)]
fn balances<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    let account: AccountAddress = ctx.parameter_cursor().get()?;

    if _is_validator(account) {
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
    parameter = "Vec<AccountAddress>",
    return_value = "bool"
)]
fn verify_validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
    let _accounts: Vec<AccountAddress> = ctx.parameter_cursor().get()?;

    for validator in _accounts {
        if !_is_validator(validator) {
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

/// View function that returns all validator addresses.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "getAddresses",
    return_value = "Vec<AccountAddress>"
)]
fn get_addresses<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<Vec<AccountAddress>> {
    Ok(_addresses())
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
    return_value = "AccountAddress"
)]
fn addresses<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<AccountAddress> {
    let index: u8 = ctx.parameter_cursor().get()?;
    Ok(_addresses()[<u8 as Into<usize>>::into(index)])
}

/// View function that returns the balance of an validator. This is to follow ERC20 interface.
#[cfg(any(feature = "production", feature = "development", feature = "sandbox"))]
#[receive(
    contract = "staking_bank",
    name = "balanceOf",
    parameter = "AccountAddress",
    return_value = "u64"
)]
fn balance_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    let _account: AccountAddress = ctx.parameter_cursor().get()?;

    if _is_validator(_account) {
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
#[receive(
    contract = "staking_bank",
    name = "getName",
    return_value = "HashSha2256",
    crypto_primitives
)]
fn get_name<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<HashSha2256> {
    let key_hash = crypto_primitives.hash_sha2_256("StakingBank".as_bytes()).0;

    Ok(HashSha2256(key_hash))
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
