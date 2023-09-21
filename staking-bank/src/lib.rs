#![cfg_attr(not(feature = "std"), no_std)]

//! # Staking Bank
use concordium_std::*;
use core::fmt::Debug;

const VALIDATOR_0: AccountAddress = AccountAddress([0u8; 32]);
const VALIDATOR_1: AccountAddress = AccountAddress([1u8; 32]);

// External order is based on validators submits on AVAX in Apr 2023.
const VALIDATOR_2: AccountAddress = AccountAddress([2u8; 32]);
const VALIDATOR_3: AccountAddress = AccountAddress([3u8; 32]);
const VALIDATOR_4: AccountAddress = AccountAddress([4u8; 32]);
const VALIDATOR_5: AccountAddress = AccountAddress([5u8; 32]);
const VALIDATOR_6: AccountAddress = AccountAddress([6u8; 32]);
const VALIDATOR_7: AccountAddress = AccountAddress([7u8; 32]);
const VALIDATOR_8: AccountAddress = AccountAddress([8u8; 32]);
const VALIDATOR_9: AccountAddress = AccountAddress([9u8; 32]);
const VALIDATOR_10: AccountAddress = AccountAddress([10u8; 32]);
const VALIDATOR_11: AccountAddress = AccountAddress([11u8; 32]);
const VALIDATOR_12: AccountAddress = AccountAddress([12u8; 32]);
const VALIDATOR_13: AccountAddress = AccountAddress([13u8; 32]);
const VALIDATOR_14: AccountAddress = AccountAddress([14u8; 32]);

#[derive(Serial, Deserial, Debug, SchemaType, PartialEq, Eq)]
pub struct State {
    /// The number of validators.
    pub number_of_validators: u8,
    /// total supply = number_of_validators * one.
    pub total_supply: u64,
    /// one = 1 * 10^18.
    pub one: u64,
}

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
    ValidatorsCountMisMatch, // -5
    /// Failed because the address is not a validator.
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

/// The parameter type for the contract init function.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct InitParamsStakingBank {
    pub validators_count: u8,
}

/// Init function that creates a new smart contract.
#[init(contract = "staking_bank", parameter = "InitParamsStakingBank")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let param: InitParamsStakingBank = ctx.parameter_cursor().get()?;

    let one = 1000000000000000000u64;

    let list = _addresses();

    ensure_eq!(
        list.len(),
        param.validators_count as usize,
        CustomContractError::ValidatorsCountMisMatch.into()
    );

    for validator in list {
        ensure!(
            _is_validator(validator),
            CustomContractError::NotValidator.into()
        );
    }

    Ok(State {
        number_of_validators: param.validators_count,
        total_supply: param.validators_count as u64 * one,
        one,
    })
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `NUMBER_OF_VALIDATORS`.
#[receive(
    contract = "staking_bank",
    name = "NUMBER_OF_VALIDATORS",
    return_value = "u8"
)]
fn number_of_validators<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u8> {
    Ok(host.state().number_of_validators)
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `TOTAL_SUPPLY`.
#[receive(contract = "staking_bank", name = "TOTAL_SUPPLY", return_value = "u64")]
fn total_supply_1<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    Ok(host.state().total_supply)
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `ONE`.
#[receive(contract = "staking_bank", name = "ONE", return_value = "u64")]
fn one<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    Ok(host.state().one)
}

/// View function that returns the content of the state for debugging purposes.
#[receive(contract = "staking_bank", name = "view", return_value = "State")]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
}

/// View function that returns validator's URL (as well as the inputted account address). The function throws an error if the address is not a validator.
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

/// View function that returns the balance of an validator.
#[receive(
    contract = "staking_bank",
    name = "balances",
    parameter = "AccountAddress",
    return_value = "u64"
)]
fn balances<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    let account: AccountAddress = ctx.parameter_cursor().get()?;

    if _is_validator(account) {
        Ok(host.state().one)
    } else {
        Ok(0u64)
    }
}

/// View function that returns a boolean if an account address is an validator.
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
#[receive(
    contract = "staking_bank",
    name = "getNumberOfValidators",
    return_value = "u8"
)]
fn get_number_of_validators<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u8> {
    Ok(host.state().number_of_validators)
}

/// View function that returns all validator addresses.
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
#[receive(
    contract = "staking_bank",
    name = "getBalances",
    return_value = "Vec<u64>"
)]
fn get_balances<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<Vec<u64>> {
    let one = host.state().one;
    let number_of_validators = host.state().number_of_validators;

    let mut balances = vec![];
    for _i in 0..number_of_validators {
        balances.push(one)
    }

    Ok(balances)
}

/// View function that returns the address of a validator from an index.
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
#[receive(
    contract = "staking_bank",
    name = "balanceOf",
    parameter = "AccountAddress",
    return_value = "u64"
)]
fn balance_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    let _account: AccountAddress = ctx.parameter_cursor().get()?;

    if _is_validator(_account) {
        Ok(host.state().one)
    } else {
        Ok(0u64)
    }
}

/// View function that returns the total supply value. This is to follow ERC20 interface.
#[receive(contract = "staking_bank", name = "totalSupply", return_value = "u64")]
fn total_supply_2<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    Ok(host.state().total_supply)
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

// Hook function to enable `atomicUpdate` via the registry contract.
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

// Hook function to enable `atomicUpdate` via the registry contract.
#[receive(contract = "staking_bank", name = "unregister")]
fn unregister<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<()> {
    // There are no requirements atm

    Ok(())
}
