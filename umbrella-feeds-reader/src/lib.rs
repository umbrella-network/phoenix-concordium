#![cfg_attr(not(feature = "std"), no_std)]

//! # Umbrella feeds reader
//!
//! This is an optional price reader for just one feed.
//! For maximum gas optimization it is recommended to use UmbrellaFeeds directly.
//!
//! This contract has a hard-coded `umbrella_feed` contract address.
//! ATTENTION: `Umbrella_feed` contract should only be upgraded via the `atomicUpdate` entry point in the registry contract to not break this link.
use concordium_std::*;
use core::fmt::Debug;

#[derive(Serialize, SchemaType, Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct PriceData {
    /// This is a placeholder, that can be used for some additional data.
    /// It is only used as marker for removed data (when data == u8::MAX) at the moment.
    pub data: u8,
    /// The heartbeat specifies how often the price data will be refreshed in case the price stays flat.
    /// ATTENTION: u64 is used here instead of u24 (different from the original solidity smart contracts).
    pub heartbeat: u64,
    /// It is the time the validators run consensus to decide on the price data.
    pub timestamp: Timestamp,
    /// The price.
    pub price: u128,
}

#[derive(Serial, Deserial, Debug, SchemaType)]
struct State {
    /// Registry contract where the list of all addresses of this protocol is stored.
    registry: ContractAddress,
    /// Umbrella_feeds contract where price data is stored.
    umbrella_feeds: ContractAddress,
    /// The key for the feed name represented by this contract. E.g. for the "ETH-USD" feed, the key will be the hash("ETH-USD").
    key: HashSha2256,
    /// Description of this feed name.
    description: String,
    /// Decimals for prices stored in the umbrella_feeds contract.
    decimals: u8,
}

/// All smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum CustomContractError {
    /// Failed to parse the parameter.
    #[from(ParseError)]
    ParseParams, // -1
    /// Failed to invoke a contract.
    InvokeContractError, // -2
    /// Failed because the address(0x0) is not valid.
    EmptyAddress, // -3
    /// Failed because the decimal value in this contract and the decimal value in the umbrella_feeds contract do not match.
    DecimalsDoesNotMatch, // -4
}

/// Mapping errors related to contract invocations to CustomContractError.
impl<T> From<CallContractError<T>> for CustomContractError {
    fn from(_cce: CallContractError<T>) -> Self {
        Self::InvokeContractError
    }
}

/// The parameter type for the contract init function.
#[derive(Debug, Serialize, SchemaType)]
pub struct InitParamsUmbrellaFeedsReader {
    pub registry: ContractAddress,
    pub umbrella_feeds: ContractAddress,
    pub decimals: u8,
    pub description: String,
}

/// Init function that creates a new smart contract. The `checkSetUp` entry point should be called after creating a new smart contract instance for a sanity check.
#[init(
    contract = "umbrella_feeds_reader",
    parameter = "InitParamsUmbrellaFeedsReader",
    crypto_primitives
)]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> InitResult<State> {
    let param: InitParamsUmbrellaFeedsReader = ctx.parameter_cursor().get()?;

    ensure!(
        param.registry
            != ContractAddress {
                index: 0,
                subindex: 0
            },
        CustomContractError::EmptyAddress.into()
    );

    let key_hash = crypto_primitives
        .hash_sha2_256(param.description.as_bytes())
        .0;

    Ok(State {
        registry: param.registry,
        decimals: param.decimals,
        umbrella_feeds: param.umbrella_feeds,
        key: HashSha2256(key_hash),
        description: param.description,
    })
}

