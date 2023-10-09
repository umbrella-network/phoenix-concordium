#![cfg_attr(not(feature = "std"), no_std)]

use crate::{CustomContractError, State};
use concordium_std::*;

// Sandbox constants and functions

pub(crate) const VALIDATOR_0: PublicKeyEd25519 = PublicKeyEd25519([0u8; 32]);
pub(crate) const VALIDATOR_1: PublicKeyEd25519 = PublicKeyEd25519([1u8; 32]);

type StakingBalanceAmount = u8;

/// The number of validators.
pub(crate) const NUMBER_OF_VALIDATORS: u8 = 2;
/// total supply = number_of_validators * ONE.
pub(crate) const TOTAL_SUPPLY: StakingBalanceAmount = 2 * 1u8;

/// Internal function that returns a boolean if the given public key is a validator.
pub(crate) fn is_validator(validator: PublicKeyEd25519) -> bool {
    public_keys().contains(&validator)
}

/// Internal function that returns all validators.
pub(crate) fn public_keys() -> [PublicKeyEd25519; 2] {
    [VALIDATOR_0, VALIDATOR_1]
}

/// View function that returns validator's URL (as well as the inputted public key). The function throws an error if the public key is not a validator.
#[receive(
    contract = "staking_bank",
    name = "validators",
    parameter = "PublicKeyEd25519",
    return_value = "(PublicKeyEd25519,String)"
)]
pub(crate) fn validators<S: HasStateApi>(
    ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<(PublicKeyEd25519, String)> {
    let id: PublicKeyEd25519 = ctx.parameter_cursor().get()?;

    match id {
        VALIDATOR_0 => Ok((id, "https://validator.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.umb.network".to_string())),
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

/// View function that returns all validators' public keys.
#[receive(
    contract = "staking_bank",
    name = "getPublicKeys",
    return_value = "[PublicKeyEd25519;2]"
)]
pub(crate) fn get_public_keys<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<[PublicKeyEd25519; 2]> {
    Ok(public_keys())
}
