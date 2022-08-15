use crate::errors::{ActorResult, CommonError, ErrorInfo};
use crate::ic_ledger_types::{Subaccount, TransferArgs, TransferResult};
use crate::named_canister_ids::{get_named_get_canister_id, CanisterNames};
use async_trait::async_trait;
use candid::{CandidType, Nat, Principal};
use ic_cdk::api::call::RejectionCode;
use ic_cdk::call;
use log::{debug, error};
use serde::Deserialize;
use std::fmt::Debug;

pub mod ic_impl;

async fn call_core<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
    logging: bool,
) -> Result<TResult, CommonError>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    if logging {
        debug!("Calling {:?}::{}", canister_name, method);
    }
    let canister_id = get_named_get_canister_id(canister_name);
    let call_result: Result<(TResult,), (RejectionCode, String)> =
        call(canister_id, method, args).await;
    if call_result.is_err() {
        let (code, message) = call_result.err().unwrap();
        let code_string = format!("{:?}", code);
        error!(
            "{:?}::{} failed with code {}: {}",
            canister_name, method, code_string, message
        );
        return Err(CommonError::CanisterCallError {
            message,
            rejection_code: code_string,
        });
    }
    let result = call_result.unwrap();
    if logging {
        debug!(
            "Call canister {:?} with method {} result: {:?}",
            canister_name, method, result
        );
    }
    Ok(result.0)
}

async fn call_canister_as_actor_result<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
) -> ActorResult<TResult>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    let result = call_core::<T, ActorResult<TResult>>(canister_name, method, args, true).await;
    match result {
        Ok(result) => result,
        Err(error) => Err(ErrorInfo::from(error)),
    }
}

async fn call_canister_as_result<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
) -> ActorResult<TResult>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    call_core::<T, TResult>(canister_name, method, args, true)
        .await
        .map_err(ErrorInfo::from)
}

async fn call_canister_as_result_no_logging<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
) -> ActorResult<TResult>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    call_core::<T, TResult>(canister_name, method, args, false)
        .await
        .map_err(ErrorInfo::from)
}

pub type TransactionId = String;

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct TransactionResponse {
    #[serde(rename = "txId")]
    tx_id: TransactionId,
    #[serde(rename = "blockHeight")]
    block_height: Nat,
}

#[async_trait]
pub trait IDFTApi {
    async fn transfer_from(
        &self,
        spender_sub_account: Option<Subaccount>,
        from: String,
        to: String,
        value: Nat,
        created_at: Option<u64>,
    ) -> ActorResult<TransactionResponse>;

    async fn transfer(
        &self,
        from_sub_account: Option<Subaccount>,
        to: String,
        value: Nat,
        created_at: Option<u64>,
    ) -> ActorResult<TransactionResponse>;

    async fn balance_of(&self, token_holder: String) -> ActorResult<Nat>;
}

#[async_trait]
pub trait IICLedgerApi {
    async fn transfer(&self, args: TransferArgs) -> ActorResult<TransferResult>;
}

#[derive(CandidType, Clone, Deserialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
pub struct CanisterIdRecord {
    pub canister_id: Principal,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
}

#[allow(non_camel_case_types)]
#[derive(CandidType, Debug, Deserialize)]
pub enum CanisterStatus {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "stopped")]
    Stopped,
}

#[derive(CandidType, Debug, Deserialize)]
pub struct CanisterStatusResponse {
    pub status: CanisterStatus,
    pub settings: CanisterSettings,
    pub module_hash: Option<Vec<u8>>,
    pub controller: Principal,
    pub memory_size: Nat,
    pub cycles: Nat,
}

// Install Wasm
#[derive(CandidType, Deserialize)]
enum InstallMode {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
}

#[derive(CandidType, Deserialize)]
struct CanisterInstall {
    mode: InstallMode,
    canister_id: Principal,
    #[serde(with = "serde_bytes")]
    wasm_module: Vec<u8>,
    #[serde(with = "serde_bytes")]
    arg: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CreateCanisterArgs {
    pub cycles: u64,
    pub settings: CanisterSettings,
}

#[async_trait]
pub trait IICManagementAPI {
    async fn create_canister(&self, args: CreateCanisterArgs) -> Result<CanisterIdRecord, String>;
    async fn canister_status(
        &self,
        id_record: CanisterIdRecord,
    ) -> Result<CanisterStatusResponse, String>;
    async fn canister_install(
        &self,
        canister_id: &Principal,
        wasm_module: Vec<u8>,
        args: Vec<u8>,
    ) -> Result<(), String>;
}
