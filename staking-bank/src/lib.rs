#![cfg_attr(not(feature = "std"), no_std)]

//! # Staking Bank
use concordium_std::*;
use core::fmt::Debug;

#[cfg(feature = "u256_amount")]
use primitive_types::U256;

const VALIDATOR_0: AccountAddress = AccountAddress([0u8; 32]);
const VALIDATOR_1: AccountAddress = AccountAddress([1u8; 32]);

// external order is based on validators submits on AVAX for Apr 2023
const VALIDATOR_2: AccountAddress = AccountAddress([2u8; 32]);
const VALIDATOR_3: AccountAddress = AccountAddress([3u8; 32]);
const VALIDATOR_4: AccountAddress = AccountAddress([4u8; 32]);
const VALIDATOR_5: AccountAddress = AccountAddress([5u8; 32]);
const VALIDATOR_6: AccountAddress = AccountAddress([6u8; 32]);
const VALIDATOR_7: AccountAddress = AccountAddress([7u8; 32]);
const VALIDATOR_8: AccountAddress = AccountAddress([8u8; 32]);
const VALIDATOR_9: AccountAddress = AccountAddress([9u8; 32]);
const VALIDATOR_10: AccountAddress = AccountAddress([10u8; 32]);
const VALIDATOR_11: AccountAddress = AccountAddress([11u8; 32]);
const VALIDATOR_12: AccountAddress = AccountAddress([12u8; 32]);
const VALIDATOR_13: AccountAddress = AccountAddress([13u8; 32]);
const VALIDATOR_14: AccountAddress = AccountAddress([14u8; 32]);

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

impl ops::Mul<U256Wrapper> for U256Wrapper {
    type Output = Self;

    fn mul(self, rhs: U256Wrapper) -> Self::Output {
        U256Wrapper(self.0 * rhs.0)
    }
}

#[derive(Serial, Deserial, Debug, SchemaType)]
struct State {
    number_of_validators: U256Wrapper,
    total_supply: U256Wrapper,
    one: U256Wrapper,
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
    ValidatorDoesNotExist,
    ValidatorsCountMisMatch,
    NotValidator,
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

/// Get _isValidator
fn _is_validator(_validator: AccountAddress) -> bool {
    _validator == VALIDATOR_0
        || _validator == VALIDATOR_1
        || _validator == VALIDATOR_2
        || _validator == VALIDATOR_3
        || _validator == VALIDATOR_4
        || _validator == VALIDATOR_5
        || _validator == VALIDATOR_6
        || _validator == VALIDATOR_7
        || _validator == VALIDATOR_8
        || _validator == VALIDATOR_9
        || _validator == VALIDATOR_10
        || _validator == VALIDATOR_11
        || _validator == VALIDATOR_12
        || _validator == VALIDATOR_13
        || _validator == VALIDATOR_14
}

/// Get _addresses
fn _addresses() -> Vec<AccountAddress> {
    vec![
        VALIDATOR_0,
        VALIDATOR_1,
        VALIDATOR_2,
        VALIDATOR_3,
        VALIDATOR_4,
        VALIDATOR_5,
        VALIDATOR_6,
        VALIDATOR_7,
        VALIDATOR_8,
        VALIDATOR_9,
        VALIDATOR_10,
        VALIDATOR_11,
        VALIDATOR_12,
        VALIDATOR_13,
        VALIDATOR_14,
    ]
}

/// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
/// for the public key/nonce of a given account.
#[derive(Serialize, SchemaType)]
#[concordium(transparent)]
pub struct InitContractsParam {
    pub validators_count: U256Wrapper,
}

/// Init function that creates a new smart contract.
#[init(
    contract = "staking_bank",
    parameter = "InitContractsParam"
)]
fn init<S: HasStateApi>(
    ctx: &impl HasInitContext,
    _state_builder: &mut StateBuilder<S>,
) -> InitResult<State> {
    let param: InitContractsParam = ctx.parameter_cursor().get()?;

    let one = U256Wrapper(U256::from_dec_str("1000000000000000000").unwrap());

    let list = _addresses();

    ensure_eq!(
        list.len(),
        param.validators_count.0.as_usize(),
        CustomContractError::ValidatorsCountMisMatch.into()
    );

    for validator in list {
        ensure!(
            _is_validator(validator),
            CustomContractError::NotValidator.into()
        );
    }

    Ok(State {
        number_of_validators: param.validators_count,
        total_supply: param.validators_count * one,
        one,
    })
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `NUMBER_OF_VALIDATORS`.
#[receive(
    contract = "staking_bank",
    name = "NUMBER_OF_VALIDATORS",
    return_value = "U256Wrapper"
)]
fn number_of_validators<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<U256Wrapper> {
    Ok(host.state().number_of_validators)
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `TOTAL_SUPPLY`.
#[receive(
    contract = "staking_bank",
    name = "TOTAL_SUPPLY",
    return_value = "U256Wrapper"
)]
fn total_supply_1<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<U256Wrapper> {
    Ok(host.state().total_supply)
}

