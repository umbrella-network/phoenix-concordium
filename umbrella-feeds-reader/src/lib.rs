#![cfg_attr(not(feature = "std"), no_std)]

//! # Umbrella feeds
use concordium_std::*;
use core::fmt::Debug;

#[derive(Serialize, SchemaType, Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct PriceData {
    /// @dev this is placeholder, that can be used for some additional data
    /// atm of creating this smart contract, it is only used as marker for removed data (when == type(uint8).max)
    pub data: u8,
    /// @dev heartbeat: how often price data will be refreshed in case price stay flat
    /// Using u64 instead of u24 here (different to solidity original smart contracts)
    pub heartbeat: u64,
    /// @dev timestamp: price time, at this time validators run consensus
    pub timestamp: Timestamp,
    /// @dev price
    pub price: u128,
}

#[derive(Serial, Deserial, Debug, SchemaType)]
struct State {
    registry: ContractAddress,
    umbrella_feeds: ContractAddress,
    key: HashSha2256,
    description: String,
    decimals: u8,
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum CustomContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams, // -1
    /// Failed to invoke a contract.
    InvokeContractError, // -2
    EmptyAddress,         // -3
    DecimalsDoesNotMatch, // -4
}

/// Mapping errors related to contract invocations to CustomContractError.
impl<T> From<CallContractError<T>> for CustomContractError {
    fn from(_cce: CallContractError<T>) -> Self {
        Self::InvokeContractError
    }
}

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Debug, Serialize, SchemaType)]
pub struct InitParamsUmbrellaFeedsReader {
    pub registry: ContractAddress,
    pub umbrella_feeds: ContractAddress,
    pub decimals: u8,
    pub key: HashSha2256,
    pub description: String,
}

/// Init function that creates a new smart contract.
#[init(
    contract = "umbrella_feeds_reader",
    parameter = "InitParamsUmbrellaFeedsReader"
)]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
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

    Ok(State {
        registry: param.registry,
        decimals: param.decimals,
        umbrella_feeds: param.umbrella_feeds,
        key: param.key,
        description: param.description,
    })
}

/// View function that returns the balance of an validator
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

/// View function that returns the balance of an validator
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
pub struct SchemTypeQuinteWrapper(pub u8, pub u128, pub u8, pub Timestamp, pub u8);

// View function that returns the balance of an validator
#[receive(
    contract = "umbrella_feeds_reader",
    name = "latestRoundData",
    return_value = "SchemTypeQuinteWrapper"
)]
fn latest_round_data<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<SchemTypeQuinteWrapper> {
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

    Ok(SchemTypeQuinteWrapper(
        0u8,
        price_data.price,
        0u8,
        price_data.timestamp,
        0u8,
    ))
}

/// The `getPriceData` and the `getPriceDataRaw` have the same logic on Concordium since the native upgrade mechanism on Concordium allows to upgrade of the `UmbrellaFeeds` contract without changing its contract address.
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
        EntrypointName::new_unchecked("prices"),
        Amount::zero(),
    )?;

    let price_data: PriceData = price_data
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    Ok(price_data)
}

/// The `getPriceData` and the `getPriceDataRaw` have the same logic on Concordium since the native upgrade mechanism on Concordium allows to upgrade of the `UmbrellaFeeds` contract without changing its contract address.
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
