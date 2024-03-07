#![cfg_attr(not(feature = "std"), no_std)]

use crate::{CustomContractError, State};
use concordium_std::*;

// Development constants and functions

// 77b0d12d7f465f24dd60859154224e49c2585f38e7e550c6ebb04b76a15db317 (public key ???)
pub(crate) const VALIDATOR_0: PublicKeyEd25519 = PublicKeyEd25519([
    119, 176, 209, 45, 127, 70, 95, 36, 221, 96, 133, 145, 84, 34, 78, 73, 194, 88, 95, 56, 231,
    229, 80, 198, 235, 176, 75, 118, 161, 93, 179, 23,
]);

// 6a33d6fe578a70be1c1ac29e5b887c92fca0c44ca7d5c820a6573fc1125fac31 (public key ???)
pub(crate) const VALIDATOR_1: PublicKeyEd25519 = PublicKeyEd25519([
    106, 51, 214, 254, 87, 138, 112, 190, 28, 26, 194, 158, 91, 136, 124, 146, 252, 160, 196, 76,
    167, 213, 200, 32, 166, 87, 63, 193, 18, 95, 172, 49,
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
        VALIDATOR_0 => Ok((id, "https://validator.dev.umb.network".to_string())),
        VALIDATOR_1 => Ok((id, "https://validator2.dev.umb.network".to_string())),
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
