#![cfg_attr(not(feature = "std"), no_std)]

//! # Registry
use concordium_std::*;
use core::fmt::Debug;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S: HasStateApi> {
    owner: Address,
    // name => contract address
    registry: StateMap<HashSha2256, ContractAddress, S>,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum CustomContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// Failed logging: Log is full.
    LogFull,
    /// Failed logging: Log is malformed.
    LogMalformed,
    NameNotRegistered,
}

/// Mapping the logging errors to Error.
impl From<LogError> for CustomContractError {
    fn from(le: LogError) -> Self {
        match le {
            LogError::Full => Self::LogFull,
            LogError::Malformed => Self::LogMalformed,
        }
    }
}

/// Init function that creates a new smart contract.
#[init(contract = "registry")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    Ok(State {
        registry: state_builder.new_map(),
        owner: Address::from(ctx.init_origin()),
    })
}

/// View function that returns contract_address from key hash.
#[receive(
    contract = "registry",
    name = "getAddress",
    parameter = "HashSha2256",
    return_value = "ContractAddress"
)]
fn get_address<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<ContractAddress> {
    let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

    let contract_address = host
        .state()
        .registry
        .get(&key_hash)
        .map(|s| *s)
        .ok_or(CustomContractError::NameNotRegistered)?;

    Ok(contract_address)
}

/// View function that returns contract_address from key string.
#[receive(
    contract = "registry",
    name = "getAddressByString",
    parameter = "String",
    return_value = "ContractAddress",
    crypto_primitives
)]
fn get_address_by_string<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<ContractAddress> {
    let key: String = ctx.parameter_cursor().get()?;

    // Calculate the message hash.
    let key_hash = crypto_primitives.hash_sha2_256(key.as_bytes()).0;

    let contract_address = host
        .state()
        .registry
        .get(&HashSha2256(key_hash))
        .map(|s| *s)
        .ok_or(CustomContractError::NameNotRegistered)?;

    Ok(contract_address)
}

/// View function that hash from a key string.
#[receive(
    contract = "registry",
    name = "stringToBytes32",
    parameter = "String",
    return_value = "HashSha2256",
    crypto_primitives
)]
fn string_to_bytes32<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<HashSha2256> {
    let key: String = ctx.parameter_cursor().get()?;

    // Calculate the message hash.
    let key_hash = crypto_primitives.hash_sha2_256(key.as_bytes()).0;

    Ok(HashSha2256(key_hash))
}

// #[concordium_cfg_test]
// mod tests {
//     use super::*;
//     use test_infrastructure::*;

//     type ContractResult<A> = Result<A, Error>;

//     #[concordium_test]
//     /// Test that initializing the contract succeeds with some state.
//     fn test_init() {
//         let ctx = TestInitContext::empty();

//         let mut state_builder = TestStateBuilder::new();

//         let state_result = init(&ctx, &mut state_builder);
//         state_result.expect_report("Contract initialization results in error");
//     }

//     #[concordium_test]
//     /// Test that invoking the `receive` endpoint with the `false` parameter
//     /// succeeds in updating the contract.
//     fn test_throw_no_error() {
//         let ctx = TestInitContext::empty();

//         let mut state_builder = TestStateBuilder::new();

//         // Initializing state
//         let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

//         let mut ctx = TestReceiveContext::empty();

//         let throw_error = false;
//         let parameter_bytes = to_bytes(&throw_error);
//         ctx.set_parameter(&parameter_bytes);

//         let mut host = TestHost::new(initial_state, state_builder);

//         // Call the contract function.
//         let result: ContractResult<()> = receive(&ctx, &mut host);

//         // Check the result.
//         claim!(result.is_ok(), "Results in rejection");
//     }

//     #[concordium_test]
//     /// Test that invoking the `receive` endpoint with the `true` parameter
//     /// results in the `YourError` being thrown.
//     fn test_throw_error() {
//         let ctx = TestInitContext::empty();

//         let mut state_builder = TestStateBuilder::new();

//         // Initializing state
//         let initial_state = init(&ctx, &mut state_builder).expect("Initialization should pass");

//         let mut ctx = TestReceiveContext::empty();

//         let throw_error = true;
//         let parameter_bytes = to_bytes(&throw_error);
//         ctx.set_parameter(&parameter_bytes);

//         let mut host = TestHost::new(initial_state, state_builder);

//         // Call the contract function.
//         let error: ContractResult<()> = receive(&ctx, &mut host);

//         // Check the result.
//         claim_eq!(
//             error,
//             Err(Error::YourError),
//             "Function should throw an error."
//         );
//     }
// }
