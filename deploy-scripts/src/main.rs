pub mod deployer;
use anyhow::{bail, Context, Error};
use concordium_rust_sdk::{
    common::types::Amount,
    smart_contracts::{
        common::{self as contracts_common, Deserial, OwnedEntrypointName, ParseResult},
        engine::v1::ReturnValue,
        types::{
            InvokeContractResult::{Failure, Success},
            OwnedContractName, OwnedParameter, OwnedReceiveName,
        },
    },
    types::{
        smart_contracts::{ContractContext, ModuleReference, WasmModule, DEFAULT_INVOKE_ENERGY},
        transactions,
        transactions::InitContractPayload,
        ContractAddress,
    },
    v2::{self, BlockIdentifier},
};
use deployer::{DeployResult, Deployer, InitResult};
use registry::{AtomicUpdateParam, ImportContractsParam};
use std::{
    io::Cursor,
    path::{Path, PathBuf},
};
use structopt::{clap::AppSettings, StructOpt};
use umbrella_feeds::InitParamsUmbrellaFeeds;

/// Reads the wasm module from a given file path.
fn get_wasm_module(file: &Path) -> Result<WasmModule, Error> {
    let wasm_module = std::fs::read(file).context("Could not read the WASM file")?;
    let mut cursor = Cursor::new(wasm_module);
    let wasm_module: WasmModule = concordium_rust_sdk::common::from_bytes(&mut cursor)?;
    Ok(wasm_module)
}

/// Try to parse the return value into a type that implements [`Deserial`].
/// Ensures that all bytes of the return value are read.
pub fn parse_return_value<T: Deserial>(return_value: ReturnValue) -> ParseResult<T> {
    use contracts_common::{Cursor, Get, ParseError};
    let mut cursor = Cursor::new(return_value.clone());
    let res = cursor.get()?;
    // Check that all bytes have been read, as leftover bytes usually indicate
    // errors.
    if cursor.offset != return_value.len() {
        return Err(ParseError::default());
    }
    Ok(res)
}

