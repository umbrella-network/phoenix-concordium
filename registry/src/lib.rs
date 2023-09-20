#![cfg_attr(not(feature = "std"), no_std)]

//! # Contracts Registry
//!
//! The protocol uses this registry to fetch current contract addresses.
//! This contract has an owner.
//! The owner can:
//! - Register contracts into this registry with the `importAddresses` and the `importContracts` entry points.
//! - Natively upgrade the `UmbrellaFeeds` contract via this registry contract by invoking the `atomicUpdate` entry point.
//! - Override contract addresses registered (e.g. in case they don't have the entry points `upgradeNatively` implemented) by invoking the `importAddresses` and the `importContracts` entry points.
//! ATTENTION: If you want to upgrade the `UmbrellaFeeds` contract, use the `atomicUpdate` function to natively upgrade the `UmbrellaFeeds` contract.
//! Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include references to the `UmbrellaFeeds` contract.
use concordium_std::*;
use core::fmt::Debug;

#[derive(Serial, DeserialWithState)]
#[concordium(state_parameter = "S")]
struct State<S: HasStateApi> {
    // The owner of this contract. It can register/override/atomically upgrade contract addresses in this registry.
    owner: Address,
    // Mapping from key to contract address. The key/name of a contract is calculated by hashing its human-readable name.
    registry: StateMap<HashSha2256, ContractAddress, S>,
}

/// All smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serial, SchemaType)]
enum CustomContractError {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams, // -1
    /// Failed to log because the log is full.
    LogFull, // -2
    /// Failed to log because the log is malformed.
    LogMalformed, // -3'
    /// Failed to retrieve a contract address because the contract is not registered in this registry.
    NameNotRegistered, // -4
    /// Failed because the invoker is not authorized to invoke the entry point.
    UnauthorizedAccount, // -5
    /// Failed to invoke a contract.
    InvokeContractError, // -6
    /// Failed to provide a valid address as the owner.
    InvalidOwner, // -7
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
    /// The event tracks whenever a new contract address gets registered/atomically upgraded in this registry (potentially replacing an old contract address).
    #[concordium(tag = 0)]
    LogRegistered(LogRegisteredEvent),
    /// The event tracks whenever the contract ownership gets transferred.
    #[concordium(tag = 1)]
    OwnershipTransferred(OwnershipTransferredEvent),
}

/// The LogRegisteredEvent is logged when a new contract address gets registered/atomically upgraded in this registry (potentially replacing an old contract address).
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct LogRegisteredEvent {
    /// The new contract address that got registered.
    pub destination: ContractAddress,
    /// The key/name of a contract.
    pub name: HashSha2256,
}

/// The OwnershipTransferredEvent is logged when the contract ownership gets transferred.
#[derive(Debug, Serialize, SchemaType, PartialEq, Eq)]
pub struct OwnershipTransferredEvent {
    /// The previous owner's address.
    pub previous_owner: Address,
    /// The new owner's address.
    pub new_owner: Address,
}

/// The init function that creates a new registry smart contract.
#[init(contract = "registry", event = "Event", enable_logger)]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    state_builder: &mut StateBuilder<S>,
    logger: &mut impl HasLogger,
) -> InitResult<State<S>> {
    let owner = Address::from(ctx.init_origin());

    // Log OwnershipTransferred event
    logger.log(&Event::OwnershipTransferred(OwnershipTransferredEvent {
        new_owner: owner,
        previous_owner: Address::from(AccountAddress([0u8; 32])),
    }))?;

    Ok(State {
        registry: state_builder.new_map(),
        owner,
    })
}

/// Part of the parameter type for the contract function `importAddresses`.
#[derive(Serialize, SchemaType)]
pub struct ImportAddressesParam {
    /// The new contract address that got registered.
    pub name: HashSha2256,
    /// The key/name of a contract.
    pub destination: ContractAddress,
}

/// The parameter type for the contract function `importAddresses`.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct ImportAddressesParams {
    /// List of ImportAddressParam.
    #[concordium(size_length = 2)]
    pub entries: Vec<ImportAddressesParam>,
}

/// The owner can import new contract addresses and override old addresses (if they exist under the provided name) by providing the new contract address and its key/name.
/// This method can be used for contracts that for some reason do not have the `getName` method.
/// ATTENTION: If you want to upgrade the `UmbrellaFeeds` contract, use the `atomicUpdate` function to natively upgrade the `UmbrellaFeeds` contract.
/// Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include references to the `UmbrellaFeeds` contract.
#[receive(
    contract = "registry",
    name = "importAddresses",
    parameter = "ImportAddressesParams",
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

    let import_contracts: ImportAddressesParams = ctx.parameter_cursor().get()?;

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

