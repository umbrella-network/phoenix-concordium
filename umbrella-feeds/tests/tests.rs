use std::collections::BTreeMap;

use concordium_smart_contract_testing::*;
use concordium_smart_contract_testing::{AccountAccessStructure, *};
use concordium_std::{
    AccountPublicKeys, AccountSignatures, CredentialSignatures, PublicKey, SignatureEd25519,
    Timestamp,
};
use concordium_std::{Deserial, HashSha2256};
use registry::ImportContractsParam;
use umbrella_feeds::{InitContractsParam, Message, PriceData, UpdateParams, UpgradeParams};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const SIGNATURE: SignatureEd25519 = SignatureEd25519([
    177, 27, 240, 37, 5, 20, 109, 227, 126, 203, 176, 159, 238, 112, 254, 237, 1, 25, 131, 179, 65,
    90, 52, 15, 204, 85, 2, 11, 126, 105, 235, 8, 87, 107, 162, 148, 141, 27, 196, 228, 114, 69,
    128, 157, 202, 30, 194, 244, 13, 189, 89, 79, 220, 244, 14, 43, 83, 137, 25, 138, 6, 178, 238,
    12,
]);

const KEY_HASH: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

// Private key: 8ECA45107A878FB879B84401084B55AD4919FC0F7D14E8915D8A5989B1AE1C01
const PUBLIC_KEY: [u8; 32] = [
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
];

const ACC_ADDR_OTHER: AccountAddress = AccountAddress([1u8; 32]);

fn setup_chain_and_contract() -> (Chain, ContractInitSuccess, ContractInitSuccess) {
    let mut chain = Chain::new();

    let balance = AccountBalance {
        total: ACC_INITIAL_BALANCE,
        staked: Amount::zero(),
        locked: Amount::zero(),
    };

    let mut inner_key_map: BTreeMap<KeyIndex, VerifyKey> = BTreeMap::new();

    inner_key_map.insert(
        KeyIndex(0u8),
        VerifyKey::Ed25519VerifyKey(
            ed25519_dalek::PublicKey::from_bytes(&PUBLIC_KEY)
                .expect("Should be able to create public key"),
        ),
    );

    let credential_public_keys = CredentialPublicKeys {
        keys: inner_key_map,
        threshold: SignatureThreshold::ONE,
    };

    let mut key_map: BTreeMap<CredentialIndex, CredentialPublicKeys> = BTreeMap::new();
    key_map.insert(CredentialIndex { index: 0u8 }, credential_public_keys);

    let keys = AccountAccessStructure {
        keys: key_map,
        threshold: AccountThreshold::ONE,
    };

    chain.create_account(Account::new_with_keys(ACC_ADDR_OTHER, balance, keys));
    chain.create_account(Account::new(ACC_ADDR_OWNER, ACC_INITIAL_BALANCE));

    // Deploying registry contract

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
        .expect("Initialization of `Umbrella feeds` should always succeed");

    // Deploy 'umbrella_feeds' contract

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./umbrella_feeds.wasm.v1")
                .expect("`Umbrella_feeds.wasm.v1` module should be loaded"),
        )
        .expect("`Umbrella_feeds.wasm.v1` deployment should always succeed");

    let input_parameter = InitContractsParam {
        registry: initialization_registry.contract_address,
        required_signatures: 2,
        staking_bank: ContractAddress {
            index: 1,
            subindex: 0,
        },
        decimals: 4,
    };

    let initialization = chain
        .contract_init(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Energy::from(10000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment.module_reference,
                init_name: OwnedContractName::new_unchecked("init_umbrella_feeds".to_string()),
                param: OwnedParameter::from_serial(&input_parameter)
                    .expect("`InitContractsParam` should be a valid inut parameter"),
            },
        )
        .expect("Initialization of `Umbrella feeds` should always succeed");

    (chain, initialization, initialization_registry)
}

#[test]
fn test_init() {
    let (chain, initialization, initalization_registry) = setup_chain_and_contract();
    assert_eq!(
        chain.contract_balance(initialization.contract_address),
        Some(Amount::zero()),
        "Contract should have no funds"
    );
}

