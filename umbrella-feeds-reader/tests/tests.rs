use std::collections::BTreeMap;

use concordium_smart_contract_testing::AccountAccessStructure;
use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use concordium_std::{
    AccountSignatures, CredentialSignatures, PublicKeyEd25519, SignatureEd25519, Timestamp,
};
use registry::{AtomicUpdateParam, ImportContractsParam};
use sha256::digest;
use staking_bank::InitParamsStakingBank;
use umbrella_feeds::{InitParamsUmbrellaFeeds, Message, PriceData, UpdateParams};
use umbrella_feeds_reader::{InitParamsUmbrellaFeedsReader, SchemaTypeQuintWrapper};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(100000000000);

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

    // Deploying 'staking bank' contract

    let deployment_staking_bank = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../staking-bank/staking_bank.wasm.v1")
                .expect("`staking_bank.wasm.v1` module should be loaded"),
        )
        .expect("`staking_bank.wasm.v1` deployment should always succeed");

    let input_parameter = InitParamsStakingBank {
        validators_count: 15u8,
    };

    let initialization_staking_bank = chain
        .contract_init(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Energy::from(10000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment_staking_bank.module_reference,
                init_name: OwnedContractName::new_unchecked("init_staking_bank".to_string()),
                param: OwnedParameter::from_serial(&input_parameter)
                    .expect("`input_parameter` should be a valid inut parameter"),
            },
        )
        .expect("Initialization of `staking_bank` should always succeed");

    // Deploying 'registry' contract

    let deployment_registry = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../registry/registry.wasm.v1")
                .expect("`registry.wasm.v1` module should be loaded"),
        )
        .expect("`registry.wasm.v1` deployment should always succeed");

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

    // Deploy 'umbrella_feeds' contract

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../umbrella-feeds/umbrella_feeds.wasm.v1")
                .expect("`umbrella_feeds.wasm.v1` module should be loaded"),
        )
        .expect("`umbrella_feeds.wasm.v1` deployment should always succeed");

    let input_parameter_2 = InitParamsUmbrellaFeeds {
        registry: initialization_registry.contract_address,
        required_signatures: 2,
        staking_bank: initialization_staking_bank.contract_address,
        decimals: 18,
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
                    .expect("`input_parameter_2` should be a valid inut parameter"),
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

/// Test to update the price feed and query values via the umbrella_feeds_reader contract.
#[test]
fn test_update_price_feed() {
    let (
        mut chain,
        initialization_umbrella_feeds,
        initialization_registry,
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
        .expect("Should be able to query viewMessageHash");

    let message_hashes: Vec<[u8; 32]> =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    for (i, message_hash) in message_hashes.iter().enumerate() {
        println!(
            "Signer {} sign this message hash: {}",
            i,
            HashSha2256(*message_hash)
        );
    }

    let signature:SignatureEd25519 = "A1D631FC140C7C22A223A5335F2E86D841D7DC0E059B1A77F3CE6B77487CF58D4516BFAD7A2EDEF7D3BFB0C0BC10B835A8854889434FED62786F431A3BFEE204".parse().unwrap();
    println!("Signature: {:?}", signature.0);

    let public_key: PublicKeyEd25519 =
        "D96C4B1218EA7EC20F4604D6C2F02FA3F36B518443F3A2D14E885E7FF715DEDD"
            .parse()
            .unwrap();
    println!("Public key: {:?}", public_key.0);

    // Updating price data in contract

    chain
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
        .expect("Should be able to update the price data in the umbrella_feeds contract");

    // Checking price data was updated correctly in contract

    let key_hash: HashSha2256 = digest(String::from("ETH-CCD")).parse().unwrap();

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
                message: OwnedParameter::from_serial(&key_hash)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceData");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);

    // Deploying 'umbrella_feeds_reader' contract

    let deployment_umbrella_feeds_reader = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./umbrella_feeds_reader.wasm.v1")
                .expect("`umbrella_feeds_reader.wasm.v1` module should be loaded"),
        )
        .expect("`umbrella_feeds_reader.wasm.v1` deployment should always succeed");

    let input_parameter_3 = InitParamsUmbrellaFeedsReader {
        registry: initialization_registry.contract_address,
        umbrella_feeds: initialization_umbrella_feeds.contract_address,
        description: "ETH-CCD".to_string(),
        decimals: 18,
    };

    let initialization_umbrella_feeds_reader = chain
        .contract_init(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Energy::from(10000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment_umbrella_feeds_reader.module_reference,
                init_name: OwnedContractName::new_unchecked(
                    "init_umbrella_feeds_reader".to_string(),
                ),
                param: OwnedParameter::from_serial(&input_parameter_3)
                    .expect("`input_parameter_3` should be a valid inut parameter"),
            },
        )
        .expect("Initialization of `umbrella_feeds_reader` should always succeed");

    // Checking the setup.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds_reader.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds_reader.checkSetUp".to_string(),
                ),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query setup");

    let is_correctly_initilized: bool =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(is_correctly_initilized, true);

    // Checking price data via umbrella_feeds_reader

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds_reader.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds_reader.getPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&key_hash)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceData");

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
                address: initialization_umbrella_feeds_reader.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds_reader.getPriceDataRaw".to_string(),
                ),
                message: OwnedParameter::from_serial(&key_hash)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceDataRaw");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);

    // Checking `getPriceDataRaw` and `getPriceData` via umbrella_feeds_reader after an upgrade to `umbrella_feeds` contract

    // We upgrade the `umbrellaFeeds` contract first

    let input_parameter = ImportContractsParam {
        entries: vec![initialization_umbrella_feeds.contract_address],
    };

    chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            ACC_ADDR_OWNER,         // Invoker account.
            Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
            Energy::from(10000),    // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization_registry.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("registry.importContracts".into()), // The receive function to call.
                message: OwnedParameter::from_serial(&input_parameter)
                    .expect("`input_parameter` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to update registry contract");

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../umbrella-feeds/umbrella_feeds_update.wasm.v1")
                .expect("`umbrella_feeds_update` module should be loaded"),
        )
        .expect("`umbrella_feeds_update` deployment should always succeed");

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
                    .expect("`input_parameter` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to update");

    // We finished upgrading the `umbrellaFeeds` contract

    // Checking `getPriceDataRaw` and `getPriceData` via umbrella_feeds_reader

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds_reader.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds_reader.getPriceDataRaw".to_string(),
                ),
                message: OwnedParameter::from_serial(&key_hash)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceDataRaw");

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
                address: initialization_umbrella_feeds_reader.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds_reader.getPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&key_hash)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceData");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);

    // Checking `latestRoundData` via umbrella_feeds_reader

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds_reader.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds_reader.latestRoundData".to_string(),
                ),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query latestRoundData");

    let stored_price_data: SchemaTypeQuintWrapper =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    let expected_price_data =
        SchemaTypeQuintWrapper(0u8, price_data.price, 0u8, price_data.timestamp, 0u8);

    assert_eq!(stored_price_data, expected_price_data);

    // Checking `DECIMAL` getter function

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_umbrella_feeds_reader.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds_reader.DECIMALS".to_string(),
                ),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query value");

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 18);
}
