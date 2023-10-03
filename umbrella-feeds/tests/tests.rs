use std::collections::BTreeMap;

use concordium_smart_contract_testing::AccountAccessStructure;
use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use concordium_std::{
    AccountSignatures, CredentialSignatures, PublicKeyEd25519, SignatureEd25519, Timestamp,
};
use registry::{AtomicUpdateParam, ImportContractsParam};
use umbrella_feeds::{InitParamsUmbrellaFeeds, Message, PriceData, UpdateParams};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const SIGNATURE_1: SignatureEd25519 = SignatureEd25519([
    202, 18, 112, 41, 118, 23, 126, 23, 39, 154, 82, 140, 202, 72, 244, 112, 79, 41, 213, 109, 1,
    6, 231, 160, 164, 18, 156, 15, 103, 58, 55, 48, 54, 74, 188, 72, 220, 150, 152, 226, 152, 134,
    148, 120, 210, 207, 160, 86, 43, 245, 225, 168, 221, 178, 204, 160, 171, 18, 227, 65, 113, 208,
    0, 10,
]);

const SIGNATURE_2: SignatureEd25519 = SignatureEd25519([
    231, 56, 47, 104, 199, 98, 238, 154, 60, 206, 231, 68, 123, 253, 248, 73, 3, 110, 22, 71, 216,
    13, 148, 171, 190, 155, 64, 234, 149, 5, 102, 21, 108, 170, 73, 2, 209, 87, 150, 141, 192, 240,
    238, 113, 252, 64, 158, 16, 53, 240, 106, 197, 177, 196, 207, 55, 11, 228, 13, 79, 253, 121,
    207, 11,
]);

const SIGNATURE_TWO_PRICE_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    127, 244, 115, 84, 34, 88, 195, 207, 121, 52, 117, 113, 167, 181, 20, 4, 7, 61, 151, 134, 191,
    205, 141, 28, 237, 83, 46, 15, 212, 183, 0, 91, 197, 12, 112, 195, 24, 151, 191, 139, 147, 34,
    30, 35, 53, 247, 165, 15, 100, 186, 111, 144, 138, 184, 128, 224, 180, 169, 185, 46, 200, 237,
    220, 14,
]);

const SIGNATURE_TWO_PRICE_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    173, 210, 22, 211, 117, 57, 237, 21, 176, 109, 13, 155, 203, 235, 132, 132, 166, 79, 111, 186,
    243, 246, 30, 198, 77, 169, 93, 198, 183, 175, 91, 19, 22, 198, 68, 8, 203, 93, 203, 204, 93,
    17, 14, 168, 14, 49, 185, 37, 185, 46, 182, 146, 38, 107, 72, 244, 40, 146, 26, 17, 179, 76,
    146, 5,
]);

const SIGNATURE_ETH_CCD_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    178, 135, 17, 215, 211, 254, 210, 156, 248, 73, 206, 80, 22, 82, 47, 163, 191, 177, 206, 16,
    54, 34, 127, 139, 173, 89, 35, 189, 110, 200, 144, 13, 104, 141, 24, 43, 28, 121, 195, 24, 9,
    144, 202, 243, 209, 212, 95, 121, 214, 234, 249, 133, 234, 18, 58, 9, 26, 146, 150, 224, 129,
    90, 55, 2,
]);

const SIGNATURE_ETH_CCD_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    80, 166, 224, 182, 42, 7, 152, 100, 155, 158, 163, 78, 233, 243, 143, 246, 170, 20, 73, 238,
    248, 176, 252, 78, 108, 237, 170, 172, 206, 113, 58, 106, 154, 34, 157, 194, 196, 189, 187,
    108, 44, 152, 20, 7, 76, 151, 221, 47, 90, 132, 53, 19, 232, 160, 163, 253, 241, 117, 132, 228,
    107, 81, 80, 1,
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

    let key_1: String = String::from("Contract1");

    let key_2: String = String::from("Contract2");

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
            price_feed: vec![(key_1.clone(), price_data_1), (key_2.clone(), price_data_2)],
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

    let signature:SignatureEd25519 = "CA12702976177E17279A528CCA48F4704F29D56D0106E7A0A4129C0F673A3730364ABC48DC9698E298869478D2CFA0562BF5E1A8DDB2CCA0AB12E34171D0000A".parse().unwrap();
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
                message: OwnedParameter::from_serial(&vec![key_1.clone(), key_2.clone()])
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
                    "umbrella_feeds.getManyPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&vec![key_1, key_2])
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

    let key_1: String = String::from("Contract1");

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
            price_feed: vec![(key_1.clone(), price_data)],
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

    let signature:SignatureEd25519 = "E7382F68C762EE9A3CCEE7447BFDF849036E1647D80D94ABBE9B40EA950566156CAA4902D157968DC0F0EE71FC409E1035F06AC5B1C4CF370BE40D4FFD79CF0B".parse().unwrap();
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
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&key_1)
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
                message: OwnedParameter::from_serial(&key_1)
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
                message: OwnedParameter::from_serial(&key_1)
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
                message: OwnedParameter::from_serial(&key_1)
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
                message: OwnedParameter::from_serial(&key_1)
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
            price_feed: vec![(String::from("ETH-CCD"), price_data)],
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

    let signature:SignatureEd25519 = "B28711D7D3FED29CF849CE5016522FA3BFB1CE1036227F8BAD5923BD6EC8900D688D182B1C79C3180990CAF3D1D45F79D6EAF985EA123A091A9296E0815A3702".parse().unwrap();
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
                    "umbrella_feeds.getPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&String::from("ETH-CCD"))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query timestam");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);
}

#[test]
fn test_get_name() {
    let (
        chain,
        initialization_umbrella_feeds,
        _initialization_registry,
        _initialization_staking_bank,
    ) = setup_chain_and_contract();

    // Checking getName.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.getName".to_string()),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query contract name");

    let value: String = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, String::from("UmbrellaFeeds"));
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
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
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
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
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
