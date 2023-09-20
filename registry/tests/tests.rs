use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use registry::{ImportAddressesParam, ImportAddressesParams, OwnershipTransferredEvent};
use registry::{ImportContractsParam, LogRegisteredEvent};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([77u8; 32]);
const OTHER_ACCOUNT: AccountAddress = AccountAddress([1u8; 32]);

const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const KEY_HASH_1: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);
const KEY_HASH_2: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 27, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

fn setup_chain_and_contract() -> (Chain, ContractInitSuccess) {
    let mut chain = Chain::new();

    // Creating accounts.
    chain.create_account(Account::new(ACC_ADDR_OWNER, ACC_INITIAL_BALANCE));
    chain.create_account(Account::new(OTHER_ACCOUNT, ACC_INITIAL_BALANCE));

    // Deploying 'registry' contract.

    let deployment_registry = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./registry.wasm.v1")
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

    (chain, initialization_registry)
}

#[test]
fn test_init() {
    let (_chain, initialization_registry) = setup_chain_and_contract();

    // Checking logged event.
    let events = initialization_registry.events;
    let event = &events[0];

    // Checking event tag.
    assert_eq!(event.as_ref()[0], 1, "Event tag is wrong");

    // Removing the tag byte at the beginning of the event.
    let event_struct: OwnershipTransferredEvent =
        from_bytes(&event.as_ref()[1..]).expect("Tag removal should work");

    assert_eq!(
        event_struct,
        OwnershipTransferredEvent {
            new_owner: Address::from(ACC_ADDR_OWNER),
            previous_owner: Address::from(AccountAddress([0u8; 32])),
        },
        "OwnershipTransferredEvent event is wrong"
    );
}

/// Test `importAddresses` function.
#[test]
fn test_import_addresses() {
    let (mut chain, initialization_registry) = setup_chain_and_contract();

    let umbrella_feeds_contract = ContractAddress {
        index: 8,
        subindex: 0,
    };

    let staking_bank_contract = ContractAddress {
        index: 99,
        subindex: 0,
    };

    let input_parameter = ImportAddressesParams {
        entries: vec![
            ImportAddressesParam {
                name: KEY_HASH_1,
                destination: umbrella_feeds_contract,
            },
            ImportAddressesParam {
                name: KEY_HASH_2,
                destination: staking_bank_contract,
            },
        ],
    };

    // Invoking 'importAddresses'.

    let update = chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            ACC_ADDR_OWNER,         // Invoker account.
            Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
            Energy::from(10000),    // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization_registry.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("registry.importAddresses".into()), // The receive function to call.
                message: OwnedParameter::from_serial(&input_parameter)
                    .expect("`input_parameter` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to importAddresses");

    // Checking logged event.
    let events: Vec<(ContractAddress, &[ContractEvent])> = update.events().collect();
    let event = &events[0].1[0];

    // Checking event tag.
    assert_eq!(event.as_ref()[0], 0, "Event tag is wrong");

    // Removing the tag byte at the beginning of the event.
    let event_struct: LogRegisteredEvent =
        from_bytes(&event.as_ref()[1..]).expect("Tag removal should work");

    assert_eq!(
        event_struct,
        LogRegisteredEvent {
            name: KEY_HASH_1,
            destination: umbrella_feeds_contract,
        },
        "LogRegistered event is wrong"
    );

    let event = &events[0].1[1];

    // Checking event tag.
    assert_eq!(event.as_ref()[0], 0, "Event tag is wrong");

    // Removing the tag byte at the beginning of the event.
    let event_struct: LogRegisteredEvent =
        from_bytes(&event.as_ref()[1..]).expect("Tag removal should work");

    assert_eq!(
        event_struct,
        LogRegisteredEvent {
            name: KEY_HASH_2,
            destination: staking_bank_contract,
        },
        "LogRegistered event is wrong"
    );

    // Checking that contract address was registered correctly in registry.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(contract_address, umbrella_feeds_contract);
}

/// Test `importContracts` function.
#[test]
fn test_import_contracts() {
    let (mut chain, initialization_registry) = setup_chain_and_contract();

    // Deploying 'dummy_contract' contract.

    let deployment = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("../dummy-contract/dummy-contract.wasm.v1")
                .expect("`dummy-contract.wasm.v1` module should be loaded"),
        )
        .expect("`dummy-contract.wasm.v1` deployment should always succeed");

    let initialization_dummy_contract = chain
        .contract_init(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            Energy::from(10000),
            InitContractPayload {
                amount: Amount::zero(),
                mod_ref: deployment.module_reference,
                init_name: OwnedContractName::new_unchecked("init_dummy_contract".to_string()),
                param: OwnedParameter::empty(),
            },
        )
        .expect("Initialization of `dummy-contract` should always succeed");

    let input_parameter = ImportContractsParam {
        entries: vec![initialization_dummy_contract.contract_address],
    };

    // Invoking 'importContracts'.

    let update = chain
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
        .expect("Should be able to importContracts");

    // Getting the key hash.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "registry.stringToHashSha2256".to_string(),
                ),
                message: OwnedParameter::from_serial(&String::from("MyName"))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let hash: HashSha2256 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    // Checking logged event.
    let events: Vec<(ContractAddress, &[ContractEvent])> = update.events().collect();
    let event = &events[1].1[0];

    // checking event tag.
    assert_eq!(event.as_ref()[0], 0, "Event tag is wrong");

    // Removing the tag byte at the beginning of the event.
    let event_struct: LogRegisteredEvent =
        from_bytes(&event.as_ref()[1..]).expect("Tag removal should work");

    assert_eq!(
        event_struct,
        LogRegisteredEvent {
            name: hash,
            destination: initialization_dummy_contract.contract_address,
        },
        "LogRegistered event is wrong"
    );

    // Checking that contract address was registered correctly in registry.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
                message: OwnedParameter::from_serial(&hash)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        contract_address,
        initialization_dummy_contract.contract_address
    );

    // Checking that contract address was registered correctly in registry.

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
                message: OwnedParameter::from_serial(&String::from("MyName"))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        contract_address,
        initialization_dummy_contract.contract_address
    );
}

