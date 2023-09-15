#![cfg_attr(not(feature = "std"), no_std)]

//! # Umbrella feeds
use concordium_std::*;
use core::fmt::Debug;
use registry::ImportContractsParam;

#[cfg(feature = "u256_amount")]
use primitive_types::U256;

/// The baseurl for the token metadata, gets appended with the token ID as hex
/// encoding before emitted in the TokenMetadata event.
const NAME: &str = "UmbrellaFeeds";

/// Does not exist on Concordium but kept for consistency.
const CHAIN_ID: u16 = 0;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(transparent)]
pub struct U256Wrapper(pub U256);

/// Uses the ULeb128 encoding with up to 37 bytes for the encoding as
/// according to CIS-2 specification.
impl schema::SchemaType for U256Wrapper {
    fn get_type() -> schema::Type {
        schema::Type::ULeb128(37)
    }
}

impl Serial for U256Wrapper {
    fn serial<W: Write>(&self, out: &mut W) -> Result<(), W::Err> {
        let mut value = self.0;
        loop {
            let mut byte = (value.low_u32() as u8) & 0b0111_1111;
            value >>= 7;
            if value != U256::zero() {
                byte |= 0b1000_0000;
            }
            out.write_u8(byte)?;

            if value.is_zero() {
                return Ok(());
            }
        }
    }
}

impl Deserial for U256Wrapper {
    fn deserial<R: Read>(source: &mut R) -> ParseResult<Self> {
        let mut result: U256 = U256::zero();
        for i in 0..36 {
            let byte = source.read_u8()?;
            let value_byte = <U256>::from(byte & 0b0111_1111);
            result = result
                .checked_add(value_byte << (i * 7))
                .ok_or(ParseError {})?;
            if byte & 0b1000_0000 == 0 {
                return Ok(U256Wrapper(result));
            }
        }
        let byte = source.read_u8()?;
        let value_byte = byte & 0b0111_1111;
        if value_byte & 0b1111_0000 != 0 {
            Err(ParseError {})
        } else {
            let value_byte = <U256>::from(value_byte);
            result = result
                .checked_add(value_byte << (36 * 7))
                .ok_or(ParseError {})?;
            if byte & 0b1000_0000 == 0 {
                Ok(U256Wrapper(result))
            } else {
                Err(ParseError {})
            }
        }
    }
}

#[derive(Serialize, SchemaType, Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct PriceData {
    /// @dev this is placeholder, that can be used for some additional data
    /// atm of creating this smart contract, it is only used as marker for removed data (when == type(uint8).max)
    pub data: u8,
    /// @dev heartbeat: how often price data will be refreshed in case price stay flat
    /// Using u64 instead of u24 here (different to solidity original smart contracts)
    pub heartbeat: u64,
    /// @dev timestamp: price time, at this time validators run consensus
    pub timestamp: u32,
    /// @dev price
    pub price: u128,
}

