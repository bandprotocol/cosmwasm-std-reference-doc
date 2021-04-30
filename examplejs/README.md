# Example Price Feeder Js

#### node version

`using node v14.15.0`

#### poc contract address

An Address of the test std_reference_basic on the testnet Tequila-0004: [terra1hurg8ze4tkdy00ppuy7feuse0y2uh0mc9vuwl9](https://finder.terra.money/tequila-0004/address/terra1hurg8ze4tkdy00ppuy7feuse0y2uh0mc9vuwl9)

## Handle Message

#### relay message

```shell=
 terracli tx wasm execute terra1hurg8ze4tkdy00ppuy7feuse0y2uh0mc9vuwl9 "{\"relay\":{\"symbols\":[\"BTC\"],\"rates\":[\"56663507549999\"],\"resolve_times\":[1619813336],\"request_ids\":[4435099]}}" --gas auto --gas-prices 1.8ukrw --gas-adjustment 1.4 --chain-id tequila-0004 --node tcp://15.164.0.235:26657 --from <OWNER_ACCOUNT>
```

## Query Message

#### get reference data bulk message

```shell=
terracli query wasm contract-store terra1hurg8ze4tkdy00ppuy7feuse0y2uh0mc9vuwl9 "{\"get_reference_data_bulk\":{\"base_symbols\":[\"BTC\"], \"quote_symbols\":[\"USD\"]}}" --chain-id tequila-0004 --node tcp://15.164.0.235:26657
```
