use std::collections::BTreeMap;

use concordium_smart_contract_testing::*;
use concordium_std::{
    AccountPublicKeys, AccountSignatures, CredentialSignatures, PublicKey, SignatureEd25519,
    Timestamp,
};
use concordium_std::{Deserial, HashSha2256};
use registry::ImportContractsParam;
use umbrella_feeds::{InitContractsParam, Message, PriceData, UpdateParams, UpgradeParams};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([0u8; 32]);
const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

fn setup_chain_and_contract() -> (Chain, ContractInitSuccess, ContractInitSuccess) {
    let mut chain = Chain::new();

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

    const SIGNATURE: SignatureEd25519 = SignatureEd25519([
        46, 96, 133, 143, 232, 24, 149, 54, 217, 245, 162, 135, 64, 125, 32, 61, 209, 147, 240,
        151, 19, 244, 137, 244, 91, 59, 120, 202, 39, 201, 82, 39, 64, 210, 250, 183, 187, 27, 147,
        50, 31, 88, 78, 79, 78, 135, 192, 72, 38, 234, 90, 226, 89, 75, 124, 86, 1, 190, 196, 195,
        248, 19, 181, 11,
    ]);

    const KEY_HASH: HashSha2256 = HashSha2256([
        120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208,
        195, 253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
    ]);

    const ACC_ADDR_OTHER: AccountAddress = AccountAddress([1u8; 32]);

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
        signature: vec![AccountSignatures {
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

    // // Create input parematers for the `permit` updateOperator function.
    // let update_operator = UpdateOperator {
    //     update:   OperatorUpdate::Add,
    //     operator: ADDR_OWNER,
    // };
    // let payload = UpdateOperatorParams(vec![update_operator]);

    // let mut inner_signature_map = BTreeMap::new();
    // inner_signature_map.insert(0u8, concordium_std::Signature::Ed25519(SIGNATURE_UPDATE_OPERATOR));

    // let mut signature_map = BTreeMap::new();
    // signature_map.insert(0u8, CredentialSignatures {
    //     sigs: inner_signature_map,
    // });

    // let permit_update_operator_param = PermitParam {
    //     signature: AccountSignatures {
    //         sigs: signature_map,
    //     },
    //     signer:    ACC_ADDR_OTHER,
    //     message:   PermitMessage {
    //         timestamp:        Timestamp::from_timestamp_millis(10000000000),
    //         contract_address: ContractAddress {
    //             index:    0,
    //             subindex: 0,
    //         },
    //         entry_point:      OwnedEntrypointName::new_unchecked("updateOperator".into()),
    //         nonce:            0,
    //         payload:          to_bytes(&payload),
    //     },
    // };

    // // Update operator with the permit function.
    // let update = chain
    //     .contract_update(
    //         Signer::with_one_key(),
    //         ACC_ADDR_OWNER,
    //         Address::Account(ACC_ADDR_OWNER),
    //         Energy::from(10000),
    //         UpdateContractPayload {
    //             amount:       Amount::zero(),
    //             address:      initialization.contract_address,
    //             receive_name: OwnedReceiveName::new_unchecked("cis3_nft.permit".to_string()),
    //             message:      OwnedParameter::from_serial(&permit_update_operator_param)
    //                 .expect("Should be a valid inut parameter"),
    //         },
    //     )
    //     .expect("Should be able to update operator with permit");

    // // Check logged events.
    // let events: Vec<(ContractAddress, &[ContractEvent])> = update.events().collect();

    // // Check update operator event.
    // let update_operator_event = &events[0].1[0];

    // // Check event tag.
    // assert_eq!(
    //     update_operator_event.as_ref()[0],
    //     UPDATE_OPERATOR_EVENT_TAG,
    //     "Update operator event tag is wrong"
    // );

    // // We remove the tag byte at the beginning of the event.
    // let update_operator_event_type: UpdateOperatorEvent =
    //     from_bytes(&update_operator_event.as_ref()[1..]).expect("Tag removal should work");

    // assert_eq!(
    //     update_operator_event_type,
    //     UpdateOperatorEvent {
    //         update:   OperatorUpdate::Add,
    //         owner:    ADDR_OTHER,
    //         operator: ADDR_OWNER,
    //     },
    //     "Update operator event is wrong"
    // );

    // // Check nonce event.
    // let nonce_event = &events[0].1[1];

    // // Check event tag.
    // assert_eq!(nonce_event.as_ref()[0], NONCE_EVENT_TAG, "Nonce event tag is wrong");

    // // We remove the tag byte at the beginning of the event.
    // let nonce_event_type: NonceEvent =
    //     from_bytes(&nonce_event.as_ref()[1..]).expect("Tag removal should work");

    // assert_eq!(
    //     nonce_event_type,
    //     NonceEvent {
    //         account: ACC_ADDR_OTHER,
    //         nonce:   0,
    //     },
    //     "Nonce event is wrong"
    // );

    // // Check operator in state
    // let operator_of_query = OperatorOfQuery {
    //     address: ADDR_OWNER,
    //     owner:   ADDR_OTHER,
    // };

    // let operator_of_query_vector = OperatorOfQueryParams {
    //     queries: vec![operator_of_query],
    // };

    // // Check operator in state
    // let invoke = chain
    //     .contract_invoke(
    //         ACC_ADDR_OWNER,
    //         Address::Account(ACC_ADDR_OWNER),
    //         Energy::from(10000),
    //         UpdateContractPayload {
    //             amount:       Amount::zero(),
    //             address:      initialization.contract_address,
    //             receive_name: OwnedReceiveName::new_unchecked("cis3_nft.operatorOf".to_string()),
    //             message:      OwnedParameter::from_serial(&operator_of_query_vector)
    //                 .expect("Should be a valid inut parameter"),
    //         },
    //     )
    //     .expect("Should be able to query operatorOf");

    // let is_operator_of: OperatorOfQueryResponse =
    //     from_bytes(&invoke.return_value).expect("Should return a valid result");

    // assert_eq!(is_operator_of.0, [true])
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