impl PriceData {
    fn default() -> PriceData {
        PriceData {
            data: 0,
            heartbeat: 0,
            timestamp: 0,
            price: 0,
        }
    }
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
    /// Failed logging: Log is full.
    LogFull, // -2
    /// Failed logging: Log is malformed.
    LogMalformed, // -3
    /// Failed to invoke a contract.
    InvokeContractError, // -4
    InvalidRequiredSignatures,             // -5
    ValidatorDoesNotExist,                 // -6
    ValidatorsCountMisMatch,               // -7
    NotValidator,                          // -8
    OverFlow,                              // -9
    NotSupportedUseUpgradeFunctionInstead, // -10
    ContractNotInitialised,                // -11
    ArraysDataDoNotMatch,                  // -12
    ChainIdMismatch,                       // -13
    OldData,                               // -14
    WrongContract,                         // -15
    Expired,                               // -16
    FeedNotExist,                          // -17
    Unauthorized,                          // -18
    /// Upgrade failed because the new module does not exist.
    FailedUpgradeMissingModule, // -19
    /// Upgrade failed because the new module does not contain a contract with a
    /// matching name.
    FailedUpgradeMissingContract, // -20
    /// Upgrade failed because the smart contract version of the module is not
    /// supported.
    FailedUpgradeUnsupportedModuleVersion, // -21
    /// Failed to verify signature because data was malformed.
    MalformedData, // -22
    /// Failed signature verification: Invalid signature.
    WrongSignature, // -23
    MissingAccount,                        // -24
    EntrypointMismatch,                    // -25
    NotEnoughSignatures,                   // -26
    SignaturesOutOfOrder,                  // -27
    InvalidSigner,                         // -28
    EmptyAddress,                          // -29
    DecimalsDoesNotMatch,                  // -30
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

/// Mapping account signature error to CustomContractError
impl From<CheckAccountSignatureError> for CustomContractError {
    fn from(e: CheckAccountSignatureError) -> Self {
        match e {
            CheckAccountSignatureError::MissingAccount => Self::MissingAccount,
            CheckAccountSignatureError::MalformedData => Self::MalformedData,
        }
    }
}

/// Mapping errors related to contract invocations to CustomContractError.
impl<T> From<CallContractError<T>> for CustomContractError {
    fn from(_cce: CallContractError<T>) -> Self {
        Self::InvokeContractError
    }
}

/// Mapping errors related to contract upgrades to CustomContractError.
impl From<UpgradeError> for CustomContractError {
    #[inline(always)]
    fn from(ue: UpgradeError) -> Self {
        match ue {
            UpgradeError::MissingModule => Self::FailedUpgradeMissingModule,
            UpgradeError::MissingContract => Self::FailedUpgradeMissingContract,
            UpgradeError::UnsupportedModuleVersion => Self::FailedUpgradeUnsupportedModuleVersion,
        }
    }
}

/// Tagged events to be serialized for the event log.
#[derive(Debug, Serial, SchemaType)]
#[concordium(repr(u8))]
enum Event {
    /// The event tracks the nonce used by the signer of the `PermitMessage`
    /// whenever the `permit` function is invoked.
    #[concordium(tag = 0)]
    LogRegistered(LogRegisteredEvent),
}

/// The NonceEvent is logged when the `permit` function is invoked. The event
/// tracks the nonce used by the signer of the `PermitMessage`.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct LogRegisteredEvent {
    /// Account that signed the `PermitMessage`.
    pub destination: ContractAddress,
    /// The nonce that was used in the `PermitMessage`.
    pub name: HashSha2256,
}

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Debug, Serialize, SchemaType)]
pub struct InitContractsParamUmbrellaFeedsReader {
    pub registry: ContractAddress,
    pub umbrella_feeds: ContractAddress,
    pub decimals: u8,
    pub key: HashSha2256,
    pub description: String,
}

/// Init function that creates a new smart contract.
#[init(
    contract = "umbrella_feeds_reader",
    parameter = "InitContractsParamUmbrellaFeedsReader",
    event = "Event"
)]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let param: InitContractsParamUmbrellaFeedsReader = ctx.parameter_cursor().get()?;

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

#[derive(SchemaType, Serial)]
pub struct SchemTypeQuinteWrapper(
    U256Wrapper,
    U256Wrapper,
    U256Wrapper,
    U256Wrapper,
    U256Wrapper,
);

/// View function that returns the balance of an validator
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
        EntrypointName::new_unchecked("getPriceDataRaw"),
        Amount::zero(),
    )?;

    let price_data: PriceData = price_data
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    // TODO

    // if (priceData.timestamp == 0) {
    //     priceData = _fallbackCall();
    // }

    // ensure!(
    //     price_data.timestamp!==
    //     0,
    //      CustomContractError::DecimalsDoesNotMatch.into()
    //  );

    Ok(SchemTypeQuinteWrapper(
        U256Wrapper(U256::from_dec_str("0").unwrap()),
        U256Wrapper(U256::from_dec_str(price_data.price.to_string().as_str()).unwrap()),
        U256Wrapper(U256::from_dec_str("0").unwrap()),
        U256Wrapper(U256::from_dec_str(price_data.timestamp.to_string().as_str()).unwrap()),
        U256Wrapper(U256::from_dec_str("0").unwrap()),
    ))
}

/// View function that returns the balance of an validator
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

    // TODO
    // if (priceData.timestamp == 0) {
    //     priceData = _fallbackCall();
    // }

    Ok(price_data)
}

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

    // TODO
    // if (priceData.timestamp == 0) {
    //     return _fallbackCallRaw();
    // }

    Ok(price_data)
}