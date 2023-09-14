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

#[derive(Serialize, SchemaType, Copy, Clone)]
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
    ParseParams,
    /// Failed logging: Log is full.
    LogFull,
    /// Failed logging: Log is malformed.
    LogMalformed,
    /// Failed to invoke a contract.
    InvokeContractError,
    InvalidRequiredSignatures,
    ValidatorDoesNotExist,
    ValidatorsCountMisMatch,
    NotValidator,
    OverFlow,
    NotSupportedUseUpgradeFunctionInstead,
    ContractNotInitialised,
    ArraysDataDoNotMatch,
    ChainIdMismatch,
    OldData,
    WrongContract,
    Expired,
    Unauthorized,
    /// Upgrade failed because the new module does not exist.
    FailedUpgradeMissingModule,
    /// Upgrade failed because the new module does not contain a contract with a
    /// matching name.
    FailedUpgradeMissingContract,
    /// Upgrade failed because the smart contract version of the module is not
    /// supported.
    FailedUpgradeUnsupportedModuleVersion,
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
        Parameter::from(to_bytes(&parameter).as_slice().try_into().unwrap()),
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
    return_value = "[u8;32]",
    crypto_primitives,
    mutable
)]
fn contract_view_message_hash<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &mut impl HasHost<State<S>, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<[u8; 32], CustomContractError> {
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
    // Prepend the `account` address of the signer.
    msg_prepend[0..32].copy_from_slice(param.signer.as_ref());
    // Prepend 8 zero bytes.
    msg_prepend[32..40].copy_from_slice(&[0u8; 8]);
    // Calculate the message hash.
    let message_hash = crypto_primitives
        .hash_sha2_256(&[&msg_prepend[0..40], &message_bytes].concat())
        .0;

    Ok(message_hash)
}

// #[derive(Serialize, SchemaType)]
// pub struct UpdateParams {
//     pub price_keys: Vec<HashSha2256>,
//     pub price_datas: Vec<PriceData>,
//     pub signatures: Vec<Signature>,
// }

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
    pub signature: Vec<AccountSignatures>,
    /// Account that created the above signature.
    pub signer: AccountAddress,
    /// Message that was signed.
    pub message: Message,
}

#[derive(Serialize)]
pub struct UpdateParamsPartial {
    /// Signature/s. The CIS3 standard supports multi-sig accounts.
    signature: Vec<AccountSignatures>,
    /// Account that created the above signature.
    signer: AccountAddress,
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

    let message = param.message;

    // Check the nonce to prevent replay attacks.
    // ensure_eq!(message.nonce, nonce, CustomContractError::NonceMismatch.into());

    // Check that the signature was intended for this contract.
    ensure_eq!(
        message.contract_address,
        ctx.self_address(),
        CustomContractError::WrongContract.into()
    );

    // Check signature is not expired.
    ensure!(
        message.timestamp > ctx.metadata().slot_time(),
        CustomContractError::Expired.into()
    );

    // Check signature has correct chain_id.
    ensure!(
        message.chain_id > CHAIN_ID,
        CustomContractError::ChainIdMismatch.into()
    );

    let message_hash = contract_view_message_hash(ctx, host, crypto_primitives)?;

    println!("{:?}", message_hash);
    // verifySignatures(priceDataHash, _signatures);

    for element in message.price_feed {
        let price_key: HashSha2256 = element.0;
        let price_data: PriceData = element.1;

        // we do not allow for older prices
        // at the same time it prevents from reusing signatures
        let old_price_data = host.state().prices.get(&price_key).map(|s| *s);
        if let Some(old_price_data) = old_price_data {
            ensure!(
                old_price_data.timestamp < price_data.timestamp,
                CustomContractError::OldData.into()
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
