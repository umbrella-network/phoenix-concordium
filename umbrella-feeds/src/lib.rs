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
struct State<S> {
    deployed_at: Timestamp,
    registry: ContractAddress,
    staking_bank: ContractAddress,
    required_signatures: u16,
    decimals: u8,
    // name => PriceData
    prices: StateMap<HashSha2256, PriceData, S>,
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
pub struct InitContractsParam {
    pub registry: ContractAddress,
    pub required_signatures: u16,
    pub staking_bank: ContractAddress,
    pub decimals: u8,
}

/// Init function that creates a new smart contract.
#[init(
    contract = "umbrella_feeds",
    parameter = "InitContractsParam",
    event = "Event"
)]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    let param: InitContractsParam = ctx.parameter_cursor().get()?;

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

/// The parameter type for the contract function `upgrade`.
/// Takes the new module and optionally an entrypoint to call in the new module
/// after triggering the upgrade. The upgrade is reverted if the entrypoint
/// fails. This is useful for doing migration in the same transaction triggering
/// the upgrade.
#[derive(Debug, Serialize, SchemaType)]
pub struct UpgradeParams {
    /// The new module reference.
    pub module: ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

/// Upgrade this smart contract instance to a new module and call optionally a
/// migration function after the upgrade.
///
/// It rejects if:
/// - Sender is not the owner of the registry contract instance.
/// - It fails to parse the parameter.
/// - If the ugrade fails.
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
    name = "upgrade",
    parameter = "UpgradeParams",
    error = "CustomContractError",
    low_level
)]
fn contract_upgrade<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<S>,
) -> Result<(), CustomContractError> {
    // Read the top-level contract state.
    let state: State<S> = host.state().read_root()?;

    let owner = host.invoke_contract_read_only(
        &state.registry,
        &Parameter::empty(),
        EntrypointName::new_unchecked("owner"),
        Amount::zero(),
    )?;

    let owner = owner
        .ok_or(CustomContractError::InvokeContractError)?
        .get()?;

    // Check that only the owner is authorized to upgrade the smart contract.
    ensure_eq!(ctx.sender(), owner, CustomContractError::Unauthorized);

    // if (_prices[keccak256(abi.encodePacked(_name))].timestamp == 0 && DEPLOYED_AT + 3 days > block.timestamp) {
    //     revert ContractNotInitialised();
    // }

    // Check that
    // ensure_eq!(
    //     state
    //         .deployed_at
    //         .checked_add(Duration::from_days(3))
    //         .ok_or(CustomContractError::OverFlow),
    //     Ok(ctx.metadata().block_time()),
    //     CustomContractError::ContractNotInitialised
    // );

    // Parse the parameter.
    let param: UpgradeParams = ctx.parameter_cursor().get()?;

    let parameter = ImportContractsParam {
        entries: vec![ctx.self_address()],
    };

    // Update contract in registry
    host.invoke_contract_raw(
        &state.registry,
        to_bytes(&parameter).as_slice().try_into().unwrap(),
        EntrypointName::new_unchecked("importContracts"),
        Amount::zero(),
    )?;

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
    bail!(CustomContractError::NotSupportedUseUpgradeFunctionInstead);
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
    // The input parameter is `PermitParam` but we only read the initial part of it
    // with `PermitParamPartial`. I.e. we read the `signature` and the
    // `signer`, but not the `message` here.
    let param: UpdateParamsPartial = cursor.get()?;

    // The input parameter is `PermitParam` but we have only read the initial part
    // of it with `PermitParamPartial` so far. We read in the `message` now.
    // `(cursor.size() - cursor.cursor_position()` is the length of the message in
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
    let mut msg_prepend = vec![0; 32 + 8];

    let mut message_hashes: Vec<[u8; 32]> = vec![];

    for i in 0..param.signer.len() {
        // Prepend the `account` address of the signer.
        msg_prepend[0..32].copy_from_slice(param.signer[i].as_ref());
        // Prepend 8 zero bytes.
        msg_prepend[32..40].copy_from_slice(&[0u8; 8]);
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
fn verify_signatures<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<bool, CustomContractError> {
    let param: UpdateParams = ctx.parameter_cursor().get()?;

    ensure!(
        param.signatures.len() >= host.state().required_signatures as usize,
        CustomContractError::NotEnoughSignatures
    );

    let mut prev_signer = AccountAddress([0u8; 32]);

    let message_hash = contract_view_message_hash(ctx, host, crypto_primitives)?;

    let mut validators: Vec<AccountAddress> = vec![];

    // to save gas we check only required number of signatures
    // case, where you can have part of signatures invalid but still enough valid in total is not supported
    for i in 0..host.state().required_signatures {
        //Check signature.
        let valid_signature = host.check_account_signature(
            param.signer[i as usize],
            &param.signatures[i as usize],
            &message_hash[i as usize],
        )?;
        ensure!(valid_signature, CustomContractError::WrongSignature);

        ensure!(
            prev_signer < param.signer[i as usize],
            CustomContractError::SignaturesOutOfOrder
        );

        validators.push(param.signer[i as usize]);

        prev_signer = param.signer[i as usize];
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

    Ok(true)
}

/// Part of the parameter type for the contract function `permit`.
/// Specifies the message that is signed.
#[derive(SchemaType, Serialize, Clone)]
pub struct Message {
    /// The contract_address that the signature is intended for.
    pub contract_address: ContractAddress,
    /// A timestamp to make signatures expire.
    pub timestamp: Timestamp,
    pub chain_id: u16,
    /// The entry_point that the signature is intended for.
    pub entry_point: OwnedEntrypointName,
    pub price_feed: Vec<(HashSha2256, PriceData)>,
}

/// The parameter type for the contract function `permit`.
/// Takes a signature, the signer, and the message that was signed.
#[derive(Serialize, SchemaType)]
pub struct UpdateParams {
    /// Signature/s. The CIS3 standard supports multi-sig accounts.
    pub signatures: Vec<AccountSignatures>,
    /// Accounts that created the above signatures.
    pub signer: Vec<AccountAddress>,
    /// Message that was signed.
    pub message: Message,
}

#[derive(Serialize)]
pub struct UpdateParamsPartial {
    /// Signature/s. The CIS3 standard supports multi-sig accounts.
    pub signature: Vec<AccountSignatures>,
    /// Accounts that created the above signatures.
    pub signer: Vec<AccountAddress>,
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

    // // Update the nonce.
    // let mut entry = host.state_mut().nonces_registry.entry(param.signer).or_insert_with(|| 0);

    // // Get the current nonce.
    // let nonce = *entry;
    // // Bump nonce.
    // *entry += 1;
    // drop(entry);

    ensure!(
        param.signatures.len() == param.signer.len(),
        CustomContractError::ArraysDataDoNotMatch
    );

    let message = param.message;

    // Check the nonce to prevent replay attacks.
    // ensure_eq!(message.nonce, nonce, CustomContractError::NonceMismatch);

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

    // Check signature has correct entrypoint.
    ensure_eq!(
        message.entry_point,
        OwnedEntrypointName::new_unchecked(String::from("update")),
        CustomContractError::EntrypointMismatch
    );

    let _is_ok = verify_signatures(ctx, host, crypto_primitives)?;

    for element in message.price_feed {
        let price_key: HashSha2256 = element.0;
        let price_data: PriceData = element.1;

        // we do not allow for older prices
        // at the same time it prevents from reusing signatures
        let old_price_data = host.state().prices.get(&price_key).map(|s| *s);
        if let Some(old_price_data) = old_price_data {
            ensure!(
                old_price_data.timestamp < price_data.timestamp,
                CustomContractError::OldData
            );
        }

        host.state_mut().prices.insert(price_key, price_data);
    }

    Ok(())
}

/// View function that returns the balance of an validator
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
    let key_hash = crypto_primitives
        .hash_sha2_256("UmbrellaFeeds".as_bytes())
        .0;

    Ok(HashSha2256(key_hash))
}

/// View function that return many price data
#[receive(
    contract = "umbrella_feeds",
    name = "getManyPriceData",
    parameter = "Vec<HashSha2256>",
    return_value = "Vec<PriceData>"
)]
fn get_mny_price_data<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Vec<PriceData>> {
    let key_hashes: Vec<HashSha2256> = ctx.parameter_cursor().get()?;

    let mut price_data = vec![];

    for key_hash in key_hashes {
        price_data.push(
            host.state()
                .prices
                .get(&key_hash)
                .map(|s| *s)
                .ok_or(CustomContractError::FeedNotExist)?,
        );
    }

    Ok(price_data)
}

