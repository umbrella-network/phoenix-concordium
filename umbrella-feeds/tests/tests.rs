use std::collections::BTreeMap;

use common_types::U256Wrapper;
use concordium_smart_contract_testing::*;
use concordium_smart_contract_testing::{AccountAccessStructure, *};
use concordium_std::{
    AccountPublicKeys, AccountSignatures, CredentialSignatures, PublicKey, PublicKeyEd25519,
    SignatureEd25519, Timestamp,
};
use concordium_std::{Deserial, HashSha2256};
use primitive_types::U256;
use registry::{AtomicUpdateParam, AtomicUpdateParams, ImportContractsParam};
use staking_bank::InitContractsParamStakingBank;
use umbrella_feeds::{InitContractsParam, Message, PriceData, UpdateParams};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const KEY_HASH: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

const SIGNATURE_1: SignatureEd25519 = SignatureEd25519([
    189, 152, 44, 34, 186, 100, 205, 30, 179, 165, 189, 160, 222, 181, 141, 211, 40, 16, 39, 157,
    133, 223, 86, 89, 119, 124, 107, 189, 82, 141, 116, 40, 9, 246, 230, 45, 235, 191, 51, 165, 44,
    93, 75, 46, 84, 25, 196, 26, 121, 102, 175, 172, 186, 68, 159, 184, 88, 93, 48, 126, 83, 2, 80,
    15,
]);

const SIGNATURE_2: SignatureEd25519 = SignatureEd25519([
    246, 255, 94, 113, 247, 234, 107, 234, 171, 161, 187, 166, 178, 254, 207, 190, 62, 58, 27, 73,
    93, 50, 138, 133, 251, 134, 108, 147, 177, 135, 6, 103, 189, 254, 232, 248, 23, 118, 167, 202,
    229, 219, 88, 107, 45, 23, 216, 52, 133, 167, 61, 67, 197, 91, 252, 116, 67, 198, 118, 180,
    204, 171, 181, 11,
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
        .expect("Initialization of `Umbrella feeds` should always succeed");

    // Deploying 'staking bank' contract

    let deployment_staking_bank = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../staking-bank/staking_bank.wasm.v1")
                .expect("`staking_bank.wasm.v1` module should be loaded"),
        )
        .expect("`staking_bank.wasm.v1` deployment should always succeed");

    let input_parameter = InitContractsParamStakingBank {
        validators_count: U256Wrapper(U256::from_dec_str("15").unwrap()),
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
                    .expect("`InitContractsParam` should be a valid inut parameter"),
            },
        )
        .expect("Initialization of `Staking_bank` should always succeed");

    // Deploy 'umbrella_feeds' contract

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./umbrella_feeds.wasm.v1")
                .expect("`Umbrella_feeds.wasm.v1` module should be loaded"),
        )
        .expect("`Umbrella_feeds.wasm.v1` deployment should always succeed");

    let input_parameter_2 = InitContractsParam {
        registry: initialization_registry.contract_address,
        required_signatures: 2,
        staking_bank: initialization_staking_bank.contract_address,
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
                param: OwnedParameter::from_serial(&input_parameter_2)
                    .expect("`InitContractsParam` should be a valid inut parameter"),
            },
        )
        .expect("Initialization of `Umbrella feeds` should always succeed");

    (
        chain,
        initialization,
        initialization_registry,
        initialization_staking_bank,
    )
}

#[test]
fn test_init() {
    let (chain, initialization, initialization_registry, initialization_staking_bank) =
        setup_chain_and_contract();
    assert_eq!(
        chain.contract_balance(initialization.contract_address),
        Some(Amount::zero()),
        "Contract should have no funds"
    );
}

/// Permit update operator function.
#[test]
fn test_update_operator() {
    let (mut chain, initialization, initialization_registry, initialization_staking_bank) =
        setup_chain_and_contract();

    let price_data = PriceData {
        data: 7,
        heartbeat: 12,
        timestamp: 9,
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
        signatures: vec![
            AccountSignatures {
                sigs: signature_map,
            },
            AccountSignatures {
                sigs: signature_map_signer_2,
            },
        ],
        signer: vec![SIGNER_1, SIGNER_2],
        message: Message {
            timestamp: Timestamp::from_timestamp_millis(10000000000),
            contract_address: initialization.contract_address,
            chain_id: 0,
            price_feed: vec![(KEY_HASH, price_data)],
            entry_point: OwnedEntrypointName::new_unchecked("update".into()),
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
                address: initialization.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "umbrella_feeds.viewMessageHash".to_string(),
                ),
                message: OwnedParameter::from_serial(&update_param)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query getPriceData");

    let message_hashes: Vec<[u8; 32]> =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    for (i, message_hash) in message_hashes.iter().enumerate() {
        println!(
            "Signer {} sign this message hash: {}",
            i,
            HashSha2256(*message_hash)
        );
    }

    let signature:SignatureEd25519 = "F6FF5E71F7EA6BEAABA1BBA6B2FECFBE3E3A1B495D328A85FB866C93B1870667BDFEE8F81776A7CAE5DB586B2D17D83485A73D43C55BFC7443C676B4CCABB50B".parse().unwrap();
    println!("Signature: {:?}", signature.0);

    let public_key: PublicKeyEd25519 =
        "D96C4B1218EA7EC20F4604D6C2F02FA3F36B518443F3A2D14E885E7FF715DEDD"
            .parse()
            .unwrap();
    println!("Public key: {:?}", public_key.0);

    // Updating price data in contract

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

    // Checking price data was updated correctly in contract

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
    let (mut chain, initialization, initialization_registry, initialization_staking_bank) =
        setup_chain_and_contract();

    let input_parameter = ImportContractsParam {
        entries: vec![initialization.contract_address],
    };

    let update = chain.contract_update(
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

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./umbrella_feeds_update.wasm.v1")
                .expect("`Contract version2` module should be loaded"),
        )
        .expect("`Contract version2` deployment should always succeed");

    let input_parameter = AtomicUpdateParams {
        entries: vec![AtomicUpdateParam {
            module: deployment.module_reference,
            migrate: None,
            contract_address: initialization.contract_address,
        }],
    };

    let update = chain
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
