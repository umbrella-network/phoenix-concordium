use std::collections::BTreeMap;

use concordium_smart_contract_testing::AccountAccessStructure;
use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use concordium_std::{
    AccountSignatures, CredentialSignatures, PublicKeyEd25519, SignatureEd25519, Timestamp,
};
use registry::{AtomicUpdateParam, ImportContractsParam};
use sha256::digest;
use umbrella_feeds::{InitParamsUmbrellaFeeds, Message, PriceData, UpdateParams};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const KEY_HASH: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

const KEY_HASH_2: HashSha2256 = HashSha2256([
    120, 14, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

const SIGNATURE_1: SignatureEd25519 = SignatureEd25519([
    146, 192, 138, 73, 29, 77, 21, 81, 107, 212, 43, 14, 39, 5, 101, 113, 227, 126, 102, 72, 118,
    190, 52, 40, 232, 168, 9, 129, 61, 197, 193, 173, 37, 112, 55, 88, 153, 214, 86, 66, 193, 128,
    240, 165, 139, 64, 18, 217, 31, 155, 235, 89, 82, 197, 254, 23, 162, 27, 132, 179, 245, 52, 58,
    3,
]);

const SIGNATURE_2: SignatureEd25519 = SignatureEd25519([
    210, 147, 206, 186, 190, 79, 103, 150, 226, 155, 7, 134, 148, 254, 142, 45, 185, 154, 12, 169,
    237, 127, 80, 144, 33, 221, 117, 32, 0, 154, 58, 225, 132, 53, 105, 193, 160, 166, 76, 149,
    249, 135, 169, 164, 143, 166, 107, 96, 54, 51, 43, 44, 0, 6, 53, 76, 194, 34, 128, 243, 21,
    156, 91, 14,
]);

const SIGNATURE_TWO_PRICE_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    203, 254, 168, 196, 227, 90, 218, 84, 95, 78, 38, 175, 98, 83, 24, 59, 173, 134, 121, 108, 19,
    114, 161, 161, 180, 184, 46, 89, 22, 43, 93, 62, 171, 86, 179, 209, 11, 179, 188, 70, 228, 47,
    249, 165, 85, 15, 110, 193, 18, 204, 204, 124, 130, 147, 109, 13, 55, 119, 72, 80, 149, 66,
    178, 6,
]);

const SIGNATURE_TWO_PRICE_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    134, 188, 255, 31, 71, 246, 166, 167, 62, 172, 245, 183, 69, 55, 23, 180, 154, 169, 33, 47,
    233, 163, 152, 61, 202, 33, 152, 123, 180, 207, 44, 120, 156, 185, 141, 60, 121, 162, 48, 14,
    79, 69, 220, 3, 136, 198, 17, 182, 33, 53, 145, 117, 65, 82, 1, 10, 70, 187, 4, 152, 105, 238,
    123, 1,
]);

const SIGNATURE_ETH_CCD_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    247, 122, 19, 218, 9, 77, 56, 34, 50, 44, 178, 93, 116, 82, 78, 117, 197, 113, 132, 100, 48,
    144, 40, 108, 171, 20, 163, 38, 7, 18, 21, 213, 26, 4, 237, 91, 194, 195, 42, 254, 11, 118, 26,
    252, 64, 30, 119, 194, 70, 118, 36, 36, 155, 112, 70, 203, 117, 89, 122, 234, 80, 227, 103, 11,
]);

const SIGNATURE_ETH_CCD_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    69, 94, 133, 241, 119, 150, 94, 22, 187, 40, 182, 90, 236, 131, 124, 222, 57, 144, 203, 9, 26,
    64, 60, 39, 130, 244, 200, 243, 143, 216, 227, 222, 67, 67, 121, 49, 15, 115, 209, 177, 128,
    153, 145, 29, 85, 116, 91, 71, 248, 215, 200, 16, 66, 17, 226, 193, 234, 8, 17, 95, 143, 240,
    136, 10,
]);

// Private key: 8ECA45107A878FB879B84401084B55AD4919FC0F7D14E8915D8A5989B1AE1C01
const PUBLIC_KEY_SIGNER_1: [u8; 32] = [
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
];

// Private key: 12827BE279AA7DB7400E9322824CF3C7D5D599005836FDA506351B9B340838A9
const PUBLIC_KEY_SIGNER_2: [u8; 32] = [
    217, 108, 75, 18, 24, 234, 126, 194, 15, 70, 4, 214, 194, 240, 47, 163, 243, 107, 81, 132, 67,
    243, 162, 209, 78, 136, 94, 127, 247, 21, 222, 221,
];

