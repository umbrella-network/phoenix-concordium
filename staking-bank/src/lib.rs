#![cfg_attr(not(feature = "std"), no_std)]

//! # Staking Bank
use concordium_std::*;
use core::fmt::Debug;

const VALIDATOR_0: AccountAddress = AccountAddress([0u8; 32]);
const VALIDATOR_1: AccountAddress = AccountAddress([1u8; 32]);

// external order is based on validators submits on AVAX for Apr 2023
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

#[derive(Serial, Deserial, Debug, SchemaType)]
struct State {
    number_of_validators: u8,
    total_supply: u64,
    one: u64,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum CustomContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams, // -1
    /// Failed logging: Log is full.
    LogFull, // -2
    /// Failed logging: Log is malformed.
    LogMalformed, // -3
    /// Failed to invoke a contract.
    InvokeContractError, // -4
    ValidatorDoesNotExist,   // -5
    ValidatorsCountMisMatch, // -6
    NotValidator,            // -7
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

/// Get _isValidator
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

/// Get _addresses
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

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct InitContractsParamStakingBank {
    pub validators_count: u8,
}

/// Init function that creates a new smart contract.
#[init(contract = "staking_bank", parameter = "InitContractsParamStakingBank")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let param: InitContractsParamStakingBank = ctx.parameter_cursor().get()?;

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

/// View function that returns validators
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
        _ => bail!(CustomContractError::ValidatorDoesNotExist.into()),
    }
}

/// View function that returns the balance of an validator
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
    let _account: AccountAddress = ctx.parameter_cursor().get()?;

    if _is_validator(_account) {
        Ok(host.state().one)
    } else {
        Ok(0u64)
    }
}

/// View function that returns the balance of an validator
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

    let mut return_value = true;

    for _validator in _accounts {
        if !_is_validator(_validator) {
            return_value = false;
        }
    }

    Ok(return_value)
}

/// View function that returns the balance of an validator
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

/// View function that returns the balance of an validator
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

/// View function that returns the balance of an validator
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

/// View function that returns the balance of an validator
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

/// View function that returns the balance of an validator
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

/// View function that returns the balance of an validator
#[receive(contract = "staking_bank", name = "totalSupply", return_value = "u64")]
fn total_supply_2<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u64> {
    Ok(host.state().total_supply)
}

/// View function that returns the balance of an validator
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

/// View function that returns the balance of an validator
#[receive(contract = "staking_bank", name = "register")]
fn register<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<()> {
    // there are no requirements atm
    Ok(())
}

/// View function that returns the balance of an validator
#[receive(contract = "staking_bank", name = "unregister")]
fn unregister<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<()> {
    // there are no requirements atm
    Ok(())
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "_addresses",
    return_value = "Vec<AccountAddress>"
)]
fn addresses_external<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<Vec<AccountAddress>> {
    Ok(_addresses())
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "_isValidator",
    parameter = "AccountAddress",
    return_value = "bool"
)]
fn is_validator<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
    let _account: AccountAddress = ctx.parameter_cursor().get()?;

    Ok(_is_validator(_account))
}

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    #[concordium_test]
    /// Test that initializing the contract succeeds with some state.
    fn test_init() {
        let mut ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        let parameter_bytes = to_bytes(&InitContractsParamStakingBank {
            validators_count: 15u8,
        });
        ctx.set_parameter(&parameter_bytes);

        let state_result = init(&ctx, &mut state_builder);
        let initial_state = state_result.expect_report("Contract initialization results in error");

        let ctx = TestReceiveContext::empty();

        let host = TestHost::new(initial_state, state_builder);

        // Call the contract function.
        let state = view(&ctx, &host);

        println!("{:?}", state.unwrap());
    }
}
