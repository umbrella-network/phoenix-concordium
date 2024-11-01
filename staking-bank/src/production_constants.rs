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

// https://umbrella.crazywhale.es: 3LZsQcLBRtgatsoYgafj2TjyYMJtKYJYzDLZLaDAnHyVG1NYEr
pub(crate) const VALIDATOR_4: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('763003106b847368c25383544a55181d92df304af4ca322092ae83c8c75d9392', 'hex').toJSON().data
    118, 48, 3, 16, 107, 132, 115, 104, 194, 83, 131, 84, 74, 85, 24, 29, 146, 223, 48, 74, 244,
    202, 50, 32, 146, 174, 131, 200, 199, 93, 147, 146,
]);

// https://umbrella-node.gateomega.com: 3JySC2orV3VGYABZ7HRkvZFbb7bt8MYZF7PWBdt4gPqFsNjaZ1
pub(crate) const VALIDATOR_5: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('f93b11e45121379f3f4cd8e752ddcc1b62314f68941d2bca13251c13deb9be3e', 'hex').toJSON().data
    249, 59, 17, 228, 81, 33, 55, 159, 63, 76, 216, 231, 82, 221, 204, 27, 98, 49, 79, 104, 148, 29,
    43, 202, 19, 37, 28, 19, 222, 185, 190, 62,
]);

// https://umb.anorak.technology: 3HzT78AMa9RhSJxmZGQdrAGQVSvQoyftWNCEHhL85XU1zqRvjc
pub(crate) const VALIDATOR_6: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('51b6e9418afbc19cb92f97b038cc4924e5e46176dcd63e3f7f748919183170c9', 'hex').toJSON().data
    81, 182, 233, 65, 138, 251, 193, 156, 185, 47, 151, 176, 56, 204, 73, 36, 229, 228, 97, 118,
    220, 214, 62, 63, 127, 116, 137, 25, 24, 49, 112, 201,
]);

// https://umbrella.validator.infstones.io: 4neQ1FMr4EZowt3jfPQQTTY2UUHmg6rvsRqjwdLDAAT34d7ajR
pub(crate) const VALIDATOR_7: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('71192110f0fe4b179174bd1b6ebf61ac9e38c75b969970d3bca5836cde02f729', 'hex').toJSON().data
    113, 25, 33, 16, 240, 254, 75, 23, 145, 116, 189, 27, 110, 191, 97, 172, 158, 56, 199, 91, 150,
    153, 112, 211, 188, 165, 131, 108, 222, 2, 247, 41,
]);

// https://umb.hashkey.cloud: 31wMWQJpoL1TinXZDUTkwvXGF1ptqvXAqjSV4jjnNrEUiw5Y36
pub(crate) const VALIDATOR_8: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('b2c969aa57cfc873aadd7b89436f69cee0131adf30f5afa968388e4f6cbf93d1', 'hex').toJSON().data
    178, 201, 105, 170, 87, 207, 200, 115, 170, 221, 123, 137, 67, 111, 105, 206, 224, 19, 26, 223,
    48, 245, 175, 169, 104, 56, 142, 79, 108, 191, 147, 209,
]);

// http://umbrella.staking4all.org:3000: 4T3tGmdHgBCkDqLfTjgkPYLWYMKQ1AWRyxemD4LwoPKDQ77eRm
pub(crate) const VALIDATOR_9: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('aaaacf4ba01fd692b01dfb639db8511a06d954b6ed6549609e3d28c42ad69840', 'hex').toJSON().data
    170, 170, 207, 75, 160, 31, 214, 146, 176, 29, 251, 99, 157, 184, 81, 26, 6, 217, 84, 182, 237,
    101, 73, 96, 158, 61, 40, 196, 42, 214, 152, 64,
]);

// http://5.161.78.230:3000: 3bnt2C49FFgsDF75kNfxZzqLN2yqvTJcxZK5xgh6oU1952y43Z
pub(crate) const VALIDATOR_10: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('fca041dbd365e9a2400b5d43322a3bb41710f9aa0fc0586856c9d02eea607a66', 'hex').toJSON().data
    252, 160, 65, 219, 211, 101, 233, 162, 64, 11, 93, 67, 50, 42, 59, 180, 23, 16, 249, 170, 15,
    192, 88, 104, 86, 201, 208, 46, 234, 96, 122, 102,
]);