const SIGNER_1: AccountAddress = AccountAddress([1u8; 32]);
const SIGNER_2: AccountAddress = AccountAddress([2u8; 32]);

fn setup_chain_and_contract() -> (
    Chain,
    ContractInitSuccess,
    ContractInitSuccess,
    ContractInitSuccess,
) {
    let mut chain = Chain::new();

    let balance = AccountBalance {
        total: ACC_INITIAL_BALANCE,
        staked: Amount::zero(),
        locked: Amount::zero(),
    };

    // Creating signer_1's keys

    let mut inner_key_map_signer_1: BTreeMap<KeyIndex, VerifyKey> = BTreeMap::new();

    inner_key_map_signer_1.insert(
        KeyIndex(0u8),
        VerifyKey::Ed25519VerifyKey(
            ed25519_dalek::PublicKey::from_bytes(&PUBLIC_KEY_SIGNER_1)
                .expect("Should be able to create public key"),
        ),
    );

    let credential_public_keys_signer_1 = CredentialPublicKeys {
        keys: inner_key_map_signer_1,
        threshold: SignatureThreshold::ONE,
    };

    let mut key_map_signer_1: BTreeMap<CredentialIndex, CredentialPublicKeys> = BTreeMap::new();
    key_map_signer_1.insert(
        CredentialIndex { index: 0u8 },
        credential_public_keys_signer_1,
    );

    let keys_signer_1 = AccountAccessStructure {
        keys: key_map_signer_1,
        threshold: AccountThreshold::ONE,
    };

    chain.create_account(Account::new_with_keys(SIGNER_1, balance, keys_signer_1));

    // Creating signer_2's keys

    let mut inner_key_map_signer_2: BTreeMap<KeyIndex, VerifyKey> = BTreeMap::new();

    inner_key_map_signer_2.insert(
        KeyIndex(0u8),
        VerifyKey::Ed25519VerifyKey(
            ed25519_dalek::PublicKey::from_bytes(&PUBLIC_KEY_SIGNER_2)
                .expect("Should be able to create public key"),
        ),
    );

    let credential_public_keys_signer_2 = CredentialPublicKeys {
        keys: inner_key_map_signer_2,
        threshold: SignatureThreshold::ONE,
    };

    let mut key_map_signer_2: BTreeMap<CredentialIndex, CredentialPublicKeys> = BTreeMap::new();
    key_map_signer_2.insert(
        CredentialIndex { index: 0u8 },
        credential_public_keys_signer_2,
    );

    let keys_signer_2 = AccountAccessStructure {
        keys: key_map_signer_2,
        threshold: AccountThreshold::ONE,
    };
    chain.create_account(Account::new_with_keys(SIGNER_2, balance, keys_signer_2));

    // Creating contract owner's keys

    chain.create_account(Account::new(ACC_ADDR_OWNER, ACC_INITIAL_BALANCE));

    // Deploying 'registry' contract

    let deployment_registry = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../registry/registry.wasm.v1")
                .expect("`Umbrella_feeds.wasm.v1` module should be loaded"),
        )
        .expect("`Umbrella_feeds.wasm.v1` deployment should always succeed");

    let initialization_registry = chain
        .contract_init(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Energy::from(10000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment_registry.module_reference,
                init_name: OwnedContractName::new_unchecked("init_registry".to_string()),
                param: OwnedParameter::empty(),
            },
        )
        .expect("Initialization of `registry` should always succeed");

    // Deploying 'staking bank' contract

    let deployment_staking_bank = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../staking-bank/staking_bank.wasm.v1")
                .expect("`staking_bank.wasm.v1` module should be loaded"),
        )
        .expect("`staking_bank.wasm.v1` deployment should always succeed");

    let initialization_staking_bank = chain
        .contract_init(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Energy::from(10000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment_staking_bank.module_reference,
                init_name: OwnedContractName::new_unchecked("init_staking_bank".to_string()),
                param: OwnedParameter::empty(),
            },
        )
        .expect("Initialization of `staking_bank` should always succeed");

    // Deploy 'umbrella_feeds' contract

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./umbrella_feeds.wasm.v1")
                .expect("`Umbrella_feeds.wasm.v1` module should be loaded"),
        )
        .expect("`Umbrella_feeds.wasm.v1` deployment should always succeed");

    let input_parameter_2 = InitParamsUmbrellaFeeds {
        registry: initialization_registry.contract_address,
        required_signatures: 2,
        staking_bank: initialization_staking_bank.contract_address,
        decimals: 4,
    };

    let initialization_umbrella_feeds = chain
        .contract_init(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Energy::from(10000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment.module_reference,
                init_name: OwnedContractName::new_unchecked("init_umbrella_feeds".to_string()),
                param: OwnedParameter::from_serial(&input_parameter_2)
                    .expect("`InitContractsParam` should be a valid inut parameter"),
            },
        )
        .expect("Initialization of `umbrella_feeds` should always succeed");

    (
        chain,
        initialization_umbrella_feeds,
        initialization_registry,
        initialization_staking_bank,
    )
}