/// Deploys a wasm module given the path to the file. Returns the module reference of the wasm module.
/// If the wasm module is already deployed on the chain, this function returns the module reference as well but without sending a deployment transaction.
async fn deploy_module(
    deployer: &mut Deployer,
    wasm_module_path: &Path,
) -> Result<ModuleReference, Error> {
    let wasm_module = get_wasm_module(wasm_module_path)?;

    let deploy_result = deployer
        .deploy_wasm_module(wasm_module, None)
        .await
        .context("Failed to deploy module `{wasm_module_path:?}`.")?;

    let module_reference = match deploy_result {
        DeployResult::ModuleDeployed(module_deploy_result) => module_deploy_result.module_reference,
        DeployResult::ModuleExists(module_reference) => module_reference,
    };

    Ok(module_reference)
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Deployment and update scripts.")]
enum Command {
    #[structopt(
        name = "deploy",
        about = "Deploy and set up the umbrella oracle protocol."
    )]
    DeployState {
        #[structopt(
            long = "node",
            default_value = "http://node.testnet.concordium.com:20000",
            help = "V2 API of the Concordium node."
        )]
        url: v2::Endpoint,
        #[structopt(
            long = "account",
            help = "Path to the file containing the Concordium account keys exported from the wallet \
                    (e.g. ./myPath/3PXwJYYPf6fyVb4GJquxSZU8puxrHfzc4XogdMVot8MUQK53tW.export)."
        )]
        key_file: PathBuf,
        #[structopt(
            long = "required_signatures",
            help = "Minimal number of signatures required for accepting price submission in the umbrella feeds contract."
        )]
        required_signatures: u16,
        #[structopt(
            long = "decimals",
            help = "Decimals for prices stored in the umbrella feeds contract."
        )]
        decimals: u8,
    },
    #[structopt(
        name = "register",
        about = "Register a list of contracts in the regisry."
    )]
    Register {
        #[structopt(
            long = "node",
            default_value = "http://node.testnet.concordium.com:20000",
            help = "V2 API of the Concordium node."
        )]
        url: v2::Endpoint,
        #[structopt(
            long = "account",
            help = "Path to the file containing the Concordium account keys exported from the wallet \
                    (e.g. ./myPath/3PXwJYYPf6fyVb4GJquxSZU8puxrHfzc4XogdMVot8MUQK53tW.export)."
        )]
        key_file: PathBuf,
        #[structopt(
            long = "registry",
            help = "Contract address of the registry (e.g. --registry \"<7074,0>\")."
        )]
        registry_contract: ContractAddress,
        #[structopt(
            long = "contract",
            help = "Contract address to be registered in the registry. Use this flag several times if you \
            have several smart contracts to be registered (e.g. --contract \
                \"<7075,0>\" --contract \"<7076,0>\")."
        )]
        contract: Vec<ContractAddress>,
    },
    #[structopt(
        name = "upgrade_staking_bank_contract",
        about = "Upgrade staking bank contract."
    )]
    UpgradeStakingBankState {
        #[structopt(
            long = "node",
            default_value = "http://node.testnet.concordium.com:20000",
            help = "V2 API of the Concordium node."
        )]
        url: v2::Endpoint,
        #[structopt(
            long = "account",
            help = "Path to the file containing the Concordium account keys exported from the wallet \
                    (e.g. ./myPath/3PXwJYYPf6fyVb4GJquxSZU8puxrHfzc4XogdMVot8MUQK53tW.export)."
        )]
        key_file: PathBuf,
        #[structopt(
            long = "registry",
            help = "Contract address of the registry (e.g. --registry \"<7074,0>\")."
        )]
        registry_contract: ContractAddress,
        #[structopt(
            long = "new_staking_bank",
            help = "Path to the new staking_bank module (e.g. --new_staking_bank ./new_staking_bank.wasm.v1)."
        )]
        new_staking_bank: PathBuf,
    },
    #[structopt(
        name = "upgrade_umbrella_feeds_contract",
        about = "Upgrade umbrella feeds contract."
    )]
    UpgradeUmbrellaFeeds {
        #[structopt(
            long = "node",
            default_value = "http://node.testnet.concordium.com:20000",
            help = "V2 API of the Concordium node."
        )]
        url: v2::Endpoint,
        #[structopt(
            long = "account",
            help = "Path to the file containing the Concordium account keys exported from the wallet \
                    (e.g. ./myPath/3PXwJYYPf6fyVb4GJquxSZU8puxrHfzc4XogdMVot8MUQK53tW.export)."
        )]
        key_file: PathBuf,
        #[structopt(
            long = "registry",
            help = "Contract address of the registry (e.g. --registry \"<7074,0>\")."
        )]
        registry_contract: ContractAddress,
        #[structopt(
            long = "new_umbrella_feeds",
            help = "Path to the new umbrella_feeds module (e.g. --new_umbrella_feeds ./new_umbrella_feeds.wasm.v1)."
        )]
        new_umbrella_feeds: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cmd = {
        let app = Command::clap()
            .setting(AppSettings::ArgRequiredElseHelp)
            .global_setting(AppSettings::ColoredHelp);
        let matches = app.get_matches();

        Command::from_clap(&matches)
    };

    match cmd {
        // Deploying a new umbrella oracle protocol
        Command::DeployState {
            url,
            key_file,
            required_signatures,
            decimals,
        } => {
            // Setting up the connection
            let concordium_client = v2::Client::new(url).await?;

            let mut deployer = Deployer::new(concordium_client, &key_file)?;

            // Deploying registry, umbrella_feeds, and staking_bank wasm modules

            print!("\nDeploying registry module....");
            let registry_module_reference = deploy_module(
                &mut deployer.clone(),
                &PathBuf::from("../registry/registry.wasm.v1"),
            )
            .await?;

            print!("\nDeploying staking_bank module....");
            let staking_bank_module_reference = deploy_module(
                &mut deployer.clone(),
                &PathBuf::from("../staking-bank/staking_bank.wasm.v1"),
            )
            .await?;

            print!("\nDeploying umbrella_feeds module....");
            let umbrella_feeds_module_reference = deploy_module(
                &mut deployer.clone(),
                &PathBuf::from("../umbrella-feeds/umbrella_feeds.wasm.v1"),
            )
            .await?;

            // Initializing registry

            print!("\nInitializing registry contract....");

            let payload = InitContractPayload {
                init_name: OwnedContractName::new("init_registry".into())?,
                amount: Amount::from_micro_ccd(0),
                mod_ref: registry_module_reference,
                param: OwnedParameter::empty(),
            };

            let init_result_registry_contract: InitResult = deployer
                .init_contract(payload, None, None)
                .await
                .context("Failed to initialize the registry contract.")?;

            // Initializing staking_bank

            print!("\nInitializing staking_bank contract....");

            let payload = InitContractPayload {
                init_name: OwnedContractName::new("init_staking_bank".into())?,
                amount: Amount::from_micro_ccd(0),
                mod_ref: staking_bank_module_reference,
                param: OwnedParameter::empty(),
            };

            let init_result_staking_bank: InitResult = deployer
                .init_contract(payload, None, None)
                .await
                .context("Failed to initialize the staking bank contract.")?;

            // Initializing umbrella_feeds

            print!("\nInitializing umbrella_feeds contract....");

            let input_parameter = InitParamsUmbrellaFeeds {
                registry: init_result_registry_contract.contract_address,
                required_signatures,
                staking_bank: init_result_staking_bank.contract_address,
                decimals,
            };

            let payload = InitContractPayload {
                init_name: OwnedContractName::new("init_umbrella_feeds".into())?,
                amount: Amount::from_micro_ccd(0),
                mod_ref: umbrella_feeds_module_reference,
                param: OwnedParameter::from_serial(&input_parameter)?,
            };

            let _init_result: InitResult = deployer
                .init_contract(payload, None, None)
                .await
                .context("Failed to initialize the umbrella feeds contract.")?;
        }
        // Registering the contracts in the registry
        Command::Register {
            url,
            key_file,
            registry_contract,
            contract,
        } => {
            // Setting up the connection
            let concordium_client = v2::Client::new(url).await?;

            let mut deployer = Deployer::new(concordium_client, &key_file)?;

            // Registering the contracts

            let bytes = contracts_common::to_bytes(&ImportContractsParam { entries: contract });

            let update_payload = transactions::UpdateContractPayload {
                amount: Amount::from_ccd(0),
                address: registry_contract,
                receive_name: OwnedReceiveName::new_unchecked(
                    "registry.importContracts".to_string(),
                ),
                message: bytes.try_into()?,
            };

            let _update_contract = deployer
                .update_contract(update_payload, None, None)
                .await
                .context("Failed to register the contracts.")?;
        }
        // Upgrading the staking_bank contract
        Command::UpgradeStakingBankState {
            url,
            key_file,
            registry_contract,
            new_staking_bank,
        } => {
            // Setting up the connection
            let concordium_client = v2::Client::new(url).await?;

            let mut deployer = Deployer::new(concordium_client, &key_file)?;

            // Checking that the module reference is different to the staking_bank module reference registered in the registry

            // Step 1: Getting the module reference from the new staking bank

            let new_wasm_module = get_wasm_module(&new_staking_bank)?;

            let new_module_reference = new_wasm_module.get_module_ref();

            // Step 2: Getting the module reference from the staking bank already registered in the registry

            let bytes = contracts_common::to_bytes(&"StakingBank");

            let payload = transactions::UpdateContractPayload {
                amount: Amount::from_ccd(0),
                address: registry_contract,
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
                message: bytes.try_into()?,
            };

            let context = ContractContext::new_from_payload(
                deployer.key.address,
                DEFAULT_INVOKE_ENERGY,
                payload,
            );

            let result = deployer
                .client
                .invoke_instance(&BlockIdentifier::LastFinal, &context)
                .await
                .context("Failed invoking instance")?;

            let old_staking_contract: ContractAddress = match result.response {
                Success {
                    return_value,
                    events: _,
                    used_energy: _,
                } => {
                    if let Some(return_value) = return_value {
                        parse_return_value::<ContractAddress>(return_value.into())
                            .context("Failed parsing contractAddress")?
                    } else {
                        bail!("Failed no return value");
                    }
                }
                Failure {
                    return_value: _,
                    reason,
                    used_energy: _,
                } => bail!("Failed querying staking bank address from registry: {reason:?}"),
            };

            let info = deployer
                .client
                .get_instance_info(old_staking_contract, &BlockIdentifier::LastFinal)
                .await
                .context("Failed querying instance info")?;

            let old_module_reference = info.response.source_module();

            if old_module_reference == new_module_reference {
                bail!("Failed the new staking bank module reference has to be different from the old staking bank module reference.")
            } else {
                // Deploying new staking_bank wasm modules

                let new_staking_bank_module_reference =
                    deploy_module(&mut deployer.clone(), &new_staking_bank).await?;

                // Initializing staking_bank

                print!("\nInitializing new staking_bank contract....");

                let payload = InitContractPayload {
                    init_name: OwnedContractName::new("init_staking_bank".into())?,
                    amount: Amount::from_micro_ccd(0),
                    mod_ref: new_staking_bank_module_reference,
                    param: OwnedParameter::empty(),
                };

                let init_result_staking_bank: InitResult = deployer
                    .init_contract(payload, None, None)
                    .await
                    .context("Failed to initialize the new staking bank contract.")?;

                // Updating staking bank address in registry contract

                print!("\nUpdating staking bank address in resgistry contract....");

                let bytes = contracts_common::to_bytes(&ImportContractsParam {
                    entries: vec![init_result_staking_bank.contract_address],
                });

                let update_payload = transactions::UpdateContractPayload {
                    amount: Amount::from_ccd(0),
                    address: registry_contract,
                    receive_name: OwnedReceiveName::new_unchecked(
                        "registry.importContracts".to_string(),
                    ),
                    message: bytes.try_into()?,
                };

                let _update_contract = deployer
                    .update_contract(update_payload, None, None)
                    .await
                    .context("Failed to register the contract.")?;
            }
        }
        // Upgrading the umbrella_feeds contract
        Command::UpgradeUmbrellaFeeds {
            url,
            key_file,
            registry_contract,
            new_umbrella_feeds,
        } => {
            // Setting up the connection
            let concordium_client = v2::Client::new(url).await?;

            let mut deployer = Deployer::new(concordium_client, &key_file)?;

            // Checking that the module reference is different from the umbrella_feeds module reference registered in the registry

            // Step 1: Getting the module reference from the new umbrella feeds contract

            let new_wasm_module = get_wasm_module(&new_umbrella_feeds)?;

            let new_module_reference = new_wasm_module.get_module_ref();

            // Step 2: Getting the module reference from the umbrella feeds contract already registered in the registry

            let bytes = contracts_common::to_bytes(&"UmbrellaFeeds");

            let payload = transactions::UpdateContractPayload {
                amount: Amount::from_ccd(0),
                address: registry_contract,
                receive_name: OwnedReceiveName::new_unchecked("registry.getAddress".to_string()),
                message: bytes.try_into()?,
            };

            let context = ContractContext::new_from_payload(
                deployer.key.address,
                DEFAULT_INVOKE_ENERGY,
                payload,
            );

            let result = deployer
                .client
                .invoke_instance(&BlockIdentifier::LastFinal, &context)
                .await
                .context("Failed invoking instance")?;

            let old_umbrella_feeds_contract: ContractAddress = match result.response {
                Success {
                    return_value,
                    events: _,
                    used_energy: _,
                } => {
                    if let Some(return_value) = return_value {
                        parse_return_value::<ContractAddress>(return_value.into())
                            .context("Failed parsing contractAddress")?
                    } else {
                        bail!("Failed no return value");
                    }
                }
                Failure {
                    return_value: _,
                    reason,
                    used_energy: _,
                } => bail!("Failed querying umbrella feeds address from registry: {reason:?}"),
            };

            let info = deployer
                .client
                .get_instance_info(old_umbrella_feeds_contract, &BlockIdentifier::LastFinal)
                .await
                .context("Failed querying instance info")?;

            let old_module_reference = info.response.source_module();

            if old_module_reference == new_module_reference {
                bail!("Failed the new umbrella feeds module reference has to be different from the old umbrella feeds module reference.")
            } else {
                // Deploying new umbrella feeds wasm modules

                let new_umbrella_feeds_module_reference =
                    deploy_module(&mut deployer.clone(), &new_umbrella_feeds).await?;

                // Natively upgrade umbrella feeds contract via registry

                print!("\nNatively upgrade umbrella feeds contract via registry....");

                let bytes = contracts_common::to_bytes(&AtomicUpdateParam {
                    module: new_umbrella_feeds_module_reference,
                    migrate: Some((
                        OwnedEntrypointName::new_unchecked("migration".to_string()),
                        OwnedParameter::empty(),
                    )),
                    contract_address: old_umbrella_feeds_contract,
                });

                let update_payload = transactions::UpdateContractPayload {
                    amount: Amount::from_ccd(0),
                    address: registry_contract,
                    receive_name: OwnedReceiveName::new_unchecked(
                        "registry.atomicUpdate".to_string(),
                    ),
                    message: bytes.try_into()?,
                };

                let _update_contract =
                    deployer
                        .update_contract(update_payload, None, None)
                        .await
                        .context("Failed to natively upgrade the umbrella feeds contract.")?;
            }
        }
    };
    Ok(())
}
