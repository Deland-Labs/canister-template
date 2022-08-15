use std::{
    fmt::{Display, Formatter},
    ops::{Add, Sub},
};

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api;

use crate::{
    errors::{CommonError, ServiceResult},
    named_canister_ids::{is_named_canister_id, CanisterNames},
    named_principals::is_named_principal,
    permissions::is_admin,
};

pub mod cycles_minting_types;
pub mod ic_ledger_types;
pub mod ic_management_types;

#[derive(Eq, Ord, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, CandidType, Deserialize)]
#[serde(transparent)]
pub struct CanisterId(pub Principal);

impl Display for CanisterId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_text())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, CandidType, Deserialize, Copy, Ord, PartialOrd)]
#[serde(transparent)]
pub struct TimeInNs(pub u64);

impl Add for TimeInNs {
    type Output = TimeInNs;

    fn add(self, rhs: Self) -> Self::Output {
        TimeInNs(self.0 + rhs.0)
    }
}

impl Sub for TimeInNs {
    type Output = TimeInNs;

    fn sub(self, rhs: Self) -> Self::Output {
        TimeInNs(self.0 - rhs.0)
    }
}

impl Display for TimeInNs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ns", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, CandidType, Deserialize, Copy, Ord, PartialOrd)]
#[serde(transparent)]
pub struct TimeInSec(pub u64);

impl From<TimeInNs> for TimeInSec {
    fn from(ns: TimeInNs) -> Self {
        TimeInSec(ns.0 / 1_000_000_000)
    }
}

impl Display for TimeInSec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} s", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, CandidType, Deserialize, Copy)]
#[serde(transparent)]
pub struct AuthPrincipal(pub Principal);

impl Display for AuthPrincipal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct CallContext {
    pub caller: Principal,
    pub now: TimeInNs,
}

impl CallContext {
    pub fn new(caller: Principal, now: TimeInNs) -> Self {
        Self { caller, now }
    }

    pub fn anonymous() -> Self {
        Self {
            caller: Principal::anonymous(),
            now: TimeInNs(1651571294_000_000_000),
        }
    }

    pub fn from_ic() -> Self {
        Self {
            caller: api::caller(),
            now: TimeInNs(api::time()),
        }
    }

    pub fn must_not_anonymous(&self) -> ServiceResult<AuthPrincipal> {
        if self.caller == Principal::anonymous() {
            return Err(CommonError::Unauthorized);
        }
        Ok(AuthPrincipal(self.caller))
    }

    pub fn must_be_system_owner(&self) -> ServiceResult<AuthPrincipal> {
        if !is_admin(&self.caller) {
            return Err(CommonError::Unauthorized);
        }
        Ok(AuthPrincipal(self.caller))
    }

    pub fn must_be_named_principal(&self, name: &str) -> ServiceResult<AuthPrincipal> {
        if !is_named_principal(name, &self.caller) {
            return Err(CommonError::Unauthorized);
        }
        Ok(AuthPrincipal(self.caller))
    }

    pub fn must_be_in_named_principal(&self, names: &[&str]) -> ServiceResult<AuthPrincipal> {
        for name in names {
            if is_named_principal(name, &self.caller) {
                return Ok(AuthPrincipal(self.caller));
            }
        }
        return Err(CommonError::Unauthorized);
    }

    pub fn must_be_named_canister(&self, name: CanisterNames) -> ServiceResult<AuthPrincipal> {
        if !is_named_canister_id(name, CanisterId(self.caller)) {
            return Err(CommonError::Unauthorized);
        }
        Ok(AuthPrincipal(self.caller))
    }

    pub fn must_be_in_named_canister(
        &self,
        names: &[CanisterNames],
    ) -> ServiceResult<AuthPrincipal> {
        for name in names {
            if is_named_canister_id(*name, CanisterId(self.caller)) {
                return Ok(AuthPrincipal(self.caller));
            }
        }
        return Err(CommonError::Unauthorized);
    }
}
