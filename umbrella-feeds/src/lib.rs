#![cfg_attr(not(feature = "std"), no_std)]

//! # Umbrella feeds
//!
//! Main contract for all on-chain data.
//! Check `UmbrellaFeedsReader` to see how to integrate.
//!
//! ATTENTION: Keep the `upgradeNatively`/`unregister` entry points in this contract at all times and make sure their logic can be
//! executed successfully via an invoke to the `atomicUpdate` entry point in the `registry` contract. Otherwise, you will not be able to
//! natively upgrade this contract via the `registry` contract anymore.
//! Using the native upgradability mechanism for this contract is necessary to not break the `UmbrellaFeedsReader` contracts
//! which include references to this `UmbrellaFeeds` contract.
use concordium_std::*;
use core::fmt::Debug;

/// The name of this contract.
const NAME: &str = "UmbrellaFeeds";

/// The concept of a CHAIN_ID does not exist on Concordium but kept for consistency.
const CHAIN_ID: u16 = 49228;

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

impl PriceData {
    fn default() -> PriceData {
        PriceData {
            data: 0,
            heartbeat: 0,
            timestamp: Timestamp::from_timestamp_millis(0),
            price: 0,
        }
    }
}
#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S> {
    /// Contract deployment time.
    deployed_at: Timestamp,
    /// Registry contract where the list of all addresses of this protocol is stored.
    registry: ContractAddress,
    /// StakingBank contract where list of validators is stored.
    staking_bank: ContractAddress,
    /// Minimal number of signatures required for accepting price submission (PoA).
    required_signatures: u16,
    /// Decimals for prices stored in this contract.
    decimals: u8,
    /// Map of all prices stored in this contract. It maps from the key to PriceData. The key for the map is the hash of the feed name.
    /// E.g. for the "ETH-USD" feed, the key will be the hash("ETH-USD").
    prices: StateMap<HashSha2256, PriceData, S>,
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
    /// Failed to provide enough signatures.
    InvalidRequiredSignatures, // -5
    /// Failed because the destroy function is not supported.
    NotSupported, // -6
    /// Failed to because the chain id is wrong.
    ChainIdMismatch, // -7
    /// Failed to because the data is outdated.
    OldData, // -8
    /// Failed because it is the wrong contract.
    WrongContract, // -9
    /// Failed because the signature is outdated.
    Expired, // -10
    /// Failed because the feed does not exist.
    FeedNotExist, // -11
    /// Failed because of unauthorized invoke of the entry point.
    Unauthorized, // -12
    /// Upgrade failed because the new module does not exist.
    FailedUpgradeMissingModule, // -13
    /// Upgrade failed because the new module does not contain a contract with a
    /// matching name.
    FailedUpgradeMissingContract, // -14
    /// Upgrade failed because the smart contract version of the module is not
    /// supported.
    FailedUpgradeUnsupportedModuleVersion, // -15
    /// Failed to verify signature because data was malformed.
    MalformedData, // -16
    /// Failed signature verification because of an invalid signature.
    WrongSignature, // -17
    /// Failed because the account is missing on the chain.
    MissingAccount, // -18
    /// Failed because not enough signatures were provided.
    NotEnoughSignatures, // -19
    /// Failed because the signatures are not in order.
    SignaturesOutOfOrder, // -20
    /// Failed because one of the given signers is not a validator.
    InvalidSigner, // -21
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

/// The parameter type for the contract init function.
#[derive(Debug, Serialize, SchemaType)]
pub struct InitParamsUmbrellaFeeds {
    pub registry: ContractAddress,
    pub required_signatures: u16,
    pub staking_bank: ContractAddress,
    pub decimals: u8,
}

/// Init function that creates a new smart contract.
#[init(contract = "umbrella_feeds", parameter = "InitParamsUmbrellaFeeds")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    let param: InitParamsUmbrellaFeeds = ctx.parameter_cursor().get()?;

    ensure!(
        param.required_signatures != 0,
        CustomContractError::InvalidRequiredSignatures.into()
    );

    Ok(State {
        deployed_at: ctx.metadata().block_time(),
        registry: param.registry,
        staking_bank: param.staking_bank,
        required_signatures: param.required_signatures,
        decimals: param.decimals,
        prices: state_builder.new_map(),
    })
}

