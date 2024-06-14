#![cfg_attr(not(feature = "std"), no_std)]

use crate::{CustomContractError, State};
use concordium_std::*;

// Production constants and functions

// https://validator.umb.network: 46eTEZwu45dFV2ByhWfDh2sNJg2hLHL6bPwaM398NAeJM7TG3L
pub(crate) const VALIDATOR_0: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('c503429376b4f7bb8c68875f1809ed71468eacab0eb8248b00ba0fd52a7443f7', 'hex').toJSON().data
    197, 3, 66, 147, 118, 180, 247, 187, 140, 104, 135, 95, 24, 9, 237, 113, 70, 142, 172, 171, 14,
    184, 36, 139, 0, 186, 15, 213, 42, 116, 67, 247,
]);

// https://validator2.umb.network: 4LpHLhzGAu8Lx6bPCXM8J1Cej7pupa4jsZCC5FdbcvkhA9pbcd
pub(crate) const VALIDATOR_1: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('f43757255869d712a2ecb18331c5fb601ded27b5670ecd1159b19b80e8cce2a5', 'hex').toJSON().data
    244, 55, 87, 37, 88, 105, 215, 18, 162, 236, 177, 131, 49, 197, 251, 96, 29, 237, 39, 181, 103,
    14, 205, 17, 89, 177, 155, 128, 232, 204, 226, 165,
]);

// https://umbrella.artemahr.tech: 3dSqJ6xwyfBUWzYevMZXMQWtYPhsk7FnCt5ZGbG5Uvzb8jm9bm
pub(crate) const VALIDATOR_2: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('b0a6bbd3e9d4ad75ba4fa022ec83574ff5fe5891c2e6bd002414cf8df8889451', 'hex').toJSON().data
    176, 166, 187, 211, 233, 212, 173, 117, 186, 79, 160, 34, 236, 131, 87, 79, 245, 254, 88, 145,
    194, 230, 189, 0, 36, 20, 207, 141, 248, 136, 148, 81,
]);

// https://umb.vtabsolutions.com:3030: 3JoSh3cNK5ypNtu5v9urbi8XFf3YcuHUecWnih7WjkdJNK4URo
pub(crate) const VALIDATOR_3: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('903a2cde70cd0ebd469e330bec41b9e818759fce2eccebe14e6cf991c63ea62a', 'hex').toJSON().data
    144, 58, 44, 222, 112, 205, 14, 189, 70, 158, 51, 11, 236, 65, 185, 232, 24, 117, 159, 206, 46,
    204, 235, 225, 78, 108, 249, 145, 198, 62, 166, 42,
]);

// https://umb.stakers.world - not set
// pub(crate) const VALIDATOR_4: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://umbrella.crazywhale.es: 3LZsQcLBRtgatsoYgafj2TjyYMJtKYJYzDLZLaDAnHyVG1NYEr
pub(crate) const VALIDATOR_5: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('763003106b847368c25383544a55181d92df304af4ca322092ae83c8c75d9392', 'hex').toJSON().data
    118, 48, 3, 16, 107, 132, 115, 104, 194, 83, 131, 84, 74, 85, 24, 29, 146, 223, 48, 74, 244,
    202, 50, 32, 146, 174, 131, 200, 199, 93, 147, 146,
]);

// https://umbrella-node.gateomega.com: 3JySC2orV3VGYABZ7HRkvZFbb7bt8MYZF7PWBdt4gPqFsNjaZ1
pub(crate) const VALIDATOR_6: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('f93b11e45121379f3f4cd8e752ddcc1b62314f68941d2bca13251c13deb9be3e', 'hex').toJSON().data
    249, 59, 17, 228, 81, 33, 55, 159, 63, 76, 216, 231, 82, 221, 204, 27, 98, 49, 79, 104, 148, 29,
    43, 202, 19, 37, 28, 19, 222, 185, 190, 62,
]);

// https://umb.anorak.technology: 3HzT78AMa9RhSJxmZGQdrAGQVSvQoyftWNCEHhL85XU1zqRvjc
// pub(crate) const VALIDATOR_7: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://umbrella.validator.infstones.io: 4neQ1FMr4EZowt3jfPQQTTY2UUHmg6rvsRqjwdLDAAT34d7ajR
// pub(crate) const VALIDATOR_8: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://umb.hashquark.io: 31wMWQJpoL1TinXZDUTkwvXGF1ptqvXAqjSV4jjnNrEUiw5Y36
// pub(crate) const VALIDATOR_9: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// http://umbrella.staking4all.org:3000: 4T3tGmdHgBCkDqLfTjgkPYLWYMKQ1AWRyxemD4LwoPKDQ77eRm
// pub(crate) const VALIDATOR_10: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://umbrella-api.validatrium.club: 4YVKfZTn1Sqim1zG7984zBAmmn7iMjRcBeNEJq2CVVCyQy6joD
pub(crate) const VALIDATOR_11: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('d1cc42366f744101e5196a558744ffeef64ee23d8c3df07ba86250ad27100647', 'hex').toJSON().data
    209, 204, 66, 54, 111, 116, 65, 1, 229, 25, 106, 85, 135, 68, 255, 238, 246, 78, 226, 61, 140,
    61, 240, 123, 168, 98, 80, 173, 39, 16, 6, 71,
]);

