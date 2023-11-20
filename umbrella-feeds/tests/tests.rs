use std::collections::BTreeMap;

use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use concordium_std::{CredentialSignatures, PublicKeyEd25519, SignatureEd25519, Timestamp};
use registry::{AtomicUpdateParam, ImportContractsParam};
use umbrella_feeds::{
    InitParamsUmbrellaFeeds, Message, PriceData, SchemTypeTripleWrapper, UpdateParams,
};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const SIGNATURE_1: SignatureEd25519 = SignatureEd25519([
    164, 191, 6, 21, 217, 234, 176, 156, 185, 143, 216, 26, 124, 119, 131, 85, 224, 141, 17, 246,
    247, 146, 73, 106, 223, 50, 233, 51, 210, 10, 32, 183, 130, 49, 169, 168, 205, 158, 101, 68,
    140, 62, 16, 113, 143, 32, 3, 196, 129, 59, 5, 194, 10, 112, 236, 121, 42, 138, 46, 149, 233,
    23, 62, 8,
]);

const SIGNATURE_2: SignatureEd25519 = SignatureEd25519([
    221, 2, 131, 233, 224, 151, 220, 44, 231, 253, 251, 56, 244, 211, 94, 96, 32, 151, 62, 173,
    103, 122, 95, 9, 143, 157, 217, 56, 156, 208, 199, 107, 109, 250, 100, 155, 140, 251, 192, 227,
    67, 144, 239, 106, 235, 189, 210, 177, 144, 127, 42, 87, 182, 4, 61, 98, 149, 248, 107, 208,
    189, 122, 160, 7,
]);

const SIGNATURE_TWO_PRICE_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    176, 166, 29, 94, 27, 30, 37, 164, 99, 249, 241, 187, 242, 243, 197, 91, 181, 113, 238, 44,
    244, 5, 188, 251, 108, 212, 194, 43, 181, 88, 23, 32, 28, 136, 50, 14, 253, 112, 235, 97, 132,
    135, 213, 134, 107, 220, 186, 245, 111, 98, 86, 238, 119, 24, 57, 200, 111, 38, 36, 100, 99,
    220, 162, 10,
]);

const SIGNATURE_TWO_PRICE_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    196, 61, 181, 111, 127, 185, 47, 4, 157, 76, 15, 236, 43, 10, 103, 167, 154, 110, 171, 53, 86,
    46, 174, 244, 189, 250, 43, 118, 128, 34, 71, 147, 194, 61, 34, 251, 120, 228, 114, 13, 231,
    74, 103, 84, 169, 23, 52, 136, 48, 136, 87, 117, 238, 213, 108, 155, 251, 168, 84, 193, 67,
    181, 148, 7,
]);

const SIGNATURE_ETH_CCD_FEEDS_1: SignatureEd25519 = SignatureEd25519([
    139, 86, 122, 152, 201, 89, 126, 247, 247, 186, 92, 39, 185, 133, 142, 216, 12, 16, 2, 70, 47,
    162, 226, 127, 199, 202, 188, 128, 231, 126, 149, 217, 113, 250, 193, 179, 231, 66, 112, 12,
    126, 82, 159, 12, 69, 41, 28, 146, 216, 84, 199, 79, 65, 180, 211, 11, 94, 217, 66, 198, 137,
    36, 231, 0,
]);

const SIGNATURE_ETH_CCD_FEEDS_2: SignatureEd25519 = SignatureEd25519([
    159, 74, 113, 181, 44, 16, 67, 8, 10, 128, 47, 127, 246, 134, 152, 159, 246, 107, 57, 218, 104,
    14, 222, 209, 4, 217, 12, 117, 114, 181, 80, 215, 234, 18, 156, 143, 15, 180, 173, 174, 155,
    217, 139, 240, 23, 209, 76, 66, 92, 13, 19, 152, 184, 216, 29, 23, 219, 167, 217, 158, 166, 95,
    216, 7,
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

    let signature:SignatureEd25519 = "B0A61D5E1B1E25A463F9F1BBF2F3C55BB571EE2CF405BCFB6CD4C22BB55817201C88320EFD70EB618487D5866BDCBAF56F6256EE771839C86F26246463DCA20A".parse().unwrap();
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

    let signature:SignatureEd25519 = "A4BF0615D9EAB09CB98FD81A7C778355E08D11F6F792496ADF32E933D20A20B78231A9A8CD9E65448C3E10718F2003C4813B05C20A70EC792A8A2E95E9173E08".parse().unwrap();
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

    let stored_price_data: SchemTypeTripleWrapper =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        stored_price_data,
        SchemTypeTripleWrapper {
            price: price_data.price,
            timestamp: price_data.timestamp,
            heartbeat: price_data.heartbeat
        }
    );

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

    let signature:SignatureEd25519 = "9F4A71B52C1043080A802F7FF686989FF66B39DA680EDED104D90C7572B550D7EA129C8F0FB4ADAE9BD98BF017D14C425C0D1398B8D81D17DBA7D99EA65FD807".parse().unwrap();
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
fn test_get_required_signatures() {
    let (
        chain,
        initialization_umbrella_feeds,
        _initialization_registry,
        _initialization_staking_bank,
    ) = setup_chain_and_contract();

    // Checking requiredSignatures.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.requiredSignatures".to_string(),
                ),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query required signatures");

    let value: u16 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 2u16);
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
