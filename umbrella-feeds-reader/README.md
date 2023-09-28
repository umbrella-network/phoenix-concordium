## Umbrella feeds reader contract

This is an optional price reader contract for just one feed that can be deployed by dApp developers who want to integrate into the Umbrella oracle system. For maximum gas optimization, it is recommended to use `UmbrellaFeeds` directly.

The purpose of this contract on Ethereum was originally to provide the same interface with the `latestRoundData` entry point as the popular `chainlink` oracle protocol for easier integration. This contract is kept for consistency on Concordium but currently, a standard or popular oracle protocol interface does not exist on Concordium that would be worth providing for easier integration here.