use concordium_smart_contract_testing::*;
use concordium_std::{HashSha2256, PublicKeyEd25519};

const ACC_ADDR_OWNER: AccountAddress = AccountAddress([77u8; 32]);

// ATTENTION: Use a different key in production. This key and its private key is exposed and used for testing here.
// Private key: 8ECA45107A878FB879B84401084B55AD4919FC0F7D14E8915D8A5989B1AE1C01
const VALIDATOR_0: PublicKeyEd25519 = PublicKeyEd25519([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

// ATTENTION: Use a different key in production. This key and its private key is exposed and used for testing here.
// Private key: 12827BE279AA7DB7400E9322824CF3C7D5D599005836FDA506351B9B340838A9
const VALIDATOR_1: PublicKeyEd25519 = PublicKeyEd25519([
    217, 108, 75, 18, 24, 234, 126, 194, 15, 70, 4, 214, 194, 240, 47, 163, 243, 107, 81, 132, 67,
    243, 162, 209, 78, 136, 94, 127, 247, 21, 222, 221,
]);

const VALIDATOR_DOES_NOT_EXIST: PublicKeyEd25519 = PublicKeyEd25519([
    000, 108, 75, 18, 24, 234, 126, 194, 15, 70, 4, 214, 194, 240, 47, 163, 243, 107, 81, 132, 67,
    243, 162, 209, 78, 136, 94, 127, 247, 21, 222, 221,
]);

const ACC_INITIAL_BALANCE: Amount = Amount::from_ccd(1000);

const KEY_HASH_1: HashSha2256 = HashSha2256([
    120, 154, 141, 6, 248, 239, 77, 224, 80, 62, 139, 136, 211, 204, 105, 208, 26, 11, 2, 208, 195,
    253, 29, 192, 126, 199, 208, 39, 69, 4, 246, 32,
]);

fn setup_chain_and_contract() -> (Chain, ContractInitSuccess) {
    let mut chain = Chain::new();

    // Creating accounts.
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
                message: OwnedParameter::from_serial(&vec![VALIDATOR_DOES_NOT_EXIST, VALIDATOR_0])
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
                message: OwnedParameter::from_serial(&vec![VALIDATOR_0, VALIDATOR_1])
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
                message: OwnedParameter::from_serial(&VALIDATOR_0)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query the balance");

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 1);

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.balances".to_string()),
                message: OwnedParameter::from_serial(&VALIDATOR_DOES_NOT_EXIST)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query the balance");

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

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

    let value: Vec<u8> = from_bytes(&invoke.return_value).expect("Should return a valid result");

    let one = 1u8;

    assert_eq!(value, vec![one, one]);

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
                message: OwnedParameter::from_serial(&VALIDATOR_0)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query the balance");

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 1u8);
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
                message: OwnedParameter::from_serial(&VALIDATOR_0)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query validator");

    let state: (PublicKeyEd25519, String) =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(
        state,
        (
            VALIDATOR_0,
            String::from("https://validator.dev.umb.network")
        )
    );

    // Checking publicKey.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked("staking_bank.publicKey".to_string()),
                message: OwnedParameter::from_serial(&0u8)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query validator");

    let value: PublicKeyEd25519 =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, VALIDATOR_0);
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

    let value: String = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, String::from("StakingBank"));
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

    assert_eq!(value, 2);

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

    assert_eq!(value, 2);

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

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 2);

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

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 2);

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

    let value: u8 = from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, 1);

    // Checking `getPublicKeys`.

    let invoke = chain
        .contract_invoke(
            ACC_ADDR_OWNER,
            Address::Account(ACC_ADDR_OWNER),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                address: initialization_staking_bank.contract_address,
                receive_name: OwnedReceiveName::new_unchecked(
                    "staking_bank.getPublicKeys".to_string(),
                ),
                message: OwnedParameter::from_serial(&KEY_HASH_1)
                    .expect("Should be a valid inut parameter"),
            },
        )
        .expect("Should be able to query value");

    let value: [PublicKeyEd25519; 2] =
        from_bytes(&invoke.return_value).expect("Should return a valid result");

    assert_eq!(value, [VALIDATOR_0, VALIDATOR_1]);
}