/// Equivalent to solidity's getter function which is automatically created from the public storage variable `ONE`.
#[receive(contract = "staking_bank", name = "ONE", return_value = "U256Wrapper")]
fn one<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<U256Wrapper> {
    Ok(host.state().one)
}

/// View function that returns the content of the state for debugging purposes.
#[receive(contract = "staking_bank", name = "view", return_value = "State")]
fn view<'b, S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &'b impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<&'b State> {
    Ok(host.state())
}

/// View function that returns validators
#[receive(
    contract = "staking_bank",
    name = "validators",
    parameter = "AccountAddress",
    return_value = "(AccountAddress,String)"
)]
fn validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<(AccountAddress, String)> {
    let id: AccountAddress = ctx.parameter_cursor().get()?;

    match id {
        VALIDATOR_0 => Ok((id, "https://validator.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.umb.network".to_string())),
        VALIDATOR_2 => Ok((id, "https://umbrella.artemahr.tech".to_string())),
        VALIDATOR_3 => Ok((id, "https://umb.vtabsolutions.com:3030".to_string())),
        VALIDATOR_4 => Ok((id, "https://umb.stakers.world".to_string())),
        VALIDATOR_5 => Ok((id, "https://umbrella.crazywhale.es".to_string())),
        VALIDATOR_6 => Ok((id, "https://umbrella-node.gateomega.com".to_string())),
        VALIDATOR_7 => Ok((id, "https://umb.anorak.technology".to_string())),
        VALIDATOR_8 => Ok((id, "https://umbrella.infstones.io".to_string())),
        VALIDATOR_9 => Ok((id, "https://umb.hashquark.io".to_string())),
        VALIDATOR_10 => Ok((id, "http://umbrella.staking4all.org:3000".to_string())),
        VALIDATOR_11 => Ok((id, "https://umbrella-api.validatrium.club".to_string())),
        VALIDATOR_12 => Ok((id, "http://5.161.78.230:3000".to_string())),
        VALIDATOR_13 => Ok((id, "https://umbnode.blockchainliverpool.com".to_string())),
        VALIDATOR_14 => Ok((id, "https://umb-api.staking.rocks".to_string())),
        _ => bail!(CustomContractError::ValidatorDoesNotExist.into()),
    }
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "balances",
    parameter = "AccountAddress",
    return_value = "U256Wrapper"
)]
fn balances<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<U256Wrapper> {
    let _account: AccountAddress = ctx.parameter_cursor().get()?;

    if _is_validator(_account) {
        Ok(host.state().one)
    } else {
        Ok(U256Wrapper(U256::from_dec_str("0").unwrap()))
    }
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "verifyValidators",
    parameter = "Vec<AccountAddress>",
    return_value = "bool"
)]
fn verify_validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
    let _accounts: Vec<AccountAddress> = ctx.parameter_cursor().get()?;

    for _validator in _accounts {
        if _is_validator(_validator) {
            return Ok(false);
        }
    }

    Ok(true)
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "getNumberOfValidators",
    return_value = "U256Wrapper"
)]
fn get_number_of_validators<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<U256Wrapper> {
    Ok(host.state().number_of_validators)
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "getAddresses",
    return_value = "Vec<AccountAddress>"
)]
fn get_addresses<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<Vec<AccountAddress>> {
    Ok(_addresses())
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "getBalances",
    return_value = "Vec<U256Wrapper>"
)]
fn get_balances<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<Vec<U256Wrapper>> {
    let one = host.state().one;
    let number_of_validators = host.state().number_of_validators;

    Ok(vec![one, number_of_validators])
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "addresses",
    parameter = "u8",
    return_value = "AccountAddress"
)]
fn addresses<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<AccountAddress> {
    let index: u8 = ctx.parameter_cursor().get()?;
    Ok(_addresses()[<u8 as Into<usize>>::into(index)])
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "balanceOf",
    parameter = "AccountAddress",
    return_value = "U256Wrapper"
)]
fn balance_of<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<U256Wrapper> {
    let _account: AccountAddress = ctx.parameter_cursor().get()?;

    if _is_validator(_account) {
        Ok(host.state().one)
    } else {
        Ok(U256Wrapper(U256::from_dec_str("0").unwrap()))
    }
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "totalSupply",
    return_value = "U256Wrapper"
)]
fn total_supply_2<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<U256Wrapper> {
    Ok(host.state().total_supply)
}

