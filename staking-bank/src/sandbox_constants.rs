#![cfg_attr(not(feature = "std"), no_std)]

use crate::{CustomContractError, State};
use concordium_std::*;

// Sandbox constants and functions

// verifyKey: 31ed9d6a2868aea363942944b0cb3fa823bb09c254868e3793f35115e34befb7
pub(crate) const VALIDATOR_0: PublicKeyEd25519 = PublicKeyEd25519([
    49, 237, 157, 106, 40, 104, 174, 163, 99, 148, 41, 68, 176, 203, 63, 168, 35, 187, 9, 194, 84,
    134, 142, 55, 147, 243, 81, 21, 227, 75, 239, 183,
]);

// verifyKey: 2b9913a3c764fb82539b55da74d01e475c69a41f7c814a9220eb367ba7fbac67
pub(crate) const VALIDATOR_1: PublicKeyEd25519 = PublicKeyEd25519([
    43, 153, 19, 163, 199, 100, 251, 130, 83, 155, 85, 218, 116, 208, 30, 71, 92, 105, 164, 31,
    124, 129, 74, 146, 32, 235, 54, 123, 167, 251, 172, 103,
]);

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
        VALIDATOR_0 => Ok((id, "https://validator.sbx.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.sbx.umb.network".to_string())),
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
