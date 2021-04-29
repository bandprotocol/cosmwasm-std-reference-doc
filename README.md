# Band Protocol CosmWasm Developer Documentation

In addition to data native to the [CosmWasm blockchain](https://docs.cosmwasm.com/), CosmWasm developers also have access to various cryptocurrency price data provided by [Band Protocol](https://bandprotocol.com/)'s oracle.

## Standard Reference Dataset Contract Info

### Data Available

The price data originates from [data requests](https://github.com/bandprotocol/bandchain/wiki/System-Overview#oracle-data-request) made on BandChain and then sent to Band's [std_reference_basic](https://finder.terra.money/tequila-0004/address/terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls) contract on Terra then retrieves and stores the results of those requests. Specifically, the following price pairs are available to be read from the [std_reference_proxy](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9) contract:

- AAPL/USD
- GOOGL/USD
- TSLA/USD

These prices are automatically updated every 5 minutes. The [std_reference_proxy](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9) itself is currently deployed on Terra tequila testnet at [`terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9`](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9).

The prices themselves are the median of the values retrieved by BandChain's validators from many sources including [CoinGecko](https://www.coingecko.com/api/documentations/v3), [CryptoCompare](https://min-api.cryptocompare.com/), [Binance](https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md), [CoinMarketcap](https://coinmarketcap.com/), [HuobiPro](https://www.huobi.vc/en-us/exchange/), [CoinBasePro](https://pro.coinbase.com/), [Kraken](https://www.kraken.com/), [Bitfinex](https://www.bitfinex.com/), [Bittrex](https://global.bittrex.com/), [BITSTAMP](https://www.bitstamp.net/), [OKEX](https://www.okex.com/), [FTX](https://ftx.com/), [HitBTC](https://hitbtc.com/), [ItBit](https://www.itbit.com/), [Bithumb](https://www.bithumb.com/), [CoinOne](https://coinone.co.kr/). The data request is then made by executing Band's aggregator oracle script, the code of which you can view on Band's [mainnet](https://cosmoscan.io/oracle-script/3). Along with the price data, developers will also have access to the latest timestamp the price was updated.

These parameters are intended to act as security parameters to help anyone using the data to verify that the data they are using is what they expect and, perhaps more importantly, actually valid.

### Standard Reference Dataset Contract Price Update Process

For the ease of development, the Band Foundation will be maintaining and updating the [std_reference_basic](https://finder.terra.money/tequila-0004/address/terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls) contract with the latest price data. In the near future, we will be releasing guides on how developers can create similar contracts themselves to retrieve data from Band's oracle.

## Retrieving the Price Data

The code below shows an example of a relatively [simple price database](xxx) contract on Terra which retrieve price data from Band's [std_reference_proxy](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9) contract and store it in the contract's state.

```shell=
===================
|                 |
| simple price db |
|                 |
===================
 |               ^
 |(1)            |(4)
 |Asking for     |Return
 |price data     |result
 |               |
 v               |
===================   (2) Ask     ===================
|                 |-------------->|                 |
|  std ref proxy  |               |     std ref     |
|                 |<--------------|                 |
===================   (3) Result  ===================

```

The contract is able to store exchange rate of any price pair that available on the [std_reference_basic](https://finder.terra.money/tequila-0004/address/terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls) contract. For more information on what oracle scripts are and how data requests work on BandChain in general, please see their [wiki](https://github.com/bandprotocol/bandchain/wiki/System-Overview#oracle-data-request) and [developer documentation](https://docs.bandchain.org/dapp-developers/requesting-data-from-bandchain)

```rust



```

### Code Breakdown

The example code above can be broken down into two sections: defining the interface for the `IStdReferenceProxy` and the actual `SimplePriceDB` SCORE code itself.

#### IStdReferenceProxy Interface

This section consists of two functions, `get_reference_data` and `get_reference_data_bulk`. This is the interface that we'll use to query price from Band oracle for the latest price of a token or a bunch of tokens.

#### SimplePriceDB class

The `SimplePriceDB` then contains the main logic of our SCORE. It's purpose will be to store the latest price of tokens.

The actual price data query can then be called through the `get_price` function. Before we can call the method, however, we need to first set the address of the `std_reference_proxy`. This is done by calling the `set_proxy` method or `contructor`. After that the price should be set by calling `set_single` or `set_multiple`.

The `set_single` function will simply calling `get_reference_data` from `std_reference_proxy` with base symbol and quote symbol. It then extract the exchange rate from the result and save to the state.

The `set_multiple` function converts the input into an array of base symbol and quote symbol arrays. After that it will call `get_reference_data_bulk` from `std_reference_proxy` with base symbols and quote symbols. It then extract the exchange rates from the results and save all of them to the state.

The full source code for the `SimplePriceDB` contract can be found [in this repo](xxx) along with the JSON for sending the `set_proxy`, `set_single`, `set_multiple`. The score itself is also deployed to the testnet at address [xxx](xxx).

## List of Band oracle contracts on Terra networks.

### Testnet

| Contract            |                   Address                    |
| ------------------- | :------------------------------------------: |
| std_reference       | terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls |
| std_reference_proxy | terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9 |

### Mainnet

| Contract            | Address |
| ------------------- | :-----: |
| std_reference       |   xxx   |
| std_reference_proxy |   xxx   |
