# Example Price Feeder Js

#### node version

`using node v14.15.0`

#### export mnemonic

```shell=
export COSMWASM_DOC_SEED=<YOUR_MNEMONIC>
```

#### install

```shell=
yarn install
```

#### run

```shell=
node index.js
```

#### poc contract address

An Address of the poc std_reference_basic on the testnet Bombay-12: [terra1kzwzdknntsl957vgd8d8ns75hk6h0cm2cg3c79](https://finder.terra.money/bombay-12/address/terra1kzwzdknntsl957vgd8d8ns75hk6h0cm2cg3c79)

## Handle Message

#### relay message

```shell=
 terrad tx wasm execute terra1kzwzdknntsl957vgd8d8ns75hk6h0cm2cg3c79 "{\"relay\":{\"symbols\":[\"BTC\"],\"rates\":[\"56663507549999\"],\"resolve_times\":[1619813336],\"request_ids\":[4435099]}}" --gas auto --fees <FEES_AMOUNT> --chain-id bombay-12 --node tcp://3.34.163.215:26657 --from <OWNER_ACCOUNT>
```

## Query Message

#### get reference data bulk message

```shell=
terrad query wasm contract-store terra1kzwzdknntsl957vgd8d8ns75hk6h0cm2cg3c79 "{\"get_reference_data_bulk\":{\"base_symbols\":[\"BTC\"], \"quote_symbols\":[\"USD\"]}}" --chain-id bombay-12 --node tcp://3.34.163.215:26657
```
