## Registry contract

The umbrella oracle protocol uses this registry to fetch current contract addresses. The contract has an owner.

The owner can:
- Register contracts into this registry with the `importAddresses` and the `importContracts` entry points.
- Natively upgrade the `UmbrellaFeeds` contract via this registry contract by invoking the `atomicUpdate` entry point.
- Override contract addresses registered (e.g. in case they don't have the entry points `upgradeNatively` implemented) by invoking the `importAddresses` and the `importContracts` entry points.

ATTENTION: 
- The `registry` will never be upgraded. 
- If you want to upgrade the `UmbrellaFeeds` contract, use the `atomicUpdate` function to natively upgrade the `UmbrellaFeeds` contract.
Using the native upgradability mechanism for the `UmbrellaFeeds` contract is necessary to not break the `UmbrellaFeedsReader` contracts which include references to the `UmbrellaFeeds` contract.
- The `stakingBank` might be re-deployed and replaced in the registry contract. If this happens also the `stakingBank` variable in the `UmbrellaFeeds` contract should be updated (via an `atomicUpdate`).
- The `UmbrellaFeedsReader` contracts are not meant to be replaced. They work with up-to-date data as long as the `UmbrellaFeeds` contract has not been replaced in the registry (only been upgraded via the `atomicUpdate`). It is not the intention to replace the `UmbrellaFeeds` contract in the future but we keep that option in the `registry` to replace the `UmbrellaFeeds` contract in case something goes wrong during an upgrade. We recommend integrating protocols to keep that in mind and have a possibility to adjust their protocol to the active contract address as it is registered in the `registry`. 