/// Permit update operator function.
#[test]
fn test_update_operator() {
    let (mut chain, initialization, initalization_registry) = setup_chain_and_contract();

    let mut inner_signature_map = BTreeMap::new();
    inner_signature_map.insert(0u8, concordium_std::Signature::Ed25519(SIGNATURE));

    let mut signature_map = BTreeMap::new();
    signature_map.insert(
        0u8,
        CredentialSignatures {
            sigs: inner_signature_map,
        },
    );

    let price_data = PriceData {
        data: 7,
        heartbeat: 12,
        timestamp: 9,
        price: 4,
    };

    let update_param = UpdateParams {
        signatures: vec![AccountSignatures {
            sigs: signature_map,
        }],
        signer: ACC_ADDR_OTHER,
        message: Message {
            timestamp: Timestamp::from_timestamp_millis(10000000000),
            contract_address: initialization.contract_address,
            chain_id: 0,
            price_feed: vec![(KEY_HASH, price_data)],
            entry_point: OwnedEntrypointName::new_unchecked("update".into()),
        },
    };

    // Check operator in state
    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.viewMessageHash".to_string(),
                ),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceData");

    let message_hash: [u8; 32] =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    println!("Sign this message hash: {}", HashSha2256(message_hash));

    let signature:SignatureEd25519 = "B11BF02505146DE37ECBB09FEE70FEED011983B3415A340FCC55020B7E69EB08576BA2948D1BC4E47245809DCA1EC2F40DBD594FDCF40E2B5389198A06B2EE0C".parse().unwrap();
    println!("Signature: {:?}", signature.0);

    // Update operator with the permit function.
    let update = chain
        .contract_update(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.update".to_string()),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to update operator with permit");

    // Check operator in state
    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.getPriceData".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceData");

    let stored_price_data: PriceData =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(stored_price_data, price_data);

}

#[test]
fn test_upgrade_without_migration_function() {
    let (mut chain, initialization, initalization_registry) = setup_chain_and_contract();

    let input_parameter = ImportContractsParam {
        entries: vec![initialization.contract_address],
    };

    let update = chain.contract_update(
        Signer::with_one_key(), // Used for specifying the number of signatures.
        ACC_ADDR_OWNER,         // Invoker account.
        Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
        Energy::from(10000),    // Maximum energy allowed for the update.
        UpdateContractPayload {
            address: initalization_registry.contract_address, // The contract to update.
            receive_name: OwnedReceiveName::new_unchecked("registry.importContracts".into()), // The receive function to call.
            message: OwnedParameter::from_serial(&input_parameter)
                .expect("`UpgradeParams` should be a valid inut parameter"), // The parameter sent to the contract.
            amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
        },
    );

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./umbrella_feeds_update.wasm.v1")
                .expect("`Contract version2` module should be loaded"),
        )
        .expect("`Contract version2` deployment should always succeed");

    let input_parameter = UpgradeParams {
        module: deployment.module_reference,
        migrate: None,
    };

    let update = chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            ACC_ADDR_OWNER,         // Invoker account.
            Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
            Energy::from(10000),    // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.upgrade".into()), // The receive function to call.
                message: OwnedParameter::from_serial(&input_parameter)
                    .expect("`UpgradeParams` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to update");

    // Upgrade `contract_version1` to `contract_version2`.
    let update = chain.contract_update(
        Signer::with_one_key(), // Used for specifying the number of signatures.
        ACC_ADDR_OWNER,         // Invoker account.
        Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
        Energy::from(10000),    // Maximum energy allowed for the update.
        UpdateContractPayload {
            address: initialization.contract_address, // The contract to update.
            receive_name: OwnedReceiveName::new_unchecked("umbrella_feeds.destroy_2".into()), // The receive function to call.
            message: OwnedParameter::from_serial(&input_parameter)
                .expect("`UpgradeParams` should be a valid inut parameter"), // The parameter sent to the contract.
            amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
        },
    );
}
