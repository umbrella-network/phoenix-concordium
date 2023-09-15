#![cfg_attr(not(feature = "std"), no_std)]

//! # Umbrella feeds
use concordium_std::*;
use core::fmt::Debug;

#[cfg(feature = "u256_amount")]
use primitive_types::U256;

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

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S: HasStateApi> {
    registry: ContractAddress,
    // key => UmbrellaFeedsReader
    readers: StateMap<HashSha2256, ContractAddress, S>,
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
    NameNotRegistered,
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
    NewUmbrellaFeedsReader(NewUmbrellaFeedsReaderEvent),
}

// TODO
// function deploy(string memory _feedName) external returns (UmbrellaFeedsReader reader) {
//     reader = deployed(_feedName);
//     IUmbrellaFeeds umbrellaFeeds = IUmbrellaFeeds(REGISTRY.getAddressByString("UmbrellaFeeds"));

//     // if UmbrellaFeeds contract is up to date, there is no need to redeploy
//     if (address(reader) != address(0) && address(reader.UMBRELLA_FEEDS()) == address(umbrellaFeeds)) {
//         return reader;
//     }

//     reader = new UmbrellaFeedsReader(REGISTRY, umbrellaFeeds, _feedName);
//     readers[hash(_feedName)] = reader;

//     emit NewUmbrellaFeedsReader(reader, _feedName);
// }

/// The NonceEvent is logged when the `permit` function is invoked. The event
/// tracks the nonce used by the signer of the `PermitMessage`.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct NewUmbrellaFeedsReaderEvent {
    /// Account that signed the `PermitMessage`.
    pub umbrella_feeds_reader: ContractAddress,
    /// The nonce that was used in the `PermitMessage`.
    pub feed_name: String,
}

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Debug, Serialize, SchemaType)]
pub struct InitContractsParamUmbrellaFeedsFactory {
    pub registry: ContractAddress,
}

/// Init function that creates a new smart contract.
#[init(
    contract = "umbrella_feeds_factory",
    parameter = "InitContractsParamUmbrellaFeedsFactory",
    event = "Event"
)]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    let param: InitContractsParamUmbrellaFeedsFactory = ctx.parameter_cursor().get()?;

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
        readers: state_builder.new_map(),
    })
}

#[receive(
    contract = "umbrella_feeds_factory",
    name = "deployed",
    parameter = "String",
    return_value = "ContractAddress",
    crypto_primitives
)]
fn deployed<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<ContractAddress> {
    let key: String = ctx.parameter_cursor().get()?;

    // Calculate the key hash.
    let key_hash = crypto_primitives.hash_sha2_256(key.as_bytes()).0;

    let reader = host
        .state()
        .readers
        .get(&HashSha2256(key_hash))
        .map(|s| *s)
        .ok_or(CustomContractError::NameNotRegistered)?;

    Ok(reader)
}

#[receive(
    contract = "umbrella_feeds_factory",
    name = "hash",
    parameter = "String",
    return_value = "HashSha2256",
    crypto_primitives
)]
fn hash<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<HashSha2256> {
    let key: String = ctx.parameter_cursor().get()?;

    // Calculate the key hash.
    let key_hash = crypto_primitives.hash_sha2_256(key.as_bytes()).0;

    Ok(HashSha2256(key_hash))
}

/// View function that returns the balance of an validator
#[receive(
    contract = "umbrella_feeds_factory",
    name = "getName",
    return_value = "HashSha2256",
    crypto_primitives
)]
fn get_name<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<HashSha2256> {
    let key_hash = crypto_primitives
        .hash_sha2_256("UmbrellaFeedsReaderFactory".as_bytes())
        .0;

    Ok(HashSha2256(key_hash))
}
