# Concordium Umbrella oracle integration example

# Compiling the integration contract

Note: The contract needs to be built for its respective environment with the `--features` flag:

Run one of the commands in this folder:

```cargo concordium build -e -o ./oracle_integration.wasm.v1 -- --features production```

```cargo concordium build -e -o ./oracle_integration.wasm.v1 -- --features development```

```cargo concordium build -e -o ./oracle_integration.wasm.v1 -- --features local``` (local environment for running integration tests)

# Testing the integration contract

Run the following two commands in this folder:

```cargo concordium build -e -o ./oracle_integration.wasm.v1 -- --features local```

```cargo concordium test```

# Price feed information

This contract retrieves the relative prices (not absolute prices) from the Umbrella oracle for various price feeds. The `update_price` function can be invoked by anyone with a specific price feed parameter, such as `ETH-USDC`. This function fetches and stores the most recent relative price for the requested price feed in the contract. For instance, when dealing with the `ETH-USDC` price feed, you would call the `update_price` function with the parameter `ETH-USDC`. If the price data retrieved from the oracle is not up-to-date, the `update_price` function will revert with the `PriceNotUpToDate` error. 

It's crucial to note that the stored prices represent relative values. For the `ETH-USDC` price feed, the stored relative price in the smart contract corresponds to the value `ETH_Price/USDC_Price` (not `USDC_Price/ETH_Price`). In simpler terms, the first token name is in the numerator (base), and the second token name is in the denominator (quote).

To correctly interpret the prices, it is recommended to query the decimal value from the `umbrella_feeds` contract using the `DECIMALS` function, which returns a value of type `u8`. For example, at the time of writing the `decimal` value on testnet is 8. For example, if the price of `ETH` is 3500.80 USD (3500 USD and 80 cents) and of `USDC` is 1 USD, then the price of the `ETH-USDC` price feed is `ETH_Price/USDC_Price = 3500.80/1` which is represented with 8 decimals as `350_080_000_000` in the smart contract.

