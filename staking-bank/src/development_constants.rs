#![cfg_attr(not(feature = "std"), no_std)]

use crate::{CustomContractError, State};
use concordium_std::*;

// Development constants and functions

// ATTENTION: Use a different key in production. The private key is exposed and used for testing here.
// Private key: 8ECA45107A878FB879B84401084B55AD4919FC0F7D14E8915D8A5989B1AE1C01
pub(crate) const VALIDATOR_0: PublicKeyEd25519 = PublicKeyEd25519([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

// ATTENTION: Use a different key in production. The private key is exposed and used for testing here.
// Private key: 12827BE279AA7DB7400E9322824CF3C7D5D599005836FDA506351B9B340838A9
pub(crate) const VALIDATOR_1: PublicKeyEd25519 = PublicKeyEd25519([
    217, 108, 75, 18, 24, 234, 126, 194, 15, 70, 4, 214, 194, 240, 47, 163, 243, 107, 81, 132, 67,
    243, 162, 209, 78, 136, 94, 127, 247, 21, 222, 221,
]);

type StakingBalanceAmount = u64;

/// The number of validators.
pub(crate) const NUMBER_OF_VALIDATORS: u8 = 2;
/// total supply = number_of_validators * ONE.
pub(crate) const TOTAL_SUPPLY: StakingBalanceAmount = 2 * 1000000000000000000u64;

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
    return_value = "[PublicKeyEd25519;22]"
)]
pub(crate) fn get_public_keys<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<[PublicKeyEd25519; 2]> {
    Ok(public_keys())
}
