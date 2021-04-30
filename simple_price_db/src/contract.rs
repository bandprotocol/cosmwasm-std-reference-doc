use crate::msg::{HandleMsg, InitMsg, QueryMsg, QueryExtMsg};
use crate::state::{owner, owner_read, oracle_ref, oracle_ref_read, price, price_read};
use crate::struct_types::ReferenceData;
use cosmwasm_std::{CanonicalAddr, HumanAddr};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage, WasmQuery, Uint128, ReadonlyStorage
};

macro_rules! unwrap_query {
    ( $e:expr, $f:expr ) => {
        match $e {
            Ok(x) => match to_binary(&x) {
                Ok(y) => Ok(y),
                Err(_) => Err(StdError::generic_err($f)),
            },
            Err(e) => return Err(e),
        }
    };
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    owner(&mut deps.storage).save(&deps.api.canonical_address(&env.message.sender)?)?;
    oracle_ref(&mut deps.storage).save(&deps.api.canonical_address(&msg.initial_oracle_ref)?)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::SetOracleRef { new_oracle_ref } => try_set_oracle_ref(deps, env, new_oracle_ref),
        HandleMsg::SavePrice { symbol } => try_set_price(deps, env, symbol),
    }
}

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

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => unwrap_query!(query_owner(deps), "SERIALIZE_OWNER_ERROR"),
        QueryMsg::OracleRef {} => unwrap_query!(query_oracle_ref(deps), "SERIALIZE_ORACLE_REF_ERROR"),
        QueryMsg::GetPrice {
            symbol
        } => unwrap_query!(
            query_price(deps, symbol),
            "SERIALIZE_REFERENCE_DATA_ERROR"
        ),
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier};
    use cosmwasm_std::{coins, from_binary, Coin, HumanAddr, MemoryStorage, StdError};

    fn init_msg(r: &str) -> InitMsg {
        InitMsg {
            initial_oracle_ref: HumanAddr::from(r),
        }
    }

    fn handle_set_oracle_ref(r: &str) -> HandleMsg {
        HandleMsg::SetOracleRef {
            new_oracle_ref: HumanAddr::from(r),
        }
    }

    fn query_owner_msg() -> QueryMsg {
        QueryMsg::Owner {}
    }

    fn query_ref_msg() -> QueryMsg {
        QueryMsg::OracleRef {}
    }

    fn get_mocks(
        sender: &str,
        sent: &[Coin],
        height: u64,
        time: u64,
    ) -> (Extern<MemoryStorage, MockApi, MockQuerier>, Env) {
        let deps = mock_dependencies(20, &[]);

        let mut env = mock_env(sender, sent);
        env.block.height = height;
        env.block.time = time;

        (deps, env)
    }

    #[test]
    fn proper_initialization() {
        let msg = init_msg("test_oracle_ref");
        let (mut deps, env) = get_mocks("owner", &coins(1000, "test_coin"), 789, 0);

        // owner not initialized yet
        match query(&deps, query_owner_msg()).unwrap_err() {
            StdError::GenericErr { msg, .. } => assert_eq!("OWNER_NOT_INITIALIZED", msg),
            _ => panic!("Test Fail: expect OWNER_NOT_INITIALIZED"),
        }

        // Check if successfully set owner
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Verify correct owner address
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("owner")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_owner_msg()).unwrap()).unwrap()
        );

        // Verify correct ref address
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("test_oracle_ref")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_ref_msg()).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_set_oracle_ref_fail_unauthorized() {
        let (mut deps, env) = get_mocks("owner", &coins(1000, "test_coin"), 789, 0);

        // should successfully init owner
        assert_eq!(
            0,
            init(&mut deps, env.clone(), init_msg("test_oracle_ref"))
                .unwrap()
                .messages
                .len()
        );

        // check owner in the state
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("owner")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_owner_msg()).unwrap()).unwrap()
        );
        // check ref in the state
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("test_oracle_ref")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_ref_msg()).unwrap()).unwrap()
        );

        let (_, alice_env) = get_mocks("alice", &coins(1000, "test_coin"), 789, 0);

        // should fail because sender is alice not owner
        match handle(&mut deps, alice_env.clone(), handle_set_oracle_ref("test_oracle_ref")).unwrap_err() {
            StdError::GenericErr { msg, .. } => assert_eq!("NOT_AUTHORIZED", msg),
            _ => panic!("Test Fail: expect NOT_AUTHORIZED"),
        }

        // check ref in the state
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("test_oracle_ref")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_ref_msg()).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_set_ref_success() {
        let (mut deps, env) = get_mocks("owner", &coins(1000, "test_coin"), 789, 0);

        // should successfully init owner
        assert_eq!(
            0,
            init(&mut deps, env.clone(), init_msg("test_oracle_ref_1"))
                .unwrap()
                .messages
                .len()
        );

        // check owner in the state
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("owner")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_owner_msg()).unwrap()).unwrap()
        );

        // check ref in the state
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("test_oracle_ref_1")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_ref_msg()).unwrap()).unwrap()
        );

        // should successfully set new owner
        assert_eq!(
            0,
            handle(&mut deps, env.clone(), handle_set_oracle_ref("test_oracle_ref_2"))
                .unwrap()
                .messages
                .len()
        );

        // check owner in the state should be new_owner
        assert_eq!(
            deps.api.canonical_address(&HumanAddr::from("test_oracle_ref_2")).unwrap(),
            from_binary::<CanonicalAddr>(&query(&deps, query_ref_msg()).unwrap()).unwrap()
        );
    }
}
