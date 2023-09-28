use concordium_smart_contract_testing::*;
use concordium_std::HashSha2256;
use sha256::digest;

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([77u8; 32]);
const OTHER_ACCOUNT: AccountAddress = AccountAddress([1u8; 32]);

const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const KEY_HASH_1: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

fn setup_chain_and_contract() -> (Chain, ContractInitSuccess) {
    let mut chain = Chain::new();

    // Creating accounts.
    chain.create_account(Account::new(ACC_ADDR_OWNER, ACC_INITIAL_BALANCE));
    chain.create_account(Account::new(OTHER_ACCOUNT, ACC_INITIAL_BALANCE));

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
        .expect("Initialization of `Staking_bank` should always succeed");

    (chain, initialization_staking_bank)
}

#[test]
fn test_verify_validators() {
    let (chain, initialization_staking_bank) = setup_chain_and_contract();

    // Checking verifyValidators.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.verifyValidators".to_string(),
                ),
                message: OwnedParameter::from_serial(&vec![
                    AccountAddress([4u8; 32]),
                    AccountAddress([99u8; 32]),
                ])
                .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query if address is validator");

    let value: bool = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, false);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.verifyValidators".to_string(),
                ),
                message: OwnedParameter::from_serial(&vec![
                    AccountAddress([4u8; 32]),
                    AccountAddress([7u8; 32]),
                ])
                .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query if address is validator");

    let value: bool = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, true);
}

#[test]
fn test_balances() {
    let (chain, initialization_staking_bank) = setup_chain_and_contract();

    // Checking balances.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.balances".to_string()),
                message: OwnedParameter::from_serial(&AccountAddress([2u8; 32]))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query the balance");

    let value: u64 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 1000000000000000000);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.balances".to_string()),
                message: OwnedParameter::from_serial(&AccountAddress([99u8; 32]))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query the balance");

    let value: u64 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 0);

    // Checking getBalances.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.getBalances".to_string(),
                ),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query the balance");

    let value: Vec<u64> = from_bytes(&invoke.return_value).expect("Should return a valid result");

    let one = 1000000000000000000u64;

    assert_eq!(
        value,
        vec![one, one, one, one, one, one, one, one, one, one, one, one, one, one, one]
    );

    // Checking balanceOf.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.balanceOf".to_string()),
                message: OwnedParameter::from_serial(&AccountAddress([3u8; 32]))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query the balance");

    let value: u64 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 1000000000000000000u64);
}

#[test]
fn test_validators() {
    let (chain, initialization_staking_bank) = setup_chain_and_contract();

    // Checking validators.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.validators".to_string(),
                ),
                message: OwnedParameter::from_serial(&AccountAddress([2u8; 32]))
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query validator");

    let state: (AccountAddress, String) =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        state,
        (
            AccountAddress([2u8; 32]),
            String::from("https://umbrella.artemahr.tech")
        )
    );

    // Checking addresses.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.addresses".to_string()),
                message: OwnedParameter::from_serial(&3u8)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query validator");

    let state: AccountAddress =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(state, AccountAddress([3u8; 32]));
}

#[test]
fn test_get_name() {
    let (chain, initialization_staking_bank) = setup_chain_and_contract();

    // Checking getName.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.getName".to_string()),
                message: OwnedParameter::empty(),
            },
        )
        .expect("Should be able to query contract state");

    let value: HashSha2256 =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, digest(String::from("StakingBank")).parse().unwrap());
}

#[test]
fn test_init() {
    let (chain, initialization_staking_bank) = setup_chain_and_contract();

    // Checking `NUMBER_OF_VALIDATORS`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.NUMBER_OF_VALIDATORS".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query value");

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 15);

    // Checking `getNumberOfValidators`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.getNumberOfValidators".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query value");

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 15);

    // Checking `TOTAL_SUPPLY`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.TOTAL_SUPPLY".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query value");

    let value: u64 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 15000000000000000000);

    // Checking `totalSupply`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.totalSupply".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query value");

    let value: u64 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 15000000000000000000);

    // Checking `ONE`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.ONE".to_string()),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query value");

    let value: u64 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 1000000000000000000);

    // Checking `getAddresses`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.getAddresses".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query value");

    let value: [AccountAddress; 15] =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        value,
        [
            AccountAddress([0u8; 32]),
            AccountAddress([1u8; 32]),
            AccountAddress([2u8; 32]),
            AccountAddress([3u8; 32]),
            AccountAddress([4u8; 32]),
            AccountAddress([5u8; 32]),
            AccountAddress([6u8; 32]),
            AccountAddress([7u8; 32]),
            AccountAddress([8u8; 32]),
            AccountAddress([9u8; 32]),
            AccountAddress([10u8; 32]),
            AccountAddress([11u8; 32]),
            AccountAddress([12u8; 32]),
            AccountAddress([13u8; 32]),
            AccountAddress([14u8; 32])
        ]
    );
}
