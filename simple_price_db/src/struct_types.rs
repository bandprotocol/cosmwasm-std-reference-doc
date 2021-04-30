use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, JsonSchema)]
pub struct ReferenceData {
    pub rate: Uint128,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}

impl ReferenceData {
    pub fn new(rate: Uint128, last_updated_base: u64, last_updated_quote: u64) -> Self {
        ReferenceData {
            rate: rate,
            last_updated_base: last_updated_base,
            last_updated_quote: last_updated_quote,
        }
    }
}