// http://5.161.78.230:3000: 3bnt2C49FFgsDF75kNfxZzqLN2yqvTJcxZK5xgh6oU1952y43Z
pub(crate) const VALIDATOR_12: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('fca041dbd365e9a2400b5d43322a3bb41710f9aa0fc0586856c9d02eea607a66', 'hex').toJSON().data
    252, 160, 65, 219, 211, 101, 233, 162, 64, 11, 93, 67, 50, 42, 59, 180, 23, 16, 249, 170, 15,
    192, 88, 104, 86, 201, 208, 46, 234, 96, 122, 102,
]);

// https://umbnode.blockchainliverpool.com:
// pub(crate) const VALIDATOR_13: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://umb-api.staking.rocks: 3JN1nQhPtot87DxHpLMC6MxXdu5emb6SKEKikHErQx3aQcJp6V
// pub(crate) const VALIDATOR_14: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://rpc.urbanhq.net: 3QBZ4utQAthJAAvBi5ezEn47yyn4EvDnP4qW9km4Jh6PSSDvBw
// pub(crate) const VALIDATOR_15: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://umbrella-node.ankastake.com: 4YAsJrF8Lx3uYyY27pKVGHSWeYtiqoJLonkKdnE7i6n13ekuek
pub(crate) const VALIDATOR_16: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('a4293385ff685fb0eb5a103ad94f57bea2096a81a221c1e7f00699faf11eab68', 'hex').toJSON().data
    164, 41, 51, 133, 255, 104, 95, 176, 235, 90, 16, 58, 217, 79, 87, 190, 162, 9, 106, 129, 162,
    33, 193, 231, 240, 6, 153, 250, 241, 30, 171, 104,
]);

// https://umbrella.tchambrella.com: 4WCwpqd85XDYfYgmiFjUvZpopW2bTFv1bVzgEZ8Y3QtnQkQrVN
// pub(crate) const VALIDATOR_17: PublicKeyEd25519 = PublicKeyEd25519([
//     // Buffer.from('', 'hex').toJSON().data
// ]);

// https://umbrella-node.cmt13.eu: 33RbkgWyCRQ8byXx2qitHXk7wgcr8SY4WSSgpX19fBU1wsDuW9
pub(crate) const VALIDATOR_18: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('169067968de918946c352f38389d440504bb3d72b7706fbe86a2cc4dac952d16', 'hex').toJSON().data
    22, 144, 103, 150, 141, 233, 24, 148, 108, 53, 47, 56, 56, 157, 68, 5, 4, 187, 61, 114, 183,
    112, 111, 190, 134, 162, 204, 77, 172, 149, 45, 22,
]);

type StakingBalanceAmount = u8;

/// The number of validators.
pub(crate) const NUMBER_OF_VALIDATORS: u8 = 10;
/// total supply = number_of_validators * ONE.
pub(crate) const TOTAL_SUPPLY: StakingBalanceAmount = 4 * 1u8;

/// Internal function that returns a boolean if the given public key is a validator.
pub(crate) fn is_validator(validator: PublicKeyEd25519) -> bool {
    public_keys().contains(&validator)
}

/// Internal function that returns all validators.
pub(crate) fn public_keys() -> [PublicKeyEd25519; 10] {
    [
        VALIDATOR_0,
        VALIDATOR_1,
        VALIDATOR_2,
        VALIDATOR_3,
        // VALIDATOR_4,
        VALIDATOR_5,
        VALIDATOR_6,
        // VALIDATOR_7,
        // VALIDATOR_8,
        // VALIDATOR_9,
        // VALIDATOR_10,
        VALIDATOR_11,
        VALIDATOR_12,
        // VALIDATOR_13,
        // VALIDATOR_14,
        // VALIDATOR_15,
        VALIDATOR_16,
        // VALIDATOR_17,
        VALIDATOR_18,
    ]
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
        VALIDATOR_2 => Ok((id, "https://umbrella.artemahr.tech".to_string())),
        VALIDATOR_3 => Ok((id, "https://umb.vtabsolutions.com:3030".to_string())),
        // VALIDATOR_4 => Ok((id, "https://umb.stakers.world".to_string())),
        VALIDATOR_5 => Ok((id, "https://umbrella.crazywhale.es".to_string())),
        VALIDATOR_6 => Ok((id, "https://umbrella-node.gateomega.com".to_string())),
        // VALIDATOR_7 => Ok((id, "https://umb.anorak.technology".to_string())),
        // VALIDATOR_8 => Ok((id, "https://umbrella.validator.infstones.io".to_string())),
        // VALIDATOR_9 => Ok((id, "https://umb.hashquark.io".to_string())),
        // VALIDATOR_10 => Ok((id, "http://umbrella.staking4all.org:3000".to_string())),
        VALIDATOR_11 => Ok((id, "https://umbrella-api.validatrium.club".to_string())),
        VALIDATOR_12 => Ok((id, "http://5.161.78.230:3000".to_string())),
        // VALIDATOR_3 => Ok((id, "https://umbnode.blockchainliverpool.com".to_string())),
        // VALIDATOR_14 => Ok((id, "https://umb-api.staking.rocks".to_string())),
        // VALIDATOR_15 => Ok((id, "https://rpc.urbanhq.net".to_string())),
        VALIDATOR_16 => Ok((id, "https://umbrella-node.ankastake.com".to_string())),
        // VALIDATOR_17 => Ok((id, "https://umbrella.tchambrella.com".to_string())),
        VALIDATOR_18 => Ok((id, "https://umbrella-node.cmt13.eu".to_string())),
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

/// View function that returns all validators' public keys.
#[receive(
    contract = "staking_bank",
    name = "getPublicKeys",
    return_value = "[PublicKeyEd25519;10]"
)]
pub(crate) fn get_public_keys<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<[PublicKeyEd25519; 10]> {
    Ok(public_keys())
}
