## Staking bank contract

The staking bank is a decentralised registry of validators.

Setup addresses in bank by providing public key as `number[]`.

To translate verifyKey to numbers:

- https://cryptii.com/pipes/integer-encoder
- Buffer.from('<verifyKey>', 'hex').toJSON().data

### Updating list of validators

1. open regular PR
2. update constants with keys in `staking-bank/src/production_constants.rs`
3. make sure number of validators matches new count, there are few places where update needs to be done, search
   for `#update-count`
4. create verifiable build (it will review closing PR with changes), see deployments README for next steps.

### Sending founds using concordium-client

https://developer.concordium.software/en/mainnet/net/references/transactions.html

`concordium-client config show` this will show accounts + aliases

to check balances:
```
concordium-client account show 46eTEZwu45dFV2ByhWfDh2sNJg2hLHL6bPwaM398NAeJM7TG3L --grpc-ip <url> --grpc-port 20000 --secure
```


to transfer tokens:

```shell
concordium-client transaction send \
--sender UMB_Production \
--amount 1 --receiver 46eTEZwu45dFV2ByhWfDh2sNJg2hLHL6bPwaM398NAeJM7TG3L \
--grpc-ip <url> --grpc-port 20000 --secure
```