// http://5.161.78.230:3000: 3JN1nQhPtot87DxHpLMC6MxXdu5emb6SKEKikHErQx3aQcJp6V
pub(crate) const VALIDATOR_11: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('512e9f39e027f3ee53aa35b9bc2c8dfc03e6ce34e0fea9606d0cbf4d0506b1aa', 'hex').toJSON().data
    81, 46, 159, 57, 224, 39, 243, 238, 83, 170, 53, 185, 188, 44, 141, 252, 3, 230, 206, 52, 224,
    254, 169, 96, 109, 12, 191, 77, 5, 6, 177, 170,
]);

// https://rpc.urbanhq.net: 3QBZ4utQAthJAAvBi5ezEn47yyn4EvDnP4qW9km4Jh6PSSDvBw
pub(crate) const VALIDATOR_12: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('13496eb6bdd6e5c4952a8807d2940c4c95f107fd8141de4afbd473d600003c9a', 'hex').toJSON().data
    19, 73, 110, 182, 189, 214, 229, 196, 149, 42, 136, 7, 210, 148, 12, 76, 149, 241, 7, 253, 129,
    65, 222, 74, 251, 212, 115, 214, 0, 0, 60, 154,
]);

// https://umbrella-node.ankastake.com: 4YAsJrF8Lx3uYyY27pKVGHSWeYtiqoJLonkKdnE7i6n13ekuek
pub(crate) const VALIDATOR_13: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('a4293385ff685fb0eb5a103ad94f57bea2096a81a221c1e7f00699faf11eab68', 'hex').toJSON().data
    164, 41, 51, 133, 255, 104, 95, 176, 235, 90, 16, 58, 217, 79, 87, 190, 162, 9, 106, 129, 162,
    33, 193, 231, 240, 6, 153, 250, 241, 30, 171, 104,
]);

// https://umbrella.tchambrella.com: 3pgXr9JN5nfmVZdytsBu48ZAKbeXhMS9MqvdpJXe18BJKwVeVv
pub(crate) const VALIDATOR_14: PublicKeyEd25519 = PublicKeyEd25519([
    // Buffer.from('95cf3ad9bbf94326107f636ab34e3a5a3f5427aabf3d95c0f841af94aa38a735', 'hex').toJSON().data
    149, 207, 58, 217, 187, 249, 67, 38, 16, 127, 99, 106, 179, 78, 58, 90, 63, 84, 39, 170, 191,
    61, 149, 192, 248, 65, 175, 148, 170, 56, 167, 53,
]);

type StakingBalanceAmount = u8;

/// The number of validators.
pub(crate) const NUMBER_OF_VALIDATORS: u8 = 15; // #update-count
/// total supply = number_of_validators * ONE.
pub(crate) const TOTAL_SUPPLY: StakingBalanceAmount = 15 * 1u8; // #update-count

/// Internal function that returns a boolean if the given public key is a validator.
pub(crate) fn is_validator(validator: PublicKeyEd25519) -> bool {
    public_keys().contains(&validator)
}

/// Internal function that returns all validators.
/// #update-count
pub(crate) fn public_keys() -> [PublicKeyEd25519; 15] {
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
        VALIDATOR_4 => Ok((id, "https://umbrella.crazywhale.es".to_string())),
        VALIDATOR_5 => Ok((id, "https://umbrella-node.gateomega.com".to_string())),
        VALIDATOR_6 => Ok((id, "https://umb.anorak.technology".to_string())),
        VALIDATOR_7 => Ok((id, "https://umbrella.validator.infstones.io".to_string())),
        VALIDATOR_8 => Ok((id, "https://umb.hashkey.cloud".to_string())),
        VALIDATOR_9 => Ok((id, "http://umbrella.staking4all.org:3000".to_string())),
        VALIDATOR_10 => Ok((id, "http://5.161.78.230:3000".to_string())),
        VALIDATOR_11 => Ok((id, "https://umb-api.staking.rocks".to_string())),
        VALIDATOR_12 => Ok((id, "https://rpc.urbanhq.net".to_string())),
        VALIDATOR_13 => Ok((id, "https://umbrella-node.ankastake.com".to_string())),
        VALIDATOR_14 => Ok((id, "https://umbrella.tchambrella.com".to_string())),
        _ => bail!(CustomContractError::NotValidator.into()),
    }
}

/// View function that returns all validators' public keys.
/// #update-count in method and in decorator
#[receive(
    contract = "staking_bank",
    name = "getPublicKeys",
    return_value = "[PublicKeyEd25519;15]"
)]
pub(crate) fn get_public_keys<S: HasStateApi>(
    _ctx: &impl HasReceiveContext,
    _host: &impl HasHost<State, StateApiType = S>,
) -> ReceiveResult<[PublicKeyEd25519; 15]> {
    Ok(public_keys())
}