/// View function to do a sanity check.
#[receive(
    contract = "umbrella_feeds_reader",
    name = "checkSetUp",
    return_value = "bool"
)]
fn check_set_up<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
    let decimals = host.invoke_contract_read_only(
        &host.state().umbrella_feeds,
        &Parameter::empty(),
        EntrypointName::new_unchecked("DECIMALS"),
        Amount::zero(),
    )?;

    let decimals: u8 = decimals
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    ensure_eq!(
        decimals,
        host.state().decimals,
        CustomContractError::DecimalsDoesNotMatch.into()
    );

    let hash: HashSha2256 = host.state().key;

    let price_data = host.invoke_contract_read_only::<HashSha2256>(
        &host.state().umbrella_feeds,
        &hash,
        EntrypointName::new_unchecked("getPriceData"),
        Amount::zero(),
    )?;

    let _price_data: PriceData = price_data
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    Ok(true)
}

/// View function that returns the content of the state for debugging purposes.
#[receive(
    contract = "umbrella_feeds_reader",
    name = "view",
    return_value = "State"
)]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
}

/// View function that returns the decimal value.
#[receive(
    contract = "umbrella_feeds_reader",
    name = "DECIMALS",
    return_value = "u8"
)]
fn decimals<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<u8> {
    Ok(host.state().decimals)
}

#[derive(SchemaType, Serial, Deserial, Debug, PartialEq, Eq)]
pub struct SchemaTypeQuintWrapper(pub u8, pub u128, pub u8, pub Timestamp, pub u8);

/// This entry point was inspired by the chainlink interface for easy migration. NOTE: not all returned data fields are covered.
/// This entry point throws an exception when there is no data, instead of returning unset values, which could be misinterpreted as actual reported values.
/// It does NOT throw when data is outdated (based on heartbeat and timestamp).
#[receive(
    contract = "umbrella_feeds_reader",
    name = "latestRoundData",
    return_value = "SchemaTypeQuintWrapper"
)]
fn latest_round_data<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<SchemaTypeQuintWrapper> {
    let hash: HashSha2256 = host.state().key;

    let price_data = host.invoke_contract_read_only::<HashSha2256>(
        &host.state().umbrella_feeds,
        &hash,
        EntrypointName::new_unchecked("getPriceData"),
        Amount::zero(),
    )?;

    let price_data: PriceData = price_data
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    Ok(SchemaTypeQuintWrapper(
        0u8,
        price_data.price,
        0u8,
        price_data.timestamp,
        0u8,
    ))
}

/// This is main endpoint for reading the feed. The feed is read from the umbrella_feeds contract using the hardcoded `key` in this contract.
/// In case the feed does not exist, this entry point throws.s
/// There is no fallback function since the native upgrade mechanism on Concordium allows to upgrade of the `UmbrellaFeeds` contract without changing its contract address.
#[receive(
    contract = "umbrella_feeds_reader",
    name = "getPriceData",
    return_value = "PriceData"
)]
fn get_price_data<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<PriceData> {
    let hash: HashSha2256 = host.state().key;

    let price_data = host.invoke_contract_read_only::<HashSha2256>(
        &host.state().umbrella_feeds,
        &hash,
        EntrypointName::new_unchecked("getPriceData"),
        Amount::zero(),
    )?;

    let price_data: PriceData = price_data
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    Ok(price_data)
}

/// This is the raw entry point for reading the feed. The feed is read from the umbrella_feeds contract using the hardcoded `key` in this contract.
/// In case the feed does not exist, this entry point does NOT throw an error but instead returns a default price value (similar to solidity contracts).
/// There is no fallback function since the native upgrade mechanism on Concordium allows to upgrade of the `UmbrellaFeeds` contract without changing its contract address.
#[receive(
    contract = "umbrella_feeds_reader",
    name = "getPriceDataRaw",
    return_value = "PriceData"
)]
fn get_price_data_raw<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<PriceData> {
    let hash: HashSha2256 = host.state().key;

    let price_data = host.invoke_contract_read_only::<HashSha2256>(
        &host.state().umbrella_feeds,
        &hash,
        EntrypointName::new_unchecked("prices"),
        Amount::zero(),
    )?;

    let price_data: PriceData = price_data
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    Ok(price_data)
}