/// The parameter type for the contract function `upgradeNatively`.
/// Takes the new module and optionally an entrypoint to call in the new module
/// after triggering the upgrade. The upgrade is reverted if the entrypoint invoke
/// fails. This is useful for doing migration in the same transaction triggering
/// the upgrade.
#[derive(Debug, Serialize, SchemaType)]
pub struct UpgradeParams {
    /// The new module reference.
    pub module: ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

/// This function is a hook function intended to be invoked via the `atomicUpdate` function in the registry contract.
/// This function upgrades this smart contract instance to a new module and calls optionally a
/// migration function after the upgrade.
///
/// It rejects if:
/// - Sender is not the registry contract instance.
/// - It fails to parse the parameter.
/// - If the upgrade fails.
/// - If the migration invoke fails.
///
/// This function is marked as `low_level`. This is **necessary** since the
/// high-level mutable functions store the state of the contract at the end of
/// execution. This conflicts with migration since the shape of the state
/// **might** be changed by the migration function. If the state is then written
/// by this function it would overwrite the state stored by the migration
/// function.
#[receive(
    contract = "umbrella_feeds",
    name = "upgradeNatively",
    parameter = "UpgradeParams",
    error = "CustomContractError",
    low_level
)]
fn upgrade_natively<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<S>,
) -> Result<(), CustomContractError> {
    // Read the top-level contract state.
    let state: State<S> = host.state().read_root()?;

    // Only the registry can upgrade this contract.
    ensure_eq!(
        ctx.sender(),
        Address::Contract(state.registry),
        CustomContractError::Unauthorized
    );

    // Parse the parameter.
    let param: UpgradeParams = ctx.parameter_cursor().get()?;

    // Trigger the upgrade.
    host.upgrade(param.module)?;

    // Call the migration function if provided.
    if let Some((func, parameters)) = param.migrate {
        host.invoke_contract_raw(
            &ctx.self_address(),
            parameters.as_parameter(),
            func.as_entrypoint_name(),
            Amount::zero(),
        )?;
    }

    Ok(())
}

#[receive(
    contract = "umbrella_feeds",
    name = "destroy",
    error = "CustomContractError",
    mutable
)]
fn destroy<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State<S>, StateApiType = S>,
) -> Result<(), CustomContractError> {
    bail!(CustomContractError::NotSupported);
}

/// Part of the parameter type for the contract function `update`.
/// Specifies the message that is signed.
#[derive(SchemaType, Serialize, Clone)]
pub struct Message {
    /// The contract_address that the signature is intended for.
    pub contract_address: ContractAddress,
    /// A timestamp to make signatures expire.
    pub timestamp: Timestamp,
    /// A chain id that this message was intended for.
    pub chain_id: u16,
    /// The price feed.
    pub price_feed: Vec<(HashSha2256, PriceData)>,
}

/// The parameter type for the contract function `update`.
/// Takes a vector of signers and signatures, and the message that was signed.
#[derive(Serialize, SchemaType)]
pub struct UpdateParams {
    /// Signers and signatures. The signatures are in a two-level map to support multi-sig accounts.
    pub signers_and_signatures: Vec<(AccountAddress, AccountSignatures)>,
    /// Message that was signed.
    pub message: Message,
}

#[derive(Serialize)]
#[concordium(transparent)]
pub struct UpdateParamsPartial {
    /// Signers and signatures. The signatures are in a two-level map to support multi-sig accounts.
    pub signers_and_signatures: Vec<(AccountAddress, AccountSignatures)>,
}

/// Helper function to calculate the `message_hash`.
#[receive(
    contract = "umbrella_feeds",
    name = "viewMessageHash",
    parameter = "UpdateParams",
    return_value = "Vec<[u8;32]>",
    crypto_primitives,
    mutable
)]
fn contract_view_message_hash<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<Vec<[u8; 32]>, CustomContractError> {
    // Parse the parameter.
    let mut cursor = ctx.parameter_cursor();
    // The input parameter is `UpdateParams` but we only read the initial part of it
    // with `UpdateParamsPartial`. I.e. we read the `signatures` and the
    // `signers`, but not the `message` here.
    let param: UpdateParamsPartial = cursor.get()?;

    // The input parameter is `UpdateParams` but we have only read the initial part
    // of it with `UpdateParamsPartial` so far. We read in the `message` now.
    // `(cursor.size() - cursor.cursor_position())` is the length of the message in
    // bytes.
    let mut message_bytes = vec![0; (cursor.size() - cursor.cursor_position()) as usize];

    cursor.read_exact(&mut message_bytes)?;

    // The message signed in the Concordium browser wallet is prepended with the
    // `account` address and 8 zero bytes. Accounts in the Concordium browser wallet
    // can either sign a regular transaction (in that case the prepend is
    // `account` address and the nonce of the account which is by design >= 1)
    // or sign a message (in that case the prepend is `account` address and 8 zero
    // bytes). Hence, the 8 zero bytes ensure that the user does not accidentally
    // sign a transaction. The account nonce is of type u64 (8 bytes).
    let mut msg_prepend = vec![0u8; 32 + 8];

    let vec_length = param.signers_and_signatures.len();

    let mut message_hashes: Vec<[u8; 32]> = Vec::with_capacity(vec_length);

    for i in 0..vec_length {
        let signer = param.signers_and_signatures[i].0;

        // Prepend the `account` address of the signer.
        msg_prepend[0..32].copy_from_slice(signer.as_ref());
        // Calculate the message hash.
        message_hashes.push(
            crypto_primitives
                .hash_sha2_256(&[&msg_prepend[0..40], &message_bytes].concat())
                .0,
        );
    }

    Ok(message_hashes)
}