/// View function that returns the balance of an validator
#[receive(
    contract = "staking_bank",
    name = "getName",
    return_value = "HashSha2256",
    crypto_primitives
)]
fn get_name<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
    crypto_primitives: &impl HasCryptoPrimitives,
) -> ReceiveResult<HashSha2256> {
    let key_hash = crypto_primitives.hash_sha2_256("StakingBank".as_bytes()).0;

    Ok(HashSha2256(key_hash))
}

/// View function that returns the balance of an validator
#[receive(contract = "staking_bank", name = "register")]
fn register<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<()> {
    // there are no requirements atm
    Ok(())
}

/// View function that returns the balance of an validator
#[receive(contract = "staking_bank", name = "unregister")]
fn unregister<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<()> {
    // there are no requirements atm
    Ok(())
}

/// View function that returns the balance of an validator
#[receive(contract = "staking_bank", name = "_addresses", return_value="Vec<AccountAddress>")]
fn addresses_external<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<Vec<AccountAddress>> {
    Ok(_addresses())
}

/// View function that returns the balance of an validator
#[receive(contract = "staking_bank", name = "_isValidator", parameter="AccountAddress", return_value="bool")]
fn is_validator<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<bool> {
  let _account: AccountAddress = ctx.parameter_cursor().get()?;

    Ok(_is_validator(_account))
}

// /// The parameter type for the contract function `permit`.
// /// Takes a signature, the signer, and the message that was signed.
// #[derive(Serialize, SchemaType)]
// pub struct ImportAddressParam {
//     ///
//     pub name: HashSha2256,
//     ///
//     pub destination: ContractAddress,
// }

// /// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
// /// for the public key/nonce of a given account.
// #[derive(Serialize, SchemaType)]
// #[concordium(transparent)]
// pub struct ImportAddressesParam {
//     /// List of
//     #[concordium(size_length = 2)]
//     pub entries: Vec<ImportAddressParam>,
// }

// ///
// #[receive(
//     contract = "staking_bank",
//     name = "importAddresses",
//     parameter = "ImportAddressesParam",
//     error = "CustomContractError",
//     enable_logger,
//     mutable
// )]
// fn import_addresses<S: HasStateApi>(
//     ctx: &impl HasReceiveContext,
//     host: &mut impl HasHost<State<S>, StateApiType = S>,
//     logger: &mut impl HasLogger,
// ) -> Result<(), CustomContractError> {
//     ensure_eq!(
//         ctx.sender(),
//         host.state().owner,
//         CustomContractError::UnauthorizedAccount
//     );

//     let import_contracts: ImportAddressesParam = ctx.parameter_cursor().get()?;

//     for entry in import_contracts.entries {
//         host.state_mut()
//             .registry
//             .insert(entry.name, entry.destination);

//         // Log LogRegistered event
//         logger.log(&Event::LogRegistered(LogRegisteredEvent {
//             name: entry.name,
//             destination: entry.destination,
//         }))?;
//     }

//     Ok(())
// }

// /// The parameter type for the contract functions `publicKeyOf/noneOf`. A query
// /// for the public key/nonce of a given account.
// #[derive(Serialize, SchemaType)]
// #[concordium(transparent)]
// pub struct ImportContractsParam {
//     /// List of
//     #[concordium(size_length = 2)]
//     pub entries: Vec<ContractAddress>,
// }

// ///
// #[receive(
//     contract = "staking_bank",
//     name = "importContracts",
//     parameter = "ImportContractsParam",
//     error = "CustomContractError",
//     enable_logger,
//     mutable
// )]
// fn import_contracts<S: HasStateApi>(
//     ctx: &impl HasReceiveContext,
//     host: &mut impl HasHost<State<S>, StateApiType = S>,
//     logger: &mut impl HasLogger,
// ) -> Result<(), CustomContractError> {
//     ensure_eq!(
//         ctx.sender(),
//         host.state().owner,
//         CustomContractError::UnauthorizedAccount
//     );

//     let import_contracts: ImportContractsParam = ctx.parameter_cursor().get()?;

//     for contract_address in import_contracts.entries {
//         let name = host.invoke_contract_read_only(
//             &contract_address,
//             &Parameter::empty(),
//             EntrypointName::new_unchecked("getName"),
//             Amount::zero(),
//         )?;

//         let name = name
//             .ok_or(CustomContractError::InvokeContractError)?
//             .get()?;

//         host.state_mut().registry.insert(name, contract_address);

//         // Log LogRegistered event
//         logger.log(&Event::LogRegistered(LogRegisteredEvent {
//             name,
//             destination: contract_address,
//         }))?;
//     }

//     Ok(())
// }

// ///
// #[receive(
//     contract = "staking_bank",
//     name = "atomicUpdate",
//     parameter = "ImportContractsParam",
//     error = "CustomContractError",
//     enable_logger,
//     mutable
// )]
// fn atomic_update<S: HasStateApi>(
//     ctx: &impl HasReceiveContext,
//     host: &mut impl HasHost<State<S>, StateApiType = S>,
//     logger: &mut impl HasLogger,
// ) -> Result<(), CustomContractError> {
//     ensure_eq!(
//         ctx.sender(),
//         host.state().owner,
//         CustomContractError::UnauthorizedAccount
//     );

