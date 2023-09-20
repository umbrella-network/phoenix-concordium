use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use registry::LogRegisteredEvent;
use registry::{ImportAddressesParam, ImportAddressesParams, OwnershipTransferredEvent};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([77u8; 32]);
const OTHER_ACCOUNT: AccountAddress = AccountAddress([1u8; 32]);

const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const KEY_HASH: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

fn setup_chain_and_contract() -> (Chain, ContractInitSuccess) {
    let mut chain = Chain::new();

    // Creating accounts
    chain.create_account(Account::new(ACC_ADDR_OWNER, ACC_INITIAL_BALANCE));
    chain.create_account(Account::new(OTHER_ACCOUNT, ACC_INITIAL_BALANCE));

    // Deploying 'registry' contract

    let deployment_registry = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            ACC_ADDR_OWNER,
            module_load_v1("./registry.wasm.v1")
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

    (chain, initialization_registry)
}

#[test]
fn test_init() {
    let (_chain, initialization_registry) = setup_chain_and_contract();

    // Check logged event.
    let events = initialization_registry.events;
    let event = &events[0];

    // Check event tag.
    assert_eq!(event.as_ref()[0], 1, "Event tag is wrong");

    // Remove the tag byte at the beginning of the event.
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

/// Test `importAddresses` function
#[test]
fn test_import_addresses() {
    let (mut chain, initialization_registry) = setup_chain_and_contract();

    let umbrella_feeds_contract = ContractAddress {
        index: 8,
        subindex: 0,
    };

    let input_parameter = ImportAddressesParams {
        entries: vec![ImportAddressesParam {
            name: KEY_HASH,
            destination: umbrella_feeds_contract,
        }],
    };

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

    // Check logged event.
    let events: Vec<(ContractAddress, &[ContractEvent])> = update.events().collect();
    let event = &events[0].1[0];

    // Check event tag.
    assert_eq!(event.as_ref()[0], 0, "Event tag is wrong");

    // Remove the tag byte at the beginning of the event.
    let event_struct: LogRegisteredEvent =
        from_bytes(&event.as_ref()[1..]).expect("Tag removal should work");

    assert_eq!(
        event_struct,
        LogRegisteredEvent {
            name: KEY_HASH,
            destination: umbrella_feeds_contract,
        },
        "LogRegistered event is wrong"
    );

    // Checking that contract address was registered correctly in registry

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_registry.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query contract address");

    let contract_address: ContractAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(contract_address, umbrella_feeds_contract);
}
