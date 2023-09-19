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
    ParseParams, // -1
    /// Failed logging: Log is full.
    LogFull, // -2
    /// Failed logging: Log is malformed.
    LogMalformed, // -3
    NameNotRegistered, // -4
    UnauthorizedAccount, // -5
    /// Failed to invoke a contract.
    InvokeContractError, // -6
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

/// Init function that creates a new smart contract.
#[init(contract = "registry", event = "Event")]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
) -> InitResult<State<S>> {
    Ok(State {
        registry: state_builder.new_map(),
        owner: Address::from(ctx.init_origin()),
    })
}

/// The parameter type for the contract function `permit`.
/// Takes a signature, the signer, and the message that was signed.
#[derive(Serialize, SchemaType)]
pub struct ImportAddressParam {
    ///
    pub name: HashSha2256,
    ///
    pub destination: ContractAddress,
}

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct ImportAddressesParam {
    /// List of
    #[concordium(size_length = 2)]
    pub entries: Vec<ImportAddressParam>,
}

/// ATTENTION: If you want to upgrade the `UmbrellaFeeds` contract, use the `atomicUpdate` function to natively upgrade the `UmbrellaFeeds` contract.
/// Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include a reference to the `UmbrellaFeeds` contract.
#[receive(
    contract = "registry",
    name = "importAddresses",
    parameter = "ImportAddressesParam",
    error = "CustomContractError",
    enable_logger,
    mutable
)]
fn import_addresses<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), CustomContractError> {
    ensure_eq!(
        ctx.sender(),
        host.state().owner,
        CustomContractError::UnauthorizedAccount
    );

    let import_contracts: ImportAddressesParam = ctx.parameter_cursor().get()?;

    for entry in import_contracts.entries {
        host.state_mut()
            .registry
            .insert(entry.name, entry.destination);

        // Log LogRegistered event
        logger.log(&Event::LogRegistered(LogRegisteredEvent {
            name: entry.name,
            destination: entry.destination,
        }))?;
    }

    Ok(())
}

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct ImportContractsParam {
    /// List of
    #[concordium(size_length = 2)]
    pub entries: Vec<ContractAddress>,
}

/// ATTENTION: If you want to upgrade the `UmbrellaFeeds` contract, use the `atomicUpdate` function to natively upgrade the `UmbrellaFeeds` contract.
/// Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include a reference to the `UmbrellaFeeds` contract.
#[receive(
    contract = "registry",
    name = "importContracts",
    parameter = "ImportContractsParam",
    error = "CustomContractError",
    enable_logger,
    mutable,
    crypto_primitives
)]
fn import_contracts<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> Result<(), CustomContractError> {
    // Only the owner or the owner via the `UmbrellaFeeds` contract can upgrade the contract addresses
    if ctx.sender() != host.state().owner {
        // Calculate the key hash.
        let key_hash = crypto_primitives
            .hash_sha2_256("UmbrellaFeeds".as_bytes())
            .0;

        let umbrella_feeds_contract = host
            .state()
            .registry
            .get(&HashSha2256(key_hash))
            .map(|s| *s)
            .ok_or(CustomContractError::NameNotRegistered)?;

        // Only the owner or the owner via the `UmbrellaFeeds` contract can upgrade the contract addresses
        ensure!(
            ctx.sender() == host.state().owner
                || ctx.sender() == concordium_std::Address::Contract(umbrella_feeds_contract),
            CustomContractError::UnauthorizedAccount
        );
    }

    let import_contracts: ImportContractsParam = ctx.parameter_cursor().get()?;

    for contract_address in import_contracts.entries {
        let name = host.invoke_contract_read_only(
            &contract_address,
            &Parameter::empty(),
            EntrypointName::new_unchecked("getName"),
            Amount::zero(),
        )?;

        let name = name
            .ok_or(CustomContractError::InvokeContractError)?
            .get()?;

        host.state_mut().registry.insert(name, contract_address);

        // Log LogRegistered event
        logger.log(&Event::LogRegistered(LogRegisteredEvent {
            name,
            destination: contract_address,
        }))?;
    }

    Ok(())
}

/// The parameter type for the contract function `permit`.
/// Takes a signature, the signer, and the message that was signed.
#[derive(Serialize, SchemaType)]
pub struct AtomicUpdateParam {
    /// The new module reference.
    pub module: ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
    pub contract_address: ContractAddress,
}

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct AtomicUpdateParams {
    /// List of
    #[concordium(size_length = 2)]
    pub entries: Vec<AtomicUpdateParam>,
}

#[derive(Debug, Serialize, SchemaType)]
pub struct UpgradeParams {
    /// The new module reference.
    pub module: ModuleReference,
    /// Optional entrypoint to call in the new module after upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

///
#[receive(
    contract = "registry",
    name = "atomicUpdate",
    parameter = "AtomicUpdateParams",
    error = "CustomContractError",
    enable_logger,
    mutable
)]
fn atomic_update<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), CustomContractError> {
    ensure_eq!(
        ctx.sender(),
        host.state().owner,
        CustomContractError::UnauthorizedAccount
    );

    let import_contracts: AtomicUpdateParams = ctx.parameter_cursor().get()?;

    for new_contract in import_contracts.entries {
        let upgrade_params = UpgradeParams {
            module: new_contract.module,
            migrate: new_contract.migrate,
        };

        //  `upgradeNatively()` hook; this can be used to natively upgrade the contract
        host.invoke_contract::<UpgradeParams>(
            &new_contract.contract_address,
            &upgrade_params,
            EntrypointName::new_unchecked("upgradeNatively"),
            Amount::zero(),
        )?;

        let name = host.invoke_contract_read_only(
            &new_contract.contract_address,
            &Parameter::empty(),
            EntrypointName::new_unchecked("getName"),
            Amount::zero(),
        )?;

        let name = name
            .ok_or(CustomContractError::InvokeContractError)?
            .get()?;

        let old_contract = host
            .state_mut()
            .registry
            .insert(name, new_contract.contract_address);

        // Only if another `old_contract` was already registered, execute the `unregister` hook.
        if let Some(old_contract) = old_contract {
            // unRegister() hook
            host.invoke_contract(
                &old_contract,
                &Parameter::empty(),
                EntrypointName::new_unchecked("unregister"),
                Amount::zero(),
            )?;
        }

        // Log LogRegistered event
        logger.log(&Event::LogRegistered(LogRegisteredEvent {
            name,
            destination: new_contract.contract_address,
        }))?;
    }

    Ok(())
}

/// View function that returns contract_address from key hash.
#[receive(
    contract = "registry",
    name = "requireAndGetAddress",
    parameter = "HashSha2256",
    return_value = "ContractAddress"
)]
fn require_and_get_address<S: HasStateApi>(
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

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `registry`.
#[receive(
    contract = "registry",
    name = "registry",
    parameter = "HashSha2256",
    return_value = "ContractAddress"
)]
fn registry<S: HasStateApi>(
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

    // Calculate the key hash.
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

/// View function that hash from a key string.
#[receive(contract = "registry", name = "owner", return_value = "Address")]
fn owner<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State<S>, StateApiType = S>,
) -> ReceiveResult<Address> {
    Ok(host.state().owner)
}
