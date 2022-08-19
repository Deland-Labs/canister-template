use crate::named_canister_ids::{CanisterNames, DEV_NAMED_CANISTER_IDS};
use candid::Principal;
use const_env::from_env;
use log::info;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub const PAGE_INPUT_MIN_LIMIT: usize = 1;
pub const PAGE_INPUT_MAX_LIMIT: usize = 100;
pub const PAGE_INPUT_MIN_OFFSET: usize = 0;
pub const PAGE_INPUT_MAX_OFFSET: usize = 10_000;

pub const ENV_DEV: &str = "dev";
pub const ENV_STAGING: &str = "staging";
pub const ENV_PRODUCTION: &str = "production";

#[from_env]
const COMMON_CANISTER_IDS_MOCK_SAMPLE_CANISTER: &str = "";
pub static CANISTER_IDS_MOCK_SAMPLE_CANISTER: Lazy<Principal> = Lazy::new(|| {
    load_dev_or_env(
        CanisterNames::MockSampleCanister,
        COMMON_CANISTER_IDS_MOCK_SAMPLE_CANISTER,
    )
});

#[from_env]
const COMMON_CANISTER_IDS_IC_LEDGER_CANISTER: &str = "";
pub static CANISTER_IDS_IC_LEDGER_CANISTER: Lazy<Principal> = Lazy::new(|| {
    load_dev_or_env(
        CanisterNames::ICLedger,
        COMMON_CANISTER_IDS_IC_LEDGER_CANISTER,
    )
});

#[from_env]
const COMMON_CANISTER_IDS_IC_MANAGEMENT_CANISTER: &str = "";
pub static CANISTER_IDS_IC_MANAGEMENT_CANISTER: Lazy<Principal> = Lazy::new(|| {
    load_dev_or_env(
        CanisterNames::ICLedger,
        COMMON_CANISTER_IDS_IC_MANAGEMENT_CANISTER,
    )
});
#[from_env]
pub const COMMON_CANISTER_ENV: &str = "dev";

pub enum CommonEnv {
    Dev,
    Staging,
    Production,
}

pub fn is_env(env: CommonEnv) -> bool {
    match env {
        CommonEnv::Dev => COMMON_CANISTER_ENV == ENV_DEV,
        CommonEnv::Staging => COMMON_CANISTER_ENV == ENV_STAGING,
        CommonEnv::Production => COMMON_CANISTER_ENV == ENV_PRODUCTION,
    }
}

pub fn is_dev_env() -> bool {
    is_env(CommonEnv::Dev)
}

fn load_dev_or_env(name: CanisterNames, env_value: &str) -> Principal {
    if is_dev_env() {
        DEV_NAMED_CANISTER_IDS.with(|ids| {
            let ids = ids.borrow();
            let id = ids.get(&name);
            if let Some(id) = id {
                info!("load_dev_or_env: from dev id list {:?} = {}", name, id);
                *id
            } else {
                info!("load_dev_or_env: from env {:?} = {}", name, env_value);
                Principal::from_str(env_value).unwrap()
            }
        })
    } else {
        Principal::from_str(env_value).unwrap()
    }
}

#[from_env]
pub const COMMON_PRINCIPAL_NAME_ADMIN: &str = "";
#[from_env]
pub const COMMON_PRINCIPAL_NAME_STATE_EXPORTER: &str = "";
#[from_env]
pub const COMMON_PRINCIPAL_NAME_TIMER_TRIGGER: &str = "";

#[cfg(test)]
mod tests;
