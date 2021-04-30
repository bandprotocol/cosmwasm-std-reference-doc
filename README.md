# Band Protocol CosmWasm Developer Documentation

In addition to data native to the [CosmWasm blockchain](https://docs.cosmwasm.com/), CosmWasm developers also have access to various cryptocurrency price data provided by [Band Protocol](https://bandprotocol.com/)'s oracle.

## Standard Reference Dataset Contract Info

### Data Available

The price data originates from [data requests](https://github.com/bandprotocol/bandchain/wiki/System-Overview#oracle-data-request) made on BandChain and then sent to Band's [std_reference_basic](https://finder.terra.money/tequila-0004/address/terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls) contract on Terra then retrieves and stores the results of those requests. Specifically, the following price pairs are available to be read from the [std_reference_proxy](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9) contract:

For example

- AAPL/USD
- GOOGL/USD
- TSLA/USD

These prices are automatically updated every 5 minutes. The [std_reference_proxy](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9) itself is currently deployed on Terra tequila testnet at [`terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9`](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9).

The prices themselves are the median of the values retrieved by BandChain's validators from many sources including [CoinGecko](https://www.coingecko.com/api/documentations/v3), [CryptoCompare](https://min-api.cryptocompare.com/), [Binance](https://github.com/binance-exchange/binance-official-api-docs/blob/master/rest-api.md), [CoinMarketcap](https://coinmarketcap.com/), [HuobiPro](https://www.huobi.vc/en-us/exchange/), [CoinBasePro](https://pro.coinbase.com/), [Kraken](https://www.kraken.com/), [Bitfinex](https://www.bitfinex.com/), [Bittrex](https://global.bittrex.com/), [BITSTAMP](https://www.bitstamp.net/), [OKEX](https://www.okex.com/), [FTX](https://ftx.com/), [HitBTC](https://hitbtc.com/), [ItBit](https://www.itbit.com/), [Bithumb](https://www.bithumb.com/), [CoinOne](https://coinone.co.kr/). The data request is then made by executing Band's aggregator oracle script, the code of which you can view on Band's [mainnet](https://cosmoscan.io/oracle-script/3). Along with the price data, developers will also have access to the latest timestamp the price was updated.

These parameters are intended to act as security parameters to help anyone using the data to verify that the data they are using is what they expect and, perhaps more importantly, actually valid.

### Standard Reference Dataset Contract Price Update Process

For the ease of development, the Band Foundation will be maintaining and updating the [std_reference_basic](https://finder.terra.money/tequila-0004/address/terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls) contract with the latest price data. In the near future, we will be releasing guides on how developers can create similar contracts themselves to retrieve data from Band's oracle.

## Retrieving the Price Data

The code below shows an example of a relatively [simple price database](https://finder.terra.money/tequila-0004/address/terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0) contract on Terra which retrieve price data from Band's [std_reference_proxy](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9) contract and store it in the contract's state. This following diagram shows the working steps of a message `SavePrice` only, which will be explain on the next section.

```shell=
       (1) Send message "SavePrice"
        |
        |
        v
===================
|                 |
| simple price db |
|                 |
===================
 |               ^
 |(2)            |(5)
 |Ask proxy      |Retrive the returned
 |contract for   |result and then save to the state
 |price          |
 v               |
===================   (3) Ask base contract  =====================
|                 |------------------------->|                   |
|  std ref proxy  |                          |   std ref basic   |
|                 |<-------------------------|                   |
===================         (4) Result       =====================

```

The contract is able to store exchange rate of any price pair that available on the [std_reference_basic](https://finder.terra.money/tequila-0004/address/terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls) contract. For more information on what oracle scripts are and how data requests work on BandChain in general, please see their [wiki](https://github.com/bandprotocol/bandchain/wiki/System-Overview#oracle-data-request) and [developer documentation](https://docs.bandchain.org/dapp-developers/requesting-data-from-bandchain)

### Code Breakdown

Now we are going to breakdown the [simple price database](https://finder.terra.money/tequila-0004/address/terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0) contract. The contract can be broken down into 4 sections which are `messages`, `state`, `struct types`, `contract logic`.

#### Messages

The full code of messages is [here](./simple_price_db/src/msg.rs).

The `InitMsg` is used to initialize state variables of the contract. Therefore, it will only be executed once when the command `terracli tx wasm instantiate ...` is used for the initialization. This message only have `initial_oracle_ref` field which is the address of the oracle contract.

```rust
pub struct InitMsg {
    pub initial_oracle_ref: HumanAddr,
}
```

The `HandleMsg` is used for state transition of the contract. This message has 2 sub messages, `SetOracleRef` and `SavePrice`. `SetOracleRef` is used when the owner want to change the reference of oracle contract. `SavePrice` is used when the owner want to consume price data from the oracle contract and then save it into the local state of this contract.

```rust
pub enum HandleMsg {
    // new_oracle_ref: a new oracle address to be set
    SetOracleRef { new_oracle_ref: HumanAddr },

    // symbol: a symbol that will be used to ask the oracle to get the price
    SavePrice { symbol: String }
}
```

The `QueryMsg` is used for contract's state reading. This message has 3 sub messages, `Owner`,`OracleRef`,`GetPrice`. The messages will be used to read the address of the owner, read the address of current oracle, read the the price of a specific symbol, respectively.

```rust
pub enum QueryMsg {
    // query owner address
    Owner {},

    // query oracle address
    OracleRef {},

    // query price that has been saved
    GetPrice { symbol: String }
}
```

The `QueryExtMsg` is used only when a call is made across a contract, which arises internally and is not caused by a direct call. This message only have 1 sub message `GetReferenceData`. `GetReferenceData` is used while trying to get a price from the oracle/[std_reference_proxy](https://finder.terra.money/tequila-0004/address/terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9).

```rust
pub enum QueryExtMsg {
    GetReferenceData {
        base_symbol: String,
        quote_symbol: String,
    }
}
```

This section consists of two functions, `get_reference_data` and `get_reference_data_bulk`. This is the interface that we'll use to query price from Band oracle for the latest price of a token or a bunch of tokens.

#### State

The full code of state is [here](./simple_price_db/src/state.rs).

The state of this contract only contain 2 local state variables and 1 mapping (`string => Uint128`) which are `owner`, `oracle_ref` and `price` (mapping).

#### Struct Types

The full code of struct types is [here](./simple_price_db/src/struct_types.rs).

This contract only have 1 custom struct which will be used only when getting price from the oracle (message `SavePrice`).

```rust
pub struct ReferenceData {
    pub rate: Uint128,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}
```

#### Contract Logic

The full code of contract logic is [here](./simple_price_db/src/contract.rs).

The contract logic can be divided into 4 parts which are `init`, `query`, `cross-contrct query` and `handle`.

The `init` part is only used at the contract initialization step. It basically set the owner of the contract and then set the reference of the oracle.

```rust
pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    owner(&mut deps.storage).save(&deps.api.canonical_address(&env.message.sender)?)?;
    oracle_ref(&mut deps.storage).save(&deps.api.canonical_address(&msg.initial_oracle_ref)?)?;
    Ok(InitResponse::default())
}
```

The `query` part is consist of 3 sub queries which are `query_owner`, `query_oracle_ref` and `query_price`. They are basically read the state and then return the result back to the querier.

```rust
fn query_owner<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CanonicalAddr> {
    owner_read(&deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("OWNER_NOT_INITIALIZED"))
}

fn query_oracle_ref<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CanonicalAddr> {
    oracle_ref_read(&deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("ORACLE_REF_NOT_INITIALIZED"))
}

fn query_price<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    symbol: String,
)  -> StdResult<Uint128> {
    match price_read(&deps.storage).get(&symbol.as_bytes()) {
        Some(data) => {
            Ok(bincode::deserialize(&data).unwrap())
        },
        _ => Err(StdError::generic_err(format!(
            "PRICE_NOT_AVAILABLE_FOR_KEY:{}",
            symbol
        ))),
    }
}
```

The `cross-contrct query` is only used when handle the message `SavePrice`. It basically send a query message `GetReferenceData` to the oracle contract and then return the struct `ReferenceData`.

```rust
// cross-contract query
fn query_reference_data<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    base_symbol: String,
    quote_symbol: String,
) -> StdResult<ReferenceData> {
    Ok(deps.querier.custom_query::<QueryMsg, ReferenceData>(
        &WasmQuery::Smart {
            contract_addr: deps.api.human_address(&query_oracle_ref(deps)?)?,
            msg: to_binary(&QueryExtMsg::GetReferenceData {
                base_symbol,
                quote_symbol,
            })?,
        }
        .into(),
    )?)
}
```

The `handle` part is consist of 2 sub handling which are `try_set_oracle_ref` and `try_set_price`. They both verify ownership before allowing sender to proceed. The `try_set_oracle_ref` will only save the `new_oracle_ref` into the state. For the `try_set_price`, it will make a `cross-contract query` to the oracle contract by using an input symbol as a parameter. After getting back the result from the oracle, the result price will be saved into the state.

```rust
pub fn try_set_oracle_ref<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    new_oracle_ref: HumanAddr,
) -> StdResult<HandleResponse> {
    let owner_addr = owner(&mut deps.storage).load()?;
    if deps.api.canonical_address(&env.message.sender)? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    oracle_ref(&mut deps.storage).save(&deps.api.canonical_address(&new_oracle_ref)?)?;

    Ok(HandleResponse::default())
}

pub fn try_set_price<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    symbol: String,
) -> StdResult<HandleResponse> {
    let owner_addr = owner(&mut deps.storage).load()?;
    if deps.api.canonical_address(&env.message.sender)? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    let reference_data = query_reference_data(deps, symbol.clone(), "USD".into())?;
    price(&mut deps.storage).set(symbol.as_bytes(), &bincode::serialize(&reference_data.rate).unwrap());

    Ok(HandleResponse::default())
}
```

## List of Band oracle contracts on Terra networks.

### Testnet Tequila-0004

| Contract            |                   Address                    |
| ------------------- | :------------------------------------------: |
| std_reference       | terra1vvnnz5g25s04m9tnv8mx9qxxhetsutjl72vpls |
| std_reference_proxy | terra16xjp4p3n4e29wgkqhjkxv2xn2q9z7jxzqgsyv9 |
| simple_price_db     | terra1p07j7w8spvgfmgpch4tcumhp97ca9jv56skjl0 |