#[test]
fn test_init() {
    let (
        chain,
        initialization_umbrella_feeds,
        _initialization_registry,
        _initialization_staking_bank,
    ) = setup_chain_and_contract();

    // Checking getChainId

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getChainId".to_string(),
                ),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query");

    let value: u16 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 49228);

    // Checking DECIMALS

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.DECIMALS".to_string(),
                ),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query");

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 4);
}

/// Test updating the price feed with two signers and two price feeds.
#[test]
fn test_update_two_price_feeds() {
    let (
        mut chain,
        initialization_umbrella_feeds,
        _initialization_registry,
        _initialization_staking_bank,
    ) = setup_chain_and_contract();

    let price_data_1 = PriceData {
        data: 7,
        heartbeat: 12,
        timestamp: Timestamp::from_timestamp_millis(9),
        price: 4,
    };

    let price_data_2 = PriceData {
        data: 73,
        heartbeat: 12342,
        timestamp: Timestamp::from_timestamp_millis(239),
        price: 44,
    };

    // Creating signer_1's signature map

    let mut inner_signature_map = BTreeMap::new();
    inner_signature_map.insert(
        0u8,
        concordium_std::Signature::Ed25519(SIGNATURE_TWO_PRICE_FEEDS_1),
    );

    let mut signature_map = BTreeMap::new();
    signature_map.insert(
        0u8,
        CredentialSignatures {
            sigs: inner_signature_map,
        },
    );

    // Creating signer_2's signature map

    let mut inner_signature_map_signer_2 = BTreeMap::new();
    inner_signature_map_signer_2.insert(
        0u8,
        concordium_std::Signature::Ed25519(SIGNATURE_TWO_PRICE_FEEDS_2),
    );

    let mut signature_map_signer_2 = BTreeMap::new();
    signature_map_signer_2.insert(
        0u8,
        CredentialSignatures {
            sigs: inner_signature_map_signer_2,
        },
    );

    // Creating input parameter for pice data update

    let update_param = UpdateParams {
        signers_and_signatures: vec![
            (
                SIGNER_1,
                AccountSignatures {
                    sigs: signature_map,
                },
            ),
            (
                SIGNER_2,
                AccountSignatures {
                    sigs: signature_map_signer_2,
                },
            ),
        ],
        message: Message {
            timestamp: Timestamp::from_timestamp_millis(10000000000),
            contract_address: initialization_umbrella_feeds.contract_address,
            chain_id: 49228,
            price_feed: vec![(KEY_HASH, price_data_1), (KEY_HASH_2, price_data_2)],
        },
    };

    // Checking message hash to be signed

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.viewMessageHash".to_string(),
                ),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query messageHash");

    let message_hashes: Vec<[u8; 32]> =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    for (i, message_hash) in message_hashes.iter().enumerate() {
        println!(
            "Signer {} sign this message hash: {}",
            i,
            HashSha2256(*message_hash)
        );
    }

    let signature:SignatureEd25519 = "86BCFF1F47F6A6A73EACF5B7453717B49AA9212FE9A3983DCA21987BB4CF2C789CB98D3C79A2300E4F45DC0388C611B6213591754152010A46BB049869EE7B01".parse().unwrap();
    println!("Signature: {:?}", signature.0);

    let public_key: PublicKeyEd25519 =
        "D96C4B1218EA7EC20F4604D6C2F02FA3F36B518443F3A2D14E885E7FF715DEDD"
            .parse()
            .unwrap();
    println!("Public key: {:?}", public_key.0);

    // Updating price data in contract

    let _update = chain
        .contract_update(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.update".to_string()),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to update operator with permit");

    // Checking price data was updated correctly in contract

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getManyPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&vec![KEY_HASH, KEY_HASH_2])
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getManyPriceData");

    let stored_price_data: Vec<PriceData> =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, vec![price_data_1, price_data_2]);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getManyPriceDataRaw".to_string(),
                ),
                message: OwnedParameter::from_serial(&vec![KEY_HASH, KEY_HASH_2])
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getManyPriceData");

    let stored_price_data: Vec<PriceData> =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, vec![price_data_1, price_data_2]);
}

