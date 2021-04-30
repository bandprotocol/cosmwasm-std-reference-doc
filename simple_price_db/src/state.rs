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
pub fn owner<S: Storage>(storage: &mut S) -> Singleton<S, CanonicalAddr> {
    singleton(storage, OWNER_KEY)
}

pub fn owner_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, CanonicalAddr> {
    singleton_read(storage, OWNER_KEY)
}

// oracle
pub fn oracle_ref<S: Storage>(storage: &mut S) -> Singleton<S, CanonicalAddr> {
    singleton(storage, ORACLE_REF_KEY)
}

pub fn oracle_ref_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, CanonicalAddr> {
    singleton_read(storage, ORACLE_REF_KEY)
}

// price
pub fn price<S: Storage>(storage: &mut S) -> PrefixedStorage<S> {
    PrefixedStorage::new(PRICE_KEY, storage)
}

pub fn price_read<S: Storage>(storage: &S) -> ReadonlyPrefixedStorage<S> {
    ReadonlyPrefixedStorage::new(PRICE_KEY, storage)
}