//     let import_contracts: ImportContractsParam = ctx.parameter_cursor().get()?;

//     for new_contract in import_contracts.entries {
//         // register() hook
//         host.invoke_contract(
//             &new_contract,
//             &Parameter::empty(),
//             EntrypointName::new_unchecked("register"),
//             Amount::zero(),
//         )?;

//         let name = host.invoke_contract_read_only(
//             &new_contract,
//             &Parameter::empty(),
//             EntrypointName::new_unchecked("getName"),
//             Amount::zero(),
//         )?;

//         let name = name
//             .ok_or(CustomContractError::InvokeContractError)?
//             .get()?;

//         let old_contract = host.state_mut().registry.insert(name, new_contract);

//         // Only if another `old_contract` was already registered, execute the `unregister` hook.
//         if let Some(old_contract) = old_contract {
//             // unRegister() hook
//             host.invoke_contract(
//                 &old_contract,
//                 &Parameter::empty(),
//                 EntrypointName::new_unchecked("unregister"),
//                 Amount::zero(),
//             )?;
//         }

//         // Log LogRegistered event
//         logger.log(&Event::LogRegistered(LogRegisteredEvent {
//             name,
//             destination: new_contract,
//         }))?;
//     }

//     Ok(())
// }

// /// View function that returns contract_address from key hash.
// #[receive(
//     contract = "staking_bank",
//     name = "getAddress",
//     parameter = "HashSha2256",
//     return_value = "ContractAddress"
// )]
// fn get_address<S: HasStateApi>(
//     ctx: &impl HasReceiveContext,
//     host: &impl HasHost<State<S>, StateApiType = S>,
// ) -> ReceiveResult<ContractAddress> {
//     let key_hash: HashSha2256 = ctx.parameter_cursor().get()?;

//     let contract_address = host
//         .state()
//         .registry
//         .get(&key_hash)
//         .map(|s| *s)
//         .ok_or(CustomContractError::NameNotRegistered)?;

//     Ok(contract_address)
// }

// /// View function that returns contract_address from key string.
// #[receive(
//     contract = "staking_bank",
//     name = "getAddressByString",
//     parameter = "String",
//     return_value = "ContractAddress",
//     crypto_primitives
// )]
// fn get_address_by_string<S: HasStateApi>(
//     ctx: &impl HasReceiveContext,
//     host: &impl HasHost<State<S>, StateApiType = S>,
//     crypto_primitives: &impl HasCryptoPrimitives,
// ) -> ReceiveResult<ContractAddress> {
//     let key: String = ctx.parameter_cursor().get()?;

//     // Calculate the message hash.
//     let key_hash = crypto_primitives.hash_sha2_256(key.as_bytes()).0;

//     let contract_address = host
//         .state()
//         .registry
//         .get(&HashSha2256(key_hash))
//         .map(|s| *s)
//         .ok_or(CustomContractError::NameNotRegistered)?;

//     Ok(contract_address)
// }

// /// View function that hash from a key string.
// #[receive(
//     contract = "staking_bank",
//     name = "stringToBytes32",
//     parameter = "String",
//     return_value = "HashSha2256",
//     crypto_primitives
// )]
// fn string_to_bytes32<S: HasStateApi>(
//     ctx: &impl HasReceiveContext,
//     _host: &impl HasHost<State<S>, StateApiType = S>,
//     crypto_primitives: &impl HasCryptoPrimitives,
// ) -> ReceiveResult<HashSha2256> {
//     let key: String = ctx.parameter_cursor().get()?;

//     // Calculate the message hash.
//     let key_hash = crypto_primitives.hash_sha2_256(key.as_bytes()).0;

//     Ok(HashSha2256(key_hash))
// }

#[concordium_cfg_test]
mod tests {
    use super::*;
    use test_infrastructure::*;

    #[concordium_test]
    /// Test that initializing the contract succeeds with some state.
    fn test_init() {
        let mut ctx = TestInitContext::empty();

        let mut state_builder = TestStateBuilder::new();

        let parameter_bytes = to_bytes(&InitContractsParam {
            validators_count: U256Wrapper(U256::from_dec_str("15").unwrap()),
        });
        ctx.set_parameter(&parameter_bytes);

        let state_result = init(&ctx, &mut state_builder);
        let initial_state = state_result.expect_report("Contract initialization results in error");

        let ctx = TestReceiveContext::empty();

        let host = TestHost::new(initial_state, state_builder);

        // Call the contract function.
        let state = view(&ctx, &host);

        println!("{:?}", state.unwrap());
    }
}

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
