## Multisig

https://gist.github.com/limemloh/8c0c55f67cf5a83ac7cc21cb646e65c1

1. Create signer account(s)
    2. if you creste it in web wallet you need to export it to `.export` file and then import to `concordium-client`
       `concordium-client config account import --name <name> ./_keys/<address>.export`
3. Create multisig account in desktop wallet

Two signers:
1. `3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz` (it will became multisig, accountA) (pass: 1)
2. `4Uuaaz27ahqQ7Nc6DYQUxW5bmJqFMDjorGtZkfXMfpkawHJVgy` (signer1, accountB)

check status of node

```commandline
concordium-client consensus status --grpc-ip node.testnet.concordium.com --grpc-port 20000
```

To import account to client:

```shell
concordium-client config account import --name UmbDeployerTestnet ./_keys/<address>.export
```

to get `credId`:

```shell
concordium-client account show 3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz --grpc-ip node.testnet.concordium.com --grpc-port 20000
```

```shell
concordium-client account update-keys \
--credId b69d89ac9124f74d6676e1d5d8ece8b6be613c92c13139c09ff115a9ad1b9aee141eecedee4cebf40333b48219087380 \
--sender 3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz \
./multisig/update-keys.json \
--grpc-ip node.testnet.concordium.com --grpc-port 20000
```

When update keys done, we need to update .export file as well: we basically needs to copy this part of body

```
  "signKey": "...", 
  "verifyKey": "..."
```

from
`signerB.export` into `accountA_modified.export` eg:

from `4Uuaaz27ahqQ7Nc6DYQUxW5bmJqFMDjorGtZkfXMfpkawHJVgy.export` to
`3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz_modified.export`

```shell
cd deploy-scripts

cargo run register --node https://node.testnet.concordium.com:20000 \
--account ../_keys/3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz_modified.export \
--registry "<7281,0>" --contract "<7373,0>" --contract "<7283,0>"
```

copy signature and update `deployer.rs` and run same script again using accountA/multisig

```shell
cargo run register --node https://node.testnet.concordium.com:20000 \
--account ../_keys/3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz.export \
--registry "<7281,0>" --contract "<7373,0>" --contract "<7283,0>"
```