/// Helper function to verify the signature.
/// This function throws if the signatures are not valid.
#[receive(
    contract = "umbrella_feeds",
    name = "verifySignatures",
    parameter = "UpdateParams",
    crypto_primitives,
    mutable
)]
fn verify_signatures<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<(), CustomContractError> {
    let param: UpdateParams = ctx.parameter_cursor().get()?;

    ensure!(
        param.signers_and_signatures.len() >= host.state().required_signatures as usize,
        CustomContractError::NotEnoughSignatures
    );

    let mut prev_signer: Option<AccountAddress> = None;

    let message_hashes = contract_view_message_hash(ctx, host, crypto_primitives)?;

    let required_signatures = host.state().required_signatures;

    let mut validators: Vec<AccountAddress> = Vec::with_capacity(required_signatures as usize);

    // To save gas we check only required number of signatures.
    // The case, where you can have part of signatures invalid but still enough valid in total is not supported.
    for i in 0..required_signatures {
        let signer = param.signers_and_signatures[i as usize].0;
        let signature = &param.signers_and_signatures[i as usize].1;

        //Check signature.
        let valid_signature =
            host.check_account_signature(signer, signature, &message_hashes[i as usize])?;
        ensure!(valid_signature, CustomContractError::WrongSignature);

        ensure!(
            prev_signer < Some(signer),
            CustomContractError::SignaturesOutOfOrder
        );

        validators.push(signer);

        prev_signer = Some(signer);
    }

    let are_valid_signers = host.invoke_contract_read_only::<Vec<AccountAddress>>(
        &host.state().staking_bank,
        &validators,
        EntrypointName::new_unchecked("verifyValidators"),
        Amount::zero(),
    )?;

    let are_valid_signers: bool = are_valid_signers
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    ensure_eq!(are_valid_signers, true, CustomContractError::InvalidSigner);

    Ok(())
}

#[receive(
    contract = "umbrella_feeds",
    name = "update",
    parameter = "UpdateParams",
    error = "CustomContractError",
    crypto_primitives,
    mutable
)]
fn update<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<(), CustomContractError> {
    let param: UpdateParams = ctx.parameter_cursor().get()?;

    let message = param.message;

    // Check that the signature was intended for this contract.
    ensure_eq!(
        message.contract_address,
        ctx.self_address(),
        CustomContractError::WrongContract
    );

    // Check signature is not expired.
    ensure!(
        message.timestamp > ctx.metadata().slot_time(),
        CustomContractError::Expired
    );

    // Check signature has correct chain_id.
    ensure_eq!(
        message.chain_id,
        CHAIN_ID,
        CustomContractError::ChainIdMismatch
    );

    verify_signatures(ctx, host, crypto_primitives)?;

    for element in message.price_feed {
        let price_key: HashSha2256 = element.0;
        let new_price_data: PriceData = element.1;

        let mut stored_price_data = host
            .state_mut()
            .prices
            .entry(price_key)
            .or_insert_with(PriceData::default);

        // We do not allow for older prices.
        // This prevents replay attacks by preventing reusing of signatures at the same time.
        ensure!(
            stored_price_data.timestamp < new_price_data.timestamp,
            CustomContractError::OldData
        );

        *stored_price_data = new_price_data;
    }

    Ok(())
}

/// View function that returns the key hash/name of this contract.
#[receive(
    contract = "umbrella_feeds",
    name = "getName",
    return_value = "HashSha2256",
    crypto_primitives
)]
fn get_name<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<HashSha2256> {
    let key_hash = crypto_primitives.hash_sha2_256(NAME.as_bytes()).0;

    Ok(HashSha2256(key_hash))
}

/// View function that returns many price data. It throws if price feed does not exist.
#[receive(
    contract = "umbrella_feeds",
    name = "getManyPriceData",
    parameter = "Vec<HashSha2256>",
    return_value = "Vec<PriceData>"
)]
fn get_many_price_data<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Vec<PriceData>> {
    let key_hashes: Vec<HashSha2256> = ctx.parameter_cursor().get()?;

    let mut price_data = Vec::with_capacity(key_hashes.len());

    for key_hash in key_hashes {
        price_data.push(
            *host
                .state()
                .prices
                .get(&key_hash)
                .ok_or(CustomContractError::FeedNotExist)?,
        );
    }

    Ok(price_data)
}

