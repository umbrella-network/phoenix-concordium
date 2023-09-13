use concordium_smart_contract_testing::*;
use concordium_std::Deserial;
use registry::ImportContractsParam;
use umbrella_feeds::{InitContractsParam, UpgradeParams};

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

    // TODO increase time to 3 days

    let update = chain.contract_update(
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
    );
    println!("{:?}", update);

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
    println!("{:?}", update);
}