/// Test getter functions.
#[test]
fn test_getter_functions() {
    let (mut chain, initialization_registry) = setup_chain_and_contract();

    let umbrella_feeds_contract = ContractAddress {
        index: 8,
        subindex: 0,
    };

    let input_parameter = ImportAddressesParams {
        entries: vec![ImportAddressesParam {
            name: KEY_HASH_1,
            destination: umbrella_feeds_contract,
        }],
    };

    // Invoking 'importAddresses'.

    let _update = chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            ACC_ADDR_OWNER,         // Invoker account.
            Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
            Energy::from(10000),    // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization_registry.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("registry.importAddresses".into()), // The receive function to call.
                message: OwnedParameter::from_serial(&input_parameter)
                    .expect("`input_parameter` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to importAddresses");

    // Checking `getAddress` function.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(contract_address, umbrella_feeds_contract);

    // Checking `getAddress` function throws for invalid key hash.

    let _invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH_2)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .is_err();

    // Checking `registry` function.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.registry".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(contract_address, umbrella_feeds_contract);

    // Checking `registry` function throws for invalid key hash.

    let _invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.registry".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH_2)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .is_err();

    // Checking `requireAndGetAddress` function.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "registry.requireAndGetAddress".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(contract_address, umbrella_feeds_contract);

    // Checking `requireAndGetAddress` function throws for invalid key hash.

    let _invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "registry.requireAndGetAddress".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_2)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .is_err();
}

/// Test owner functionalities
#[test]
fn test_owner_functionalities() {
    let (mut chain, initialization_registry) = setup_chain_and_contract();

    // Checking `owner`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.owner".to_string()),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query owner address");

    let owner: Address = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(owner, Address::from(ACC_ADDR_OWNER));

    // Invoking 'transferOwnership'.

    let update = chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            ACC_ADDR_OWNER,         // Invoker account.
            Address::Account(ACC_ADDR_OWNER), // Sender (can also be a contract).
            Energy::from(10000),    // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization_registry.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("registry.transferOwnership".into()), // The receive function to call.
                message: OwnedParameter::from_serial(&Address::from(OTHER_ACCOUNT))
                    .expect("`input_parameter` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to transferOwnership");

    // Checking logged event.
    let events: Vec<(ContractAddress, &[ContractEvent])> = update.events().collect();
    let event = &events[0].1[0];

    // Checking event tag.
    assert_eq!(event.as_ref()[0], 1, "Event tag is wrong");

    // Removing the tag byte at the beginning of the event.
    let event_struct: OwnershipTransferredEvent =
        from_bytes(&event.as_ref()[1..]).expect("Tag removal should work");

    assert_eq!(
        event_struct,
        OwnershipTransferredEvent {
            new_owner: Address::from(OTHER_ACCOUNT),
            previous_owner: Address::from(ACC_ADDR_OWNER),
        },
        "OwnershipTransferredEvent event is wrong"
    );

    // Checking `owner`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.owner".to_string()),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query owner address");

    let owner: Address = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(owner, Address::from(OTHER_ACCOUNT));

    // Invoking 'renounceOwnership'.

    let update = chain
        .contract_update(
            Signer::with_one_key(), // Used for specifying the number of signatures.
            OTHER_ACCOUNT,          // Invoker account.
            Address::Account(OTHER_ACCOUNT), // Sender (can also be a contract).
            Energy::from(10000),    // Maximum energy allowed for the update.
            UpdateContractPayload {
                address: initialization_registry.contract_address, // The contract to update.
                receive_name: OwnedReceiveName::new_unchecked("registry.renounceOwnership".into()), // The receive function to call.
                message: OwnedParameter::from_serial(&Address::from(OTHER_ACCOUNT))
                    .expect("`input_parameter` should be a valid inut parameter"), // The parameter sent to the contract.
                amount: Amount::from_ccd(0), // Sending the contract 0 CCD.
            },
        )
        .expect("Should be able to transferOwnership");

    // Checking logged event.
    let events: Vec<(ContractAddress, &[ContractEvent])> = update.events().collect();
    let event = &events[0].1[0];

    // Checking event tag.
    assert_eq!(event.as_ref()[0], 1, "Event tag is wrong");

    // Removing the tag byte at the beginning of the event.
    let event_struct: OwnershipTransferredEvent =
        from_bytes(&event.as_ref()[1..]).expect("Tag removal should work");

    assert_eq!(
        event_struct,
        OwnershipTransferredEvent {
            new_owner: Address::from(AccountAddress([0u8; 32])),
            previous_owner: Address::from(OTHER_ACCOUNT),
        },
        "OwnershipTransferredEvent event is wrong"
    );

    // Checking `owner`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.owner".to_string()),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query owner address");

    let owner: Address = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(owner, Address::from(AccountAddress([0u8; 32])));
}
