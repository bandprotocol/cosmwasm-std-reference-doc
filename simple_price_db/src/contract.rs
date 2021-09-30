use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryExtMsg};
use crate::state::{owner, owner_read, oracle_ref, oracle_ref_read, price, price_read};
use crate::struct_types::ReferenceData;
use cosmwasm_std::{CanonicalAddr, Addr, DepsMut, MessageInfo, Deps, Response};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, StdError, StdResult, Storage, WasmQuery, Uint128,
    entry_point
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    owner(deps.storage).save(&deps.api.addr_canonicalize(&info.sender.as_str())?)?;
    oracle_ref(deps.storage).save(&deps.api.addr_canonicalize(&msg.initial_oracle_ref.as_str())?)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetOracleRef { new_oracle_ref } => try_set_oracle_ref(deps, info, new_oracle_ref),
        ExecuteMsg::SavePrice { symbol } => try_set_price(deps, info, symbol),
    }
}

pub fn try_set_oracle_ref(
    deps: DepsMut,
    info: MessageInfo,
    new_oracle_ref: Addr,
) -> StdResult<Response> {
    let owner_addr = owner(deps.storage).load()?;
    if deps.api.addr_canonicalize(&info.sender.as_str())? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    oracle_ref(deps.storage).save(&deps.api.addr_canonicalize(&new_oracle_ref.as_str())?)?;

    Ok(Response::default())
}

pub fn try_set_price(
    deps: DepsMut,
    info: MessageInfo,
    symbol: String,
) -> StdResult<Response> {
    let owner_addr = owner(deps.storage).load()?;
    if deps.api.addr_canonicalize(&info.sender.as_str())? != owner_addr {
        return Err(StdError::generic_err("NOT_AUTHORIZED"));
    }

    let reference_data = query_reference_data(deps.as_ref(), symbol.clone(), "USD".into())?;
    price(deps.storage).set(symbol.as_bytes(), &bincode::serialize(&reference_data.rate).unwrap());

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
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

fn query_owner(deps: Deps) -> StdResult<CanonicalAddr> {
    owner_read(deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("OWNER_NOT_INITIALIZED"))
}

fn query_oracle_ref(deps: Deps) -> StdResult<CanonicalAddr> {
    oracle_ref_read(deps.storage)
        .load()
        .map_err(|_| StdError::generic_err("ORACLE_REF_NOT_INITIALIZED"))
}

fn query_price(
    deps: Deps,
    symbol: String,
)  -> StdResult<Uint128> {
    match price_read(deps.storage).get(&symbol.as_bytes()) {
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
fn query_reference_data(
    deps: Deps,
    base_symbol: String,
    quote_symbol: String,
) -> StdResult<ReferenceData> {
    Ok(deps.querier.custom_query::<QueryMsg, ReferenceData>(
        &WasmQuery::Smart {
            contract_addr: deps.api.addr_humanize(&query_oracle_ref(deps)?)?.into_string(),
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
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{coins, from_binary, Coin, Addr, StdError, OwnedDeps, Timestamp};

    fn init_msg(r: &str) -> InstantiateMsg {
        InstantiateMsg {
            initial_oracle_ref: Addr::unchecked(r),
        }
    }

    fn handle_set_oracle_ref(r: &str) -> ExecuteMsg {
        ExecuteMsg::SetOracleRef {
            new_oracle_ref: Addr::unchecked(r),
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
    ) -> (OwnedDeps<MockStorage, MockApi, MockQuerier>, Env, MessageInfo) {
        let deps = mock_dependencies(&[]);

        let mut env = mock_env();
        env.block.height = height;
        env.block.time = Timestamp::from_seconds(time);

        let info = mock_info(sender, sent);

        (deps, env, info)
    }

    #[test]
    fn proper_initialization() {
        let msg = init_msg("test_oracle_ref");
        let (mut deps, env, info) = get_mocks("owner", &coins(1000, "test_coin"), 789, 0);

        // owner not initialized yet
        match query(deps.as_ref(), env.clone(), query_owner_msg()).unwrap_err() {
            StdError::GenericErr { msg, .. } => assert_eq!("OWNER_NOT_INITIALIZED", msg),
            _ => panic!("Test Fail: expect OWNER_NOT_INITIALIZED"),
        }

        // Check if successfully set owner
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Verify correct owner address
        assert_eq!(
            deps.api.addr_canonicalize("owner").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_owner_msg()).unwrap()).unwrap()
        );

        // Verify correct ref address
        assert_eq!(
            deps.api.addr_canonicalize("test_oracle_ref").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_ref_msg()).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_set_oracle_ref_fail_unauthorized() {
        let (mut deps, env, info) = get_mocks("owner", &coins(1000, "test_coin"), 789, 0);

        // should successfully instantiate owner
        assert_eq!(
            0,
            instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg("test_oracle_ref"))
                .unwrap()
                .messages
                .len()
        );

        // check owner in the state
        assert_eq!(
            deps.api.addr_canonicalize("owner").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_owner_msg()).unwrap()).unwrap()
        );
        // check ref in the state
        assert_eq!(
            deps.api.addr_canonicalize("test_oracle_ref").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_ref_msg()).unwrap()).unwrap()
        );

        let (_, alice_env, alice_info) = get_mocks("alice", &coins(1000, "test_coin"), 789, 0);

        // should fail because sender is alice not owner
        match execute(deps.as_mut(), alice_env.clone(), alice_info.clone(), handle_set_oracle_ref("test_oracle_ref")).unwrap_err() {
            StdError::GenericErr { msg, .. } => assert_eq!("NOT_AUTHORIZED", msg),
            _ => panic!("Test Fail: expect NOT_AUTHORIZED"),
        }

        // check ref in the state
        assert_eq!(
            deps.api.addr_canonicalize("test_oracle_ref").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_ref_msg()).unwrap()).unwrap()
        );
    }

    #[test]
    fn test_set_ref_success() {
        let (mut deps, env, info) = get_mocks("owner", &coins(1000, "test_coin"), 789, 0);

        // should successfully instantiate owner
        assert_eq!(
            0,
            instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg("test_oracle_ref_1"))
                .unwrap()
                .messages
                .len()
        );

        // check owner in the state
        assert_eq!(
            deps.api.addr_canonicalize("owner").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_owner_msg()).unwrap()).unwrap()
        );

        // check ref in the state
        assert_eq!(
            deps.api.addr_canonicalize("test_oracle_ref_1").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_ref_msg()).unwrap()).unwrap()
        );

        // should successfully set new owner
        assert_eq!(
            0,
            execute(deps.as_mut(), env.clone(), info.clone(), handle_set_oracle_ref("test_oracle_ref_2"))
                .unwrap()
                .messages
                .len()
        );

        // check owner in the state should be new_owner
        assert_eq!(
            deps.api.addr_canonicalize("test_oracle_ref_2").unwrap(),
            from_binary::<CanonicalAddr>(&query(deps.as_ref(), env.clone(), query_ref_msg()).unwrap()).unwrap()
        );
    }
}
