# Scripts

This project contains scripts:

- to deploy the whole protocol (deploying the `registry`, `staking_bank`, and `umbrella_feeds` contracts and setting up the protocol)
- to register contracts in the `registry` (registering a list of contracts in the `registry` contract using the `importContracts` entry point)
- to upgrade the `staking_bank` contract (checking that the new `staking_bank` module reference differs from the old one. If yes, deploying and initializing a new `staking_bank` and registering it in the `registry` contract using the `importContracts` entry point). ATTENTION: After upgrading the `staking_bank` contract, you also have to upgrade the `umbrella_feeds` contract so it can pick up the new `staking_bank` address from the registry.
- to upgrade the `umbrella_feeds` contract (checking that the new `umbrella_feeds` module reference differs from the old one. If yes, deploying the new `umbrella_feeds` module and natively upgrade the old umbrella feeds contract with it via the `registry` contract using the `atomicUpdate` entry point)

# Running The Scripts

Build and run the scripts from the deploy-scripts folder using
```
cargo run <subcommand> <options>
```

To explore available subcommands, use the `help` flag:
```
cargo run -- --help
```

To explore available options for the e.g. subcommand `deploy`, use the `help` flag:
```
cargo run deploy --help
```

# Examples

## To deploy a new umbrella oracle protocol:

Compile your contracts for the respective environment by executing one of the commands in the root folder of this project:
```
make build-all-production
make build-all-development
make build-all-sandbox
```

Execute the deployment script in this folder to set up the protocol with the above-compiled contracts (an example command is shown below):
```
cargo run deploy --node http://node.testnet.concordium.com:20000 --account ./4SizPU2ipqQQza9Xa6fUkQBCDjyd1vTNUNDGbBeiRGpaJQc6qX.export --required_signatures 3 --decimals 8
```

## To register contracts in the `registry` contract:

Execute the register script in this folder (an example command is shown below):

```
cargo run register --node http://node.testnet.concordium.com:20000 --account ./4SizPU2ipqQQza9Xa6fUkQBCDjyd1vTNUNDGbBeiRGpaJQc6qX.export --registry "<7074,0>" --contract "<7076,0>" --contract "<7075,0>" 
```

## To upgrade the `staking_bank` contract:

Compile a new `staking_bank` contract.

Execute the upgrade script in this folder (an example command is shown below):

```
cargo run upgrade_staking_bank_contract --node http://node.testnet.concordium.com:20000 --account ./4SizPU2ipqQQza9Xa6fUkQBCDjyd1vTNUNDGbBeiRGpaJQc6qX.export --registry "<7074,0>" --new_staking_bank ../staking-bank/staking_bank.wasm.v1
```

## To upgrade the `umbrella_feeds` contract:

Compile a new `umbrella_feeds` contract.

Execute the upgrade script in this folder (an example command is shown below):

```
cargo run upgrade_umbrella_feeds_contract --node http://node.testnet.concordium.com:20000 --account ./4SizPU2ipqQQza9Xa6fUkQBCDjyd1vTNUNDGbBeiRGpaJQc6qX.export --registry "<7074,0>" --new_umbrella_feeds ../umbrella-feeds/umbrella_feeds.wasm.v1
```

Note: The `account` parameter should be a Concordium wallet account either exported from the
browser wallet or the mobile wallets, or in the format emitted by the
genesis tool.

The outputs of the above commands should be similar to:

```
Deploying module....
Module with reference 3774d4b9ae86ae3c5192e13455d7515073f5163a25deabc55abdab31d1cc002e already exists on the chain.

Initializing contract....
Sent transaction with hash: bdb43d1f00a4c5ba02ec81e0e2da52b6920582a16acd21a364ec3e3734ad4f12
Transaction finalized: tx_hash=bdb43d1f00a4c5ba02ec81e0e2da52b6920582a16acd21a364ec3e3734ad4f12 contract=(7000, 0)

Updating contract....
Sent transaction with hash: 4843efc3b700bce8e67f2cc3f17da3124cf0a7323652fb778412ecd768ae2fe5
Transaction finalized: tx_hash=4843efc3b700bce8e67f2cc3f17da3124cf0a7323652fb778412ecd768ae2fe5
```
