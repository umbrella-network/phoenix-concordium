#![cfg_attr(not(feature = "std"), no_std)]

use crate::{CustomContractError, State};
use concordium_std::*;

// Sandbox constants and functions

pub(crate) const VALIDATOR_0: AccountAddress = AccountAddress([0u8; 32]);
pub(crate) const VALIDATOR_1: AccountAddress = AccountAddress([1u8; 32]);

/// The number of validators.
pub(crate) const NUMBER_OF_VALIDATORS: u8 = 2;
/// total supply = number_of_validators * ONE.
pub(crate) const TOTAL_SUPPLY: u64 = 2 * 1000000000000000000u64;

/// Internal function that returns a boolean if the given address is a validator.
pub(crate) fn is_validator(validator: AccountAddress) -> bool {
    addresses().contains(&validator)
}

/// Internal function that returns all validators.
pub(crate) fn addresses() -> [AccountAddress; 2] {
    [VALIDATOR_0, VALIDATOR_1]
}

/// View function that returns validator's URL (as well as the inputted account address). The function throws an error if the address is not a validator.
#[receive(
    contract = "staking_bank",
    name = "validators",
    parameter = "AccountAddress",
    return_value = "(AccountAddress,String)"
)]
pub(crate) fn validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<(AccountAddress, String)> {
    let id: AccountAddress = ctx.parameter_cursor().get()?;

    match id {
        VALIDATOR_0 => Ok((id, "https://validator.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.umb.network".to_string())),
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

/// View function that returns all validator addresses.
#[receive(
    contract = "staking_bank",
    name = "getAddresses",
    return_value = "[AccountAddress;2]"
)]
pub(crate) fn get_addresses<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<[AccountAddress; 2]> {
    Ok(addresses())
}
