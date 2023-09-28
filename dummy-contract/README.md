## Dummy contract

This contract is only used in the integration tests. 

In some tests (e.g. the `registry` integration tests), we need a contract that can be invoked with the `getName` entry point. We cannot use the other available smart contracts because we cause a circular dependency in the `Cargo.toml` file of the `registry` contract if added.