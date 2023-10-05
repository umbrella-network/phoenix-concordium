## Umbrella feeds contract

This is the main contract for all on-chain data. 
Relative prices are stored in this contract for different price feeds (not absolute prices). 
E.g. for the `ETH-USDC` price feed, the key will be the String `"ETH-USDC"`. The prices map can be queried with the key `"ETH-USDC"` to get the associated price data.

```
pub struct PriceData {
    /// This is a placeholder, that can be used for some additional data.
    pub data: u8,
    /// The heartbeat specifies the interval in seconds that the price data will be refreshed in case the price stays flat.
    pub heartbeat: u64,
    /// It is the time when the validators run the consensus to decide on the price data.
    pub timestamp: Timestamp,
    /// The price.
    pub price: u128,
}
```

The prices stored in the `priceDate` are relative prices. E.g. for the price feed `ETH-USDC`, the relative price stored in the smart contract is the value `ETH/USDC` (not `USDC/ETH`). Meaning the first token name is in the `numerator (base)` and the second token name is in the `denominator (quote)`.

The `UmbrellaFeedsReader` can be used to integrate easier (see its documentation) but it is recommended to integrate directly into this contract for a lower execution cost.

ATTENTION: Keep the `upgradeNatively`/`unregister` entry points in this contract at all times and make sure their logic can be
executed successfully via an invoke to the `atomicUpdate` entry point in the `registry` contract. Otherwise, you will not be able to
natively upgrade this contract via the `registry` contract anymore.
Using the native upgradability mechanism for this contract is necessary to not break the `UmbrellaFeedsReader` contracts
which include references to this `UmbrellaFeeds` contract.