/// Test updating the price feed with two signer and one price feed.
#[test]
fn test_update_price_feed() {
    let (
        mut chain,
        initialization_umbrella_feeds,
        _initialization_registry,
        _initialization_staking_bank,
    ) = setup_chain_and_contract();

    let price_data = PriceData {
        data: 7,
        heartbeat: 12,
        timestamp: Timestamp::from_timestamp_millis(9),
        price: 4,
    };

    // Creating signer_1's signature map

    let mut inner_signature_map = BTreeMap::new();
    inner_signature_map.insert(0u8, concordium_std::Signature::Ed25519(SIGNATURE_1));

    let mut signature_map = BTreeMap::new();
    signature_map.insert(
        0u8,
        CredentialSignatures {
            sigs: inner_signature_map,
        },
    );

    // Creating signer_2's signature map

    let mut inner_signature_map_signer_2 = BTreeMap::new();
    inner_signature_map_signer_2.insert(0u8, concordium_std::Signature::Ed25519(SIGNATURE_2));

    let mut signature_map_signer_2 = BTreeMap::new();
    signature_map_signer_2.insert(
        0u8,
        CredentialSignatures {
            sigs: inner_signature_map_signer_2,
        },
    );

    // Creating input parameter for pice data update

    let update_param = UpdateParams {
        signers_and_signatures: vec![
            (
                SIGNER_1,
                AccountSignatures {
                    sigs: signature_map,
                },
            ),
            (
                SIGNER_2,
                AccountSignatures {
                    sigs: signature_map_signer_2,
                },
            ),
        ],
        message: Message {
            timestamp: Timestamp::from_timestamp_millis(10000000000),
            contract_address: initialization_umbrella_feeds.contract_address,
            chain_id: 49228,
            price_feed: vec![(KEY_HASH, price_data)],
        },
    };

    // Checking message hash to be signed

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.viewMessageHash".to_string(),
                ),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query messageHash");

    let message_hashes: Vec<[u8; 32]> =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    for (i, message_hash) in message_hashes.iter().enumerate() {
        println!(
            "Signer {} sign this message hash: {}",
            i,
            HashSha2256(*message_hash)
        );
    }

    let signature:SignatureEd25519 = "D293CEBABE4F6796E29B078694FE8E2DB99A0CA9ED7F509021DD7520009A3AE1843569C1A0A64C95F987A9A48FA66B6036332B2C0006354CC22280F3159C5B0E".parse().unwrap();
    println!("Signature: {:?}", signature.0);

    let public_key: PublicKeyEd25519 =
        "D96C4B1218EA7EC20F4604D6C2F02FA3F36B518443F3A2D14E885E7FF715DEDD"
            .parse()
            .unwrap();
    println!("Public key: {:?}", public_key.0);

    // Updating price data in contract

    let _update = chain
        .contract_update(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.update".to_string()),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to update operator with permit");

    // Checking price data was updated correctly in contract with various getter functions.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.prices".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query prices");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query prices");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getPrice".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query prices");

    let stored_price_data: u128 =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data.price);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getPriceTimestamp".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query timestam");

    let stored_price_data: Timestamp =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data.timestamp);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getPriceTimestampHeartbeat".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query timestam");

    let stored_price_data: (u128, Timestamp, u64) =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        stored_price_data,
        (price_data.price, price_data.timestamp, price_data.heartbeat)
    );
}

