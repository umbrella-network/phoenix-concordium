## Staking bank contract

The staking bank is a decentralised registry of validators.

Setup addresses in bank by providing public key as `number[]`. 

To translate verifyKey to numbers:
- https://cryptii.com/pipes/integer-encoder
- Buffer.from('<verifyKey>', 'hex').toJSON().data


### Sending founds using concordium-client

https://developer.concordium.software/en/mainnet/net/references/transactions.html

`concordium-client config show` this will show accounts + aliases

```
concordium-client account show UMB_Production --grpc-ip <url> --grpc-port 20000 --secure
concordium-client account show 46eTEZwu45dFV2ByhWfDh2sNJg2hLHL6bPwaM398NAeJM7TG3L --grpc-ip concordium.prod.umb.network --grpc-port 20000 --secure
```

```shell
concordium-client transaction send \
--sender UMB_Production \
--amount 1 --receiver 46eTEZwu45dFV2ByhWfDh2sNJg2hLHL6bPwaM398NAeJM7TG3L \
--grpc-ip <url> --grpc-port 20000 --secure
```
