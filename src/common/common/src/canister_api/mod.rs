use std::fmt::Debug;

use async_trait::async_trait;
use candid::{CandidType, Nat};
use ic_cdk::api::call::call_with_payment;
use ic_cdk::call;
use log::{debug, error};
use serde::Deserialize;

pub use ic_api::*;

use crate::errors::{ActorResult, CommonError, ErrorInfo};
use crate::named_canister_ids::{get_named_canister_id, CanisterNames};
use crate::types::ic_ledger_types::{Subaccount, TransferArgs, TransferResult};
use crate::types::ic_management_types::*;

pub mod ic_api;
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
    let canister_id = get_named_canister_id(canister_name);
    let (call_res,): (TResult,) =
        call(canister_id.0, method, args)
            .await
            .map_err(|(code, message)| {
                let code_string = format!("{:?}", code);
                error!(
                    "{:?}::{} failed with code {}: {}",
                    canister_name, method, code_string, message
                );
                CommonError::CanisterCallError {
                    message,
                    rejection_code: code_string,
                }
            })?;

    if logging {
        debug!(
            "Call canister {:?} with method {} result: {:?}",
            canister_name, method, call_res
        );
    }
    Ok(call_res)
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

async fn call_core_with_payment<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
    cycles: u64,
    logging: bool,
) -> Result<TResult, CommonError>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    if logging {
        debug!(
            "Calling {:?}::{} with payment {} cycles",
            canister_name, method, cycles
        );
    }
    let canister_id = get_named_canister_id(canister_name);
    let (call_res,): (TResult,) = call_with_payment(canister_id.0, method, args, cycles)
        .await
        .map_err(|(code, message)| {
            let code_string = format!("{:?}", code);
            error!(
                "{:?}::{} failed with code {}: {}",
                canister_name, method, code_string, message
            );
            CommonError::CanisterCallError {
                message,
                rejection_code: code_string,
            }
        })?;

    if logging {
        debug!(
            "Call canister {:?} with method {} with payment {} result: {:?}",
            canister_name, method, cycles, call_res
        );
    }
    Ok(call_res)
}

async fn call_canister_with_payment_as_actor_result<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
    cycles: u64,
) -> ActorResult<TResult>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    let result = call_core_with_payment::<T, ActorResult<TResult>>(
        canister_name,
        method,
        args,
        cycles,
        true,
    )
    .await;
    match result {
        Ok(result) => result,
        Err(error) => Err(ErrorInfo::from(error)),
    }
}

async fn call_canister_with_payment_as_result<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
    cycles: u64,
) -> ActorResult<TResult>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    call_core_with_payment::<T, TResult>(canister_name, method, args, cycles, true)
        .await
        .map_err(ErrorInfo::from)
}

async fn call_canister_with_payment_as_result_no_logging<T, TResult>(
    canister_name: CanisterNames,
    method: &str,
    args: T,
    cycles: u64,
) -> ActorResult<TResult>
where
    T: candid::utils::ArgumentEncoder,
    TResult: for<'a> Deserialize<'a> + CandidType + Debug,
{
    call_core_with_payment::<T, TResult>(canister_name, method, args, cycles, false)
        .await
        .map_err(ErrorInfo::from)
}