/// Test updating the price feed with the `ETH-CCD` feed name.
#[test]
fn test_update_price_feed_and_check_price_via_feed_name() {
    let (
        mut chain,
        initialization_umbrella_feeds,
        _initialization_registry,
        _initialization_staking_bank,
    ) = setup_chain_and_contract();

    let price_data = PriceData {
        data: 7,
        heartbeat: 12,
        timestamp: Timestamp::from_timestamp_millis(9),
        price: 4,
    };

    // Creating signer_1's signature map

    let mut inner_signature_map = BTreeMap::new();
    inner_signature_map.insert(
        0u8,
        concordium_std::Signature::Ed25519(SIGNATURE_ETH_CCD_FEEDS_1),
    );

    let mut signature_map = BTreeMap::new();
    signature_map.insert(
        0u8,
        CredentialSignatures {
            sigs: inner_signature_map,
        },
    );

    // Creating signer_2's signature map

    let mut inner_signature_map_signer_2 = BTreeMap::new();
    inner_signature_map_signer_2.insert(
        0u8,
        concordium_std::Signature::Ed25519(SIGNATURE_ETH_CCD_FEEDS_2),
    );

    let mut signature_map_signer_2 = BTreeMap::new();
    signature_map_signer_2.insert(
        0u8,
        CredentialSignatures {
            sigs: inner_signature_map_signer_2,
        },
    );

    // Creating input parameter for pice data update

    let update_param = UpdateParams {
        signers_and_signatures: vec![
            (
                SIGNER_1,
                AccountSignatures {
                    sigs: signature_map,
                },
            ),
            (
                SIGNER_2,
                AccountSignatures {
                    sigs: signature_map_signer_2,
                },
            ),
        ],
        message: Message {
            timestamp: Timestamp::from_timestamp_millis(10000000000),
            contract_address: initialization_umbrella_feeds.contract_address,
            chain_id: 49228,
            price_feed: vec![(digest(String::from("ETH-CCD")).parse().unwrap(), price_data)],
        },
    };

    // Checking message hash to be signed

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.viewMessageHash".to_string(),
                ),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query messageHash");

    let message_hashes: Vec<[u8; 32]> =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    for (i, message_hash) in message_hashes.iter().enumerate() {
        println!(
            "Signer {} sign this message hash: {}",
            i,
            HashSha2256(*message_hash)
        );
    }

    let signature:SignatureEd25519 = "455E85F177965E16BB28B65AEC837CDE3990CB091A403C2782F4C8F38FD8E3DE434379310F73D1B18099911D55745B47F8D7C8104211E2C1EA08115F8FF0880A".parse().unwrap();
    println!("Signature: {:?}", signature.0);

    let public_key: PublicKeyEd25519 =
        "D96C4B1218EA7EC20F4604D6C2F02FA3F36B518443F3A2D14E885E7FF715DEDD"
            .parse()
            .unwrap();
    println!("Public key: {:?}", public_key.0);

    // Updating price data in contract

    let _update = chain
        .contract_update(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.update".to_string()),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to update operator with permit");

    // Checking price data was updated correctly in contract.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getPriceDataByName".to_string(),
                ),
                message: OwnedParameter::from_serial(&"ETH-CCD")
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query timestam");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);
}

#[test]
fn test_upgrade_without_migration_function() {
    let (
        mut chain,
        initialization_umbrella_feeds,
        initialization_registry,
        _initialization_staking_bank,
    ) = setup_chain_and_contract();

    // Importing umbrella_feeds into the registry contract

    let input_parameter = ImportContractsParam {
        entries: vec![initialization_umbrella_feeds.contract_address],
    };

    let _update = chain.contract_update(
        Signer::with_one_key(), // Used for specifying the number of signatures.
        ACC_ADDR_OWNER,         // Invoker account.
        Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
        Energy::from(10000),    // Maximum energy allowed for the update.
        UpdateContractPayload {
            address: initialization_registry.contract_address, // The contract to update.
            receive_name: OwnedReceiveName::new_unchecked("registry.importContracts".into()), // The receive function to call.
            message: OwnedParameter::from_serial(&input_parameter)
                .expect("`UpgradeParams` should be a valid inut parameter"), // The parameter sent to the contract.
            amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
        },
    );

    // Checking that the contract was registered correctly

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "registry.getAddressByString".to_string(),
                ),
                message: OwnedParameter::from_serial(&String::from("UmbrellaFeeds"))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        contract_address,
        initialization_umbrella_feeds.contract_address
    );

    // Deploying an upgraded umbrella_feeds module.

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./umbrella_feeds_update.wasm.v1")
                .expect("`Contract version2` module should be loaded"),
        )
        .expect("`Contract version2` deployment should always succeed");

    // Upgrading umbrella_feeds contract with the new module reference.

    let input_parameter = AtomicUpdateParam {
        module: deployment.module_reference,
        migrate: None,
        contract_address: initialization_umbrella_feeds.contract_address,
    };

    let _update = chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            ACC_ADDR_OWNER,         // Invoker account.
            Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
            Energy::from(100000),   // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization_registry.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("registry.atomicUpdate".into()), // The receive function to call.
                message: OwnedParameter::from_serial(&input_parameter)
                    .expect("`UpgradeParams` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to update");

    // Checking that the upgrade was successfully by calling a function that only exists in the upgraded `umbrella_feeds` contract.

    let _update = chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            ACC_ADDR_OWNER,         // Invoker account.
            Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
            Energy::from(10000),    // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization_umbrella_feeds.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.DECIMALS_2".into()), // The receive function to call.
                message: OwnedParameter::empty(),
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to update");

    // Checking that the contract was updated in the registry correctly

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "registry.getAddressByString".to_string(),
                ),
                message: OwnedParameter::from_serial(&String::from("UmbrellaFeeds"))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        contract_address,
        initialization_umbrella_feeds.contract_address
    );
}