/// View function that return many price data
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

    let mut price_data = vec![];

    for key_hash in key_hashes {
        price_data.push(
            host.state()
                .prices
                .get(&key_hash)
                .map(|s| *s)
                .unwrap_or(PriceData::default()),
        );
    }

    Ok(price_data)
}

/// View function that returns the balance of an validator
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
        .unwrap_or(PriceData::default());

    Ok(price_data)
}

/// View function that returns the balance of an validator
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

    let price_data = host
        .state()
        .prices
        .get(&key_hash)
        .map(|s| *s)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(price_data)
}

/// View function that returns the balance of an validator
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

    let price_data = host
        .state()
        .prices
        .get(&key_hash)
        .map(|s| *s)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(price_data.price)
}

/// View function that returns the balance of an validator
#[receive(
    contract = "umbrella_feeds",
    name = "getPriceTimestamp",
    parameter = "HashSha2256",
    return_value = "u32"
)]
fn get_price_timestamp<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u32> {
    let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

    let price_data = host
        .state()
        .prices
        .get(&key_hash)
        .map(|s| *s)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(price_data.timestamp)
}

#[derive(SchemaType, Serial)]
pub struct SchemTypeTripleWrapper(u128, u32, u64);

/// View function that returns the balance of an validator
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

    let price_data = host
        .state()
        .prices
        .get(&key_hash)
        .map(|s| *s)
        .ok_or(CustomContractError::FeedNotExist)?;

    Ok(SchemTypeTripleWrapper(
        price_data.price,
        price_data.timestamp,
        price_data.heartbeat,
    ))
}

/// View function that returns the balance of an validator
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
        .unwrap_or(PriceData::default());

    Ok(price_data)
}

/// View function that returns the balance of an validator
#[receive(contract = "umbrella_feeds", name = "getChainId", return_value = "u16")]
fn get_chain_id<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<u16> {
    Ok(CHAIN_ID)
}
