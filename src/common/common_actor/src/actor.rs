use std::collections::HashMap;

use candid::candid_method;
use ic_cdk::api;
use ic_cdk_macros::*;
use log::{debug, error, info};

use common::constants::is_dev_env;
use common::dto::{
    from_state_export_data, to_state_export_data, GetStatsResponse, LoadStateRequest,
    StateExportResponse,
};
use common::errors::{BooleanActorResponse, CommonError, ErrorInfo};
use common::named_principals::PRINCIPAL_NAME_STATE_EXPORTER;
use common::permissions::{must_be_named_principal, must_be_system_owner};
use common::state::StableState;

use crate::state::{State, STATE};
use crate::stats_service::{Stats, StatsService};

#[query(name = "get_stats")]
#[candid_method(query, rename = "get_stats")]
pub fn get_stats() -> GetStatsResponse<Stats> {
    let now = api::time();
    let service = StatsService::default();
    let stats = service.get_stats(now);
    GetStatsResponse::new(Ok(stats))
}

#[update(name = "export_state")]
#[candid_method(update, rename = "export_state")]
pub async fn export_state() -> StateExportResponse {
    let caller = &api::caller();
    let permission_result = must_be_named_principal(caller, PRINCIPAL_NAME_STATE_EXPORTER);
    if permission_result.is_err() {
        return StateExportResponse::new(Err(permission_result.err().unwrap()));
    }

    let source_data = STATE.with(|state| to_state_export_data(state.encode()));
    StateExportResponse::new(Ok(source_data))
}

#[update(name = "load_state")]
#[candid_method(update, rename = "load_state")]
pub fn load_state(request: LoadStateRequest) -> BooleanActorResponse {
    if !is_dev_env() {
        return BooleanActorResponse::new(Err(CommonError::Unknown {
            detail: "!is_dev_env()".to_string(),
        }));
    }
    debug!("load_state: {}", request);
    let caller = &api::caller();
    if must_be_system_owner(caller).is_err() {
        error!("load_state: caller is not system owner");
        return BooleanActorResponse::new(Err(CommonError::PermissionDenied));
    }
    STATE.with(|s| {
        let bytes = from_state_export_data(request);
        let result = State::decode(bytes);
        if result.is_err() {
            let err_msg = format!("Failed to decode state: {:?}", result.err());
            error!("{}", err_msg.to_string());
            return BooleanActorResponse::Err(ErrorInfo::from(CommonError::Unknown {
                detail: err_msg,
            }));
        }
        let new_state = result.unwrap();
        s.replace(new_state);
        info!("load_state: success");
        return BooleanActorResponse::Ok(true);
    })
}

#[query(name = "get_wasm_info")]
#[candid_method(query)]
fn get_wasm_info() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("BUILD_TIMESTAMP", env!("BUILD_TIMESTAMP"));
    map.insert("CARGO_PKG_VERSION", env!("CARGO_PKG_VERSION"));
    map.insert("GIT_BRANCH", env!("GIT_BRANCH"));
    map.insert("SOURCE_TIMESTAMP", env!("SOURCE_TIMESTAMP"));
    map.insert("GIT_DIRTY", env!("GIT_DIRTY"));
    map.insert("GIT_COMMIT", env!("GIT_COMMIT"));
    map.insert("RUSTC_VERSION", env!("RUSTC_VERSION"));
    map
}
