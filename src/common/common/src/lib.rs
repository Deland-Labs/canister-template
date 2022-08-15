use crate::errors::{CommonError, ServiceResult};
use crate::named_canister_ids::{is_named_canister_id, CanisterNames};
use crate::named_principals::is_named_principal;
use crate::permissions::is_admin;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub mod constants;
pub mod dto;
pub mod errors;
pub mod http;
pub mod ic_logger;
pub mod metrics_encoder;
pub mod named_canister_ids;
pub mod named_principals;
pub mod permissions;
pub mod state;
pub mod timeout_lock;
pub mod types;

pub mod canister_api;
#[cfg(test)]
mod test_common;
