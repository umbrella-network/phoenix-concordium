## Registry contract

The umbrella oracle protocol uses this registry to fetch current contract addresses. The contract has an owner.

The owner can:
- Register contracts into this registry with the `importAddresses` and the `importContracts` entry points.
- Natively upgrade the `UmbrellaFeeds` contract via this registry contract by invoking the `atomicUpdate` entry point.
- Override contract addresses registered (e.g. in case they don't have the entry points `upgradeNatively` implemented) by invoking the `importAddresses` and the `importContracts` entry points.

ATTENTION: If you want to upgrade the `UmbrellaFeeds` contract, use the `atomicUpdate` function to natively upgrade the `UmbrellaFeeds` contract.
Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include references to the `UmbrellaFeeds` contract.

ATTENTION: The remaining contracts of the protocol (`registry`, `staking_bank` as well as `umbrell_feeds_reader` contracts) can never be replaced/updated without breaking this protocol (meaning breaking this protocol in the viewpoint of immutable contracts that are listening to these umbrella contracts for price information). It is not recommended to override these contract addresses in the registry in the future. The override feature is still kept for consistency since it was present in the original solidity contracts that were translated.