/// View function that returns many price data. This function does NOT throw if the price feed does not exists.
/// The original solidity function returned a default PriceData instead of throwing an error when the price feed did not exsts in this contract.
/// This behavior (also not typical for Rust) is mimicked.
#[receive(
    contract = "umbrella_feeds",
    name = "getManyPriceDataRaw",
    parameter = "Vec<HashSha2256>",
    return_value = "Vec<PriceData>"
)]
fn get_many_price_data_raw<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Vec<PriceData>> {
    let key_hashes: Vec<HashSha2256> = ctx.parameter_cursor().get()?;

    let mut price_data = Vec::with_capacity(key_hashes.len());

    for key_hash in key_hashes {
        price_data.push(
            host.state()
                .prices
                .get(&key_hash)
                .map(|s| *s)
                .unwrap_or_else(PriceData::default),
        );
    }

    Ok(price_data)
}

/// View function that returns the price data of one price feed. This function does NOT throw if the price feed does not exists.
/// The original solidity function returned a default PriceData instead of throwing an error when the price feed did not exsts in this contract.
/// This behavior (also not typical for Rust) is mimicked.
#[receive(
    contract = "umbrella_feeds",
    name = "prices",
    parameter = "HashSha2256",
    return_value = "PriceData"
)]
fn prices<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<PriceData> {
    let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

    let price_data = host
        .state()
        .prices
        .get(&key_hash)
        .map(|s| *s)
        .unwrap_or_else(PriceData::default);

    Ok(price_data)
}

/// View function that returns the price data of one price feed. It throws if the price feed does not exist.
#[receive(
    contract = "umbrella_feeds",
    name = "getPriceData",
    parameter = "HashSha2256",
    return_value = "PriceData"
)]
fn get_price_data<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<PriceData> {
    let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

    let price_data = *host
        .state()
        .prices
        .get(&key_hash)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(price_data)
}

/// View function that returns the price of one price feed.
#[receive(
    contract = "umbrella_feeds",
    name = "getPrice",
    parameter = "HashSha2256",
    return_value = "u128"
)]
fn get_price<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u128> {
    let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

    let price_data = *host
        .state()
        .prices
        .get(&key_hash)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(price_data.price)
}

/// View function that returns the time stamp of one price feed.
#[receive(
    contract = "umbrella_feeds",
    name = "getPriceTimestamp",
    parameter = "HashSha2256",
    return_value = "Timestamp"
)]
fn get_price_timestamp<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Timestamp> {
    let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

    let price_data = *host
        .state()
        .prices
        .get(&key_hash)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(price_data.timestamp)
}

#[derive(SchemaType, Serial)]
pub struct SchemTypeTripleWrapper(u128, Timestamp, u64);

/// View function that returns the price, timestamp, and heartbeat of one price feed.
#[receive(
    contract = "umbrella_feeds",
    name = "getPriceTimestampHeartbeat",
    parameter = "HashSha2256",
    return_value = "SchemTypeTripleWrapper"
)]
fn get_price_timestamp_heartbeat<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<SchemTypeTripleWrapper> {
    let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

    let price_data = *host
        .state()
        .prices
        .get(&key_hash)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(SchemTypeTripleWrapper(
        price_data.price,
        price_data.timestamp,
        price_data.heartbeat,
    ))
}

/// View function that returns the price data by name.
#[receive(
    contract = "umbrella_feeds",
    name = "getPriceDataByName",
    parameter = "String",
    return_value = "PriceData",
    crypto_primitives
)]
fn get_price_data_by_name<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<PriceData> {
    let key: String = ctx.parameter_cursor().get()?;

    // Calculate the key hash.
    let key_hash = crypto_primitives.hash_sha2_256(key.as_bytes()).0;

    let price_data = host
        .state()
        .prices
        .get(&HashSha2256(key_hash))
        .map(|s| *s)
        .unwrap_or_else(PriceData::default);

    Ok(price_data)
}

/// View function that returns the chain id.
#[receive(contract = "umbrella_feeds", name = "getChainId", return_value = "u16")]
fn get_chain_id<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u16> {
    Ok(CHAIN_ID)
}

/// View function that returns the decimals value.
#[receive(contract = "umbrella_feeds", name = "DECIMALS", return_value = "u8")]
fn decimals<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u8> {
    Ok(host.state().decimals)
}

/// Hook function to enable `atomicUpdate` via the registry contract.
#[receive(contract = "umbrella_feeds", name = "unregister")]
fn unregister<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<()> {
    Ok(())
}
