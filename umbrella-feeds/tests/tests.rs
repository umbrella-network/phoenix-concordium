use std::collections::BTreeMap;

use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use concordium_std::{CredentialSignatures, PublicKeyEd25519, SignatureEd25519, Timestamp};
use registry::{AtomicUpdateParam, ImportContractsParam};
use umbrella_feeds::{InitParamsUmbrellaFeeds, Message, PriceData, UpdateParams};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const SIGNATURE_1: SignatureEd25519 = SignatureEd25519([
    62, 249, 216, 64, 254, 146, 219, 193, 206, 85, 5, 3, 8, 159, 59, 13, 220, 137, 7, 24, 223, 3,
    175, 250, 197, 11, 60, 69, 23, 216, 168, 204, 14, 149, 79, 4, 53, 160, 185, 1, 64, 89, 235,
    247, 68, 154, 180, 154, 184, 134, 106, 107, 63, 255, 162, 165, 144, 52, 5, 54, 117, 246, 85,
    11,
]);

const SIGNATURE_2: SignatureEd25519 = SignatureEd25519([
    22, 201, 213, 137, 246, 0, 4, 72, 162, 15, 41, 249, 203, 196, 21, 119, 113, 193, 117, 154, 177,
    236, 61, 1, 172, 130, 50, 63, 136, 145, 195, 172, 33, 2, 28, 19, 165, 86, 39, 169, 88, 60, 173,
    150, 71, 220, 65, 195, 2, 131, 46, 131, 98, 188, 42, 123, 191, 240, 139, 211, 203, 99, 26, 6,
]);

const SIGNATURE_TWO_PRICE_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    232, 217, 219, 103, 59, 31, 70, 31, 76, 128, 163, 240, 9, 235, 60, 68, 146, 123, 96, 90, 43,
    166, 204, 43, 220, 254, 92, 142, 132, 163, 161, 68, 72, 167, 50, 120, 205, 218, 202, 102, 16,
    11, 170, 25, 229, 141, 117, 233, 179, 90, 159, 237, 126, 56, 163, 47, 19, 127, 81, 124, 124,
    242, 101, 12,
]);

const SIGNATURE_TWO_PRICE_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    238, 25, 132, 147, 139, 172, 54, 151, 142, 195, 59, 126, 46, 204, 21, 247, 30, 186, 85, 237,
    16, 41, 116, 22, 204, 35, 38, 222, 203, 79, 167, 86, 74, 228, 9, 11, 144, 178, 69, 227, 63,
    107, 102, 121, 240, 237, 24, 4, 199, 88, 115, 162, 130, 206, 183, 141, 79, 242, 193, 144, 42,
    205, 221, 10,
]);

const SIGNATURE_ETH_CCD_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    159, 193, 0, 133, 68, 49, 251, 167, 191, 33, 12, 88, 229, 186, 240, 12, 71, 240, 169, 192, 87,
    123, 98, 142, 245, 117, 233, 211, 92, 208, 209, 30, 67, 64, 117, 225, 161, 142, 219, 32, 193,
    93, 31, 89, 247, 76, 54, 39, 152, 186, 192, 151, 2, 142, 105, 52, 93, 27, 132, 114, 221, 252,
    248, 13,
]);

const SIGNATURE_ETH_CCD_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    120, 223, 9, 151, 2, 222, 215, 181, 221, 168, 14, 103, 178, 27, 50, 33, 207, 172, 47, 89, 34,
    120, 237, 111, 54, 140, 85, 192, 199, 164, 178, 219, 121, 92, 133, 20, 164, 189, 194, 59, 57,
    199, 248, 175, 128, 168, 93, 120, 159, 203, 57, 92, 33, 111, 4, 129, 133, 22, 38, 36, 245, 6,
    238, 14,
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

fn setup_chain_and_contract() -> (
    Chain,
    ContractInitSuccess,
    ContractInitSuccess,
    ContractInitSuccess,
) {
    let mut chain = Chain::new();

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
                PublicKeyEd25519(PUBLIC_KEY_SIGNER_1),
                SIGNATURE_TWO_PRICE_FEEDS_1,
            ),
            (
                PublicKeyEd25519(PUBLIC_KEY_SIGNER_2),
                SIGNATURE_TWO_PRICE_FEEDS_2,
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

    let message_hash: [u8; 32] =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    println!(
        "Signer sign this message hash: {}",
        HashSha2256(message_hash)
    );

    let signature:SignatureEd25519 = "EE1984938BAC36978EC33B7E2ECC15F71EBA55ED10297416CC2326DECB4FA7564AE4090B90B245E33F6B6679F0ED1804C75873A282CEB78D4FF2C1902ACDDD0A".parse().unwrap();
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
            (PublicKeyEd25519(PUBLIC_KEY_SIGNER_1), SIGNATURE_1),
            (PublicKeyEd25519(PUBLIC_KEY_SIGNER_2), SIGNATURE_2),
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

    let message_hash: [u8; 32] =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    println!(
        "Signer sign this message hash: {}",
        HashSha2256(message_hash)
    );

    let signature:SignatureEd25519 = "16C9D589F6000448A20F29F9CBC4157771C1759AB1EC3D01AC82323F8891C3AC21021C13A55627A9583CAD9647DC41C302832E8362BC2A7BBFF08BD3CB631A06".parse().unwrap();
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
                PublicKeyEd25519(PUBLIC_KEY_SIGNER_1),
                SIGNATURE_ETH_CCD_FEEDS_1,
            ),
            (
                PublicKeyEd25519(PUBLIC_KEY_SIGNER_2),
                SIGNATURE_ETH_CCD_FEEDS_2,
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

    let message_hash: [u8; 32] =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    println!(
        "Signer sign this message hash: {}",
        HashSha2256(message_hash)
    );

    let signature:SignatureEd25519 = "9FC100854431FBA7BF210C58E5BAF00C47F0A9C0577B628EF575E9D35CD0D11E434075E1A18EDB20C15D1F59F74C362798BAC097028E69345D1B8472DDFCF80D".parse().unwrap();
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
