# Concordium Umbrella oracle.

This is the Umbrella oracle protocol on Concordium.

The protocol consists of three smart contract folders:

- `Registry`
- `StakingBank`
- `UmbrellaFeeds`

There exists one smart contract folder that showcases the integration into the protocol:

- `OracleIntegration` (This folder has a separate README.md file with additional information)

# Init

https://developer.concordium.software/en/mainnet/smart-contracts/tutorials/setup-env.html

1. https://rustup.rs/
2. new way of install: `cargo install --locked cargo-concordium`

# Compiling the contracts

In each of the above contract folders, you can build the smart contract (with its embedded schema) with the following
command (except for the `StakingBank` contract):

```cargo concordium build -e```

Note: The `StakingBank` contract needs to be built for its respective environment with the `--features` flag:

```cargo concordium build -e -- --features production```

```cargo concordium build -e -- --features development```

```cargo concordium build -e -- --features sandbox```

# Testing the contracts

In each of the above contract folders, you can run the integration test with the following command (except for
the `StakingBank` contract):

```cargo concordium test```

To test the `StakingBank` contract use the following command:

```cargo concordium test -- --features development```

# Using the makeFile

You can execute from the root of this folder the following commands via the `makeFile` to simplify development and
testing:

```make build-all-production``` to build all contracts with production setting.

```make build-all-development``` to build all contracts with development setting.

```make build-all-sandbox``` to build all contracts with sandbox setting.

```make build-all``` to build all contracts (the staking bank is built three times with production, sandbox, and
development setting).

```make test-all``` to run all tests.

```make fmt-all``` to run the formatter over all contracts and tests.

```make clippy-all``` to run the linter over all contracts and tests.
