## Multisig

https://gist.github.com/limemloh/8c0c55f67cf5a83ac7cc21cb646e65c1

1. Create signer account(s)
   2. if you creste it in web wallet you need to export it to `.export` file and then import to `concordium-client`
   `concordium-client config account import --name <name> ./_keys/<address>.export`
3. Create multisig account in desktop wallet


Two signers:
1. `4Uuaaz27ahqQ7Nc6DYQUxW5bmJqFMDjorGtZkfXMfpkawHJVgy` (deployer)
2. `3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz` (it will became multisig) (pass: 1)

check status of node
```commandline
concordium-client consensus status --grpc-ip node.testnet.concordium.com --grpc-port 20000
```

To import account to client:
```shell
concordium-client config account import --name UmbDeployerTestnet ./_keys/4Uuaaz27ahqQ7Nc6DYQUxW5bmJqFMDjorGtZkfXMfpkawHJVgy.export
concordium-client config account import --name UmbMultisigTestnet_1 ./_keys/3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz.export
```

to get `credId`:

```shell
concordium-client account show 3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz --grpc-ip node.testnet.concordium.com --grpc-port 20000
```

```shell
concordium-client account update-keys \
--credId b69d89ac9124f74d6676e1d5d8ece8b6be613c92c13139c09ff115a9ad1b9aee141eecedee4cebf40333b48219087380 \
--sender UmbMultisigTestnet_1 \
./multisig/update-keys.json \
--grpc-ip node.testnet.concordium.com --grpc-port 20000
```

When update keys done, we need to update .export file as well: we basically needs to copy body of
`signer1.export` into `multisigAccount.export` eg:

`3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz_modified.export` - it should have body of `4Uuaaz27ahqQ7Nc6DYQUxW5bmJqFMDjorGtZkfXMfpkawHJVgy.export`

```shell
cd deploy-scripts

cargo run register --node http://node.testnet.concordium.com:20000 \
--account ./3Pz2yuzVqxiAPD29PDAJi1bECDojpxyyyzMQm5ToSnmkMENDyz_modified.export \
--registry "<7281,0>" --contract "<7373,0>" --contract "<7283,0>"
```

### Adjust code

1. when using multisig, we need to have hardcoded timestamp! must be the same for all signers
