use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{
    singleton,
    singleton_read,
    PrefixedStorage,
    ReadonlyPrefixedStorage,
    ReadonlySingleton,
    Singleton
};

pub static OWNER_KEY: &[u8] = b"owner";
pub static ORACLE_REF_KEY: &[u8] = b"oracle_ref";
pub static PRICE_KEY: &[u8] = b"price";

// owner
pub fn owner(storage: &mut dyn Storage) -> Singleton<CanonicalAddr> {
    singleton(storage, OWNER_KEY)
}

pub fn owner_read(storage: &dyn Storage) -> ReadonlySingleton<CanonicalAddr> {
    singleton_read(storage, OWNER_KEY)
}

// oracle
pub fn oracle_ref(storage: &mut dyn Storage) -> Singleton<CanonicalAddr> {
    singleton(storage, ORACLE_REF_KEY)
}

pub fn oracle_ref_read(storage: &dyn Storage) -> ReadonlySingleton<CanonicalAddr> {
    singleton_read(storage, ORACLE_REF_KEY)
}

// price
pub fn price(storage: &mut dyn Storage) -> PrefixedStorage {
    PrefixedStorage::new(storage, PRICE_KEY)
}

pub fn price_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, PRICE_KEY)
}