/// The parameter type for the contract function `importContracts`.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct ImportContractsParam {
    /// List of contract addresses.
    #[concordium(size_length = 2)]
    pub entries: Vec<ContractAddress>,
}

/// The owner can import new contract addresses and override old addresses (if they exist under the provided name) by providing the new contract address. The key/name of the contract is queried from the provided contract address.
/// ATTENTION: If you want to upgrade the `UmbrellaFeeds` contract, use the `atomicUpdate` function to natively upgrade the `UmbrellaFeeds` contract.
/// Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include references to the `UmbrellaFeeds` contract.
#[receive(
    contract = "registry",
    name = "importContracts",
    parameter = "ImportContractsParam",
    error = "CustomContractError",
    enable_logger,
    mutable
)]
fn import_contracts<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), CustomContractError> {
    ensure_eq!(
        ctx.sender(),
        host.state().owner,
        CustomContractError::UnauthorizedAccount
    );

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

/// The parameter type for the contract function `atomicUpdate`.
#[derive(Serialize, SchemaType)]
pub struct AtomicUpdateParam {
    /// The new module reference.
    pub module: ModuleReference,
    /// Optional entry point to call in the new module after the upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
    /// The contract address to natively upgrade.
    pub contract_address: ContractAddress,
}

/// The parameter type for the contract function `upgradeNatively`.
#[derive(Debug, Serialize, SchemaType)]
pub struct UpgradeParams {
    /// The new module reference.
    pub module: ModuleReference,
    /// Optional entry point to call in the new module after the upgrade.
    pub migrate: Option<(OwnedEntrypointName, OwnedParameter)>,
}

/// This method ensures, that the old and the new contracts are aware of their states in the registry by calling the `upgradeNatively` and the `unregister` hooks.
/// ATTENTION: If you want to upgrade the `UmbrellaFeeds` contract, use this function to natively upgrade the `UmbrellaFeeds` contract.
/// Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include references to the `UmbrellaFeeds` contract.
#[receive(
    contract = "registry",
    name = "atomicUpdate",
    parameter = "AtomicUpdateParam",
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

    let params: AtomicUpdateParam = ctx.parameter_cursor().get()?;

    let upgrade_params = UpgradeParams {
        module: params.module,
        migrate: params.migrate,
    };

    //  `upgradeNatively()` hook; this can be used to natively upgrade the contract
    host.invoke_contract::<UpgradeParams>(
        &params.contract_address,
        &upgrade_params,
        EntrypointName::new_unchecked("upgradeNatively"),
        Amount::zero(),
    )?;

    let name = host.invoke_contract_read_only(
        &params.contract_address,
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
        .insert(name, params.contract_address);

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
        destination: params.contract_address,
    }))?;

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
    name = "stringToHashSha2256",
    parameter = "String",
    return_value = "HashSha2256",
    crypto_primitives
)]
fn string_to_hash_sha2256<S: HasStateApi>(
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

#[receive(
    contract = "registry",
    name = "renounceOwnership",
    error = "CustomContractError",
    enable_logger,
    mutable
)]
fn renounce_ownership<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), CustomContractError> {
    ensure_eq!(
        ctx.sender(),
        host.state().owner,
        CustomContractError::UnauthorizedAccount
    );

    let previous_owner = host.state_mut().owner;
    host.state_mut().owner = Address::from(AccountAddress([0u8; 32]));

    // Log OwnershipTransferred event
    logger.log(&Event::OwnershipTransferred(OwnershipTransferredEvent {
        new_owner: Address::from(AccountAddress([0u8; 32])),
        previous_owner,
    }))?;

    Ok(())
}

#[receive(
    contract = "registry",
    name = "transferOwnership",
    parameter = "Address",
    error = "CustomContractError",
    enable_logger,
    mutable
)]
fn transfer_ownership<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &mut impl HasHost<State<S>, StateApiType = S>,
    logger: &mut impl HasLogger,
) -> Result<(), CustomContractError> {
    ensure_eq!(
        ctx.sender(),
        host.state().owner,
        CustomContractError::UnauthorizedAccount
    );

    let new_owner: Address = ctx.parameter_cursor().get()?;

    ensure!(
        new_owner != Address::from(AccountAddress([0u8; 32])),
        CustomContractError::InvalidOwner
    );

    let previous_owner = host.state_mut().owner;
    host.state_mut().owner = new_owner;

    // Log OwnershipTransferred event
    logger.log(&Event::OwnershipTransferred(OwnershipTransferredEvent {
        new_owner,
        previous_owner,
    }))?;

    Ok(())
}
