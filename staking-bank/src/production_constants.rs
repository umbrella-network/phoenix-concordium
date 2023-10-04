#![cfg_attr(not(feature = "std"), no_std)]

use crate::{CustomContractError, State};
use concordium_std::*;

// Production constants and functions

pub(crate) const VALIDATOR_0: PublicKeyEd25519 = PublicKeyEd25519([0u8; 32]);
pub(crate) const VALIDATOR_1: PublicKeyEd25519 = PublicKeyEd25519([1u8; 32]);
pub(crate) const VALIDATOR_2: PublicKeyEd25519 = PublicKeyEd25519([2u8; 32]);
pub(crate) const VALIDATOR_3: PublicKeyEd25519 = PublicKeyEd25519([3u8; 32]);
pub(crate) const VALIDATOR_4: PublicKeyEd25519 = PublicKeyEd25519([4u8; 32]);
pub(crate) const VALIDATOR_5: PublicKeyEd25519 = PublicKeyEd25519([5u8; 32]);
pub(crate) const VALIDATOR_6: PublicKeyEd25519 = PublicKeyEd25519([6u8; 32]);
pub(crate) const VALIDATOR_7: PublicKeyEd25519 = PublicKeyEd25519([7u8; 32]);
pub(crate) const VALIDATOR_8: PublicKeyEd25519 = PublicKeyEd25519([8u8; 32]);
pub(crate) const VALIDATOR_9: PublicKeyEd25519 = PublicKeyEd25519([9u8; 32]);
pub(crate) const VALIDATOR_10: PublicKeyEd25519 = PublicKeyEd25519([10u8; 32]);
pub(crate) const VALIDATOR_11: PublicKeyEd25519 = PublicKeyEd25519([11u8; 32]);
pub(crate) const VALIDATOR_12: PublicKeyEd25519 = PublicKeyEd25519([12u8; 32]);
pub(crate) const VALIDATOR_13: PublicKeyEd25519 = PublicKeyEd25519([13u8; 32]);
pub(crate) const VALIDATOR_14: PublicKeyEd25519 = PublicKeyEd25519([14u8; 32]);

/// The number of validators.
pub(crate) const NUMBER_OF_VALIDATORS: u8 = 15;
/// total supply = number_of_validators * ONE.
pub(crate) const TOTAL_SUPPLY: u64 = 15 * 1000000000000000000u64;

/// Internal function that returns a boolean if the given address is a validator.
pub(crate) fn is_validator(validator: PublicKeyEd25519) -> bool {
    addresses().contains(&validator)
}

/// Internal function that returns all validators.
pub(crate) fn addresses() -> [PublicKeyEd25519; 15] {
    [
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

/// View function that returns validator's URL (as well as the inputted account address). The function throws an error if the address is not a validator.
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
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

/// View function that returns all validator addresses.
#[receive(
    contract = "staking_bank",
    name = "getAddresses",
    return_value = "[PublicKeyEd25519;15]"
)]
pub(crate) fn get_addresses<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<[PublicKeyEd25519; 15]> {
    Ok(addresses())
}
