# Concordium Umbrella oracle.

This is a temporary development repository to develop smart contracts for the Umbrella oracle on Concordium.

The protocol consists of four smart contract folders:

- `Registry`
- `StakingBank`
- `UmbrellaFeeds`
- `UmbrellaFeedsReader`

# Compiling the contracts

In each of the above contract folders, you can build the smart contract (with its embedded schema) with the following command (except for the `StakingBank` contract):

```cargo concordium build -e```

Note: The `StakingBank` contract needs to be built for its respective environment with the `--features` flag:

```cargo concordium build -e -- --features production```

```cargo concordium build -e -- --features development```

```cargo concordium build -e -- --features sandbox```

# Testing the contracts

In each of the above contract folders, you can run the integration test with the following command (except for the `StakingBank` contract):

```cargo concordium test```

To test the `StakingBank` contract use the following command:

```cargo concordium test -- --features production```

# Using the makeFile
 
You can execute from the root of this folder the following commands via the `makeFile` to simplify development and testing:

```make build-all``` to build all contracts.

```make test-all``` to run all tests.

```make fmt-all``` to run the formatter over all contracts and tests.

```make clippy-all``` to run the linter over all contracts and tests.
