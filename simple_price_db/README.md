# Simple Price DB Example

An Address on the testnet Tequila-0004: [terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0](https://finder.terra.money/tequila-0004/address/terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0)

### Handing Messages

#### Set Oracle

```=shell
terracli tx wasm execute terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0 "{\"set_oracle_ref\":{\"new_oracle_ref\": \"terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9\" }}" --gas auto --gas-prices 1.8ukrw --gas-adjustment 1.4 --chain-id tequila-0004 --node tcp://15.164.0.235:26657 --from <OWNER_ACCOUNT>
```

#### Set Price

```=shell
terracli tx wasm execute terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0 "{\"save_price\":{\"symbol\": \"AAPL\" }}" --gas auto --gas-prices 1.8ukrw --gas-adjustment 1.4 --chain-id tequila-0004 --node tcp://15.164.0.235:26657 --from <OWNER_ACCOUNT>
```

### Query Messages

#### Query Owner

```=shell
terracli query wasm contract-store terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0 "{\"owner\":{}}" --chain-id tequila-0004 --node tcp://15.164.0.235:26657
```

#### Query Oracle

```=shell
terracli query wasm contract-store terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0 "{\"oracle_ref\":{}}" --chain-id tequila-0004 --node tcp://15.164.0.235:26657
```

#### Query Price

```=shell
terracli query wasm contract-store terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0 "{\"get_price\":{\"symbol\":\"AAPL\"}}" --chain-id tequila-0004 --node tcp://15.164.0.235:26657
```
