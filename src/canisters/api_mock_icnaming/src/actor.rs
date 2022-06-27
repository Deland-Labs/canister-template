use crate::errors::{BooleanActorResponse, NamingError, ServiceResult};
use crate::naming::FirstLevelName;
use crate::state::{
    canister_module_init, is_approved_to, is_name_owner, must_not_anonymous, set_approval,
    validate_name, State, STATE,
};
use crate::user_quota_store::{AuthPrincipal, QuotaType, TransferQuotaDetails};
use candid::{candid_method, CandidType, Deserialize, Principal};
use common::dto_name_api::TransferFromQuotaRequest;
use common::error::{ErrorInfo, ICNSActorResult, ICNSError};
use common::permissions::{is_admin, must_be_system_owner};
use ic_cdk::api;
use ic_cdk_macros::*;
use log::{debug, info};
use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct GetNamesResponse {
    pub names: Vec<(String, String)>,
}

#[init]
#[candid_method(init)]
fn init_function() {
    let owner = api::id();
    STATE.with(|state| {
        let mut map = state.borrow().registries.borrow_mut();
        map.insert(String::from("user1.org"), owner.clone());
        map.insert(String::from("user2.org"), owner.clone());
        map.insert(String::from("user3.org"), owner.clone());
        map.insert(String::from("user4.org"), owner.clone());
    });
    canister_module_init();
}

#[update(name = "set_registry_owner")]
#[candid_method(update, rename = "set_registry_owner")]
fn set_registry_owner(name: String, new_owner: Principal) -> ICNSActorResult<BooleanActorResponse> {
    STATE.with(|state| {
        let mut map = state.borrow().registries.borrow_mut();
        return if let Some(owner) = map.get_mut(&name) {
            *owner = new_owner.clone();
            Ok(BooleanActorResponse::Ok(true))
        } else {
            Ok(BooleanActorResponse::Ok(false))
        };
    })
}

#[query(name = "get_names")]
#[candid_method(query, rename = "get_names")]
fn get_names() -> ServiceResult<GetNamesResponse> {
    STATE.with(|state| {
        let mut map = state.borrow().registries.borrow_mut();
        let res = map
            .borrow()
            .iter()
            .map(|(k, v)| {
                return (k.clone(), v.to_text().clone());
            })
            .collect();
        Ok(GetNamesResponse { names: res })
    })
}

#[update(name = "add_quota")]
#[candid_method(update, rename = "add_quota")]
pub fn add_quota(
    quota_owner: Principal,
    quota_type: QuotaType,
    diff: u32,
) -> ServiceResult<BooleanActorResponse> {
    let caller = &api::caller();
    debug!("add_quota: caller: {}", caller);
    // must_be_system_owner(caller)?;

    STATE.with(|s| {
        let mut store = s.user_quota_store.borrow_mut();
        let result = store.add_quota(AuthPrincipal(quota_owner.clone()), quota_type, diff);
        Ok(BooleanActorResponse::Ok(true))
    })
}

#[query(name = "get_quota")]
#[candid_method(query, rename = "get_quota")]
pub fn get_quota(quota_type: QuotaType) -> ServiceResult<u32> {
    let caller = &api::caller();
    debug!("get_quota: caller: {}", caller);

    // match must_not_anonymous(caller) {
    //     Ok(_) => {}
    //     Err(e) => return BooleanActorResponse::new(Err(e)),
    // }
    STATE.with(|s| {
        let user_quota_manager = s.user_quota_store.borrow();
        Ok(user_quota_manager
            .get_quota(&AuthPrincipal(caller.clone()), &quota_type)
            .unwrap_or(0))
    })
}

#[derive(Debug, Deserialize, CandidType)]
pub struct BatchTransferRequest {
    pub items: Vec<TransferQuotaDetails>,
}

#[update(name = "batch_transfer_quota")]
#[candid_method(update)]
async fn batch_transfer_quota(request: BatchTransferRequest) -> BooleanActorResponse {
    let caller = api::caller();
    let result = must_not_anonymous(&caller);
    if result.is_err() {
        return BooleanActorResponse::new(Err(result.err().unwrap()));
    }
    let caller = result.unwrap();
    for item in request.items.iter() {
        match must_not_anonymous(&item.to) {
            Ok(_) => {}
            Err(e) => return BooleanActorResponse::new(Err(e)),
        }
        assert_ne!(caller.0, item.to);
        info!(
            "batch_transfer_quota: caller: {}, to: {}, quota_type: {}, diff: {}",
            caller, item.to, item.quota_type, item.diff
        );
    }
    let sum = request.items.iter().fold(0, |acc, item| acc + item.diff);
    info!("batch_transfer_quota: caller: {}, sum: {}", caller, sum);
    let result = STATE.with(|s| {
        let mut store = s.user_quota_store.borrow_mut();
        store.batch_transfer_quota(caller, request.items.as_slice())?;
        Ok(true)
    });
    BooleanActorResponse::new(result)
}

#[update(name = "transfer_quota")]
#[candid_method(update, rename = "transfer_quota")]
async fn transfer_quota(to: Principal, quota_type: QuotaType, diff: u32) -> BooleanActorResponse {
    let caller = api::caller();
    match must_not_anonymous(&caller) {
        Ok(_) => {}
        Err(e) => return BooleanActorResponse::new(Err(e)),
    }
    match must_not_anonymous(&to) {
        Ok(_) => {}
        Err(e) => return BooleanActorResponse::new(Err(e)),
    }
    assert_ne!(caller, to);
    debug!(
        "transfer_quota: caller: {}, to: {} quota_type: {} diff: {}",
        caller, to, quota_type, diff
    );
    let result = STATE.with(|s| {
        let mut store = s.user_quota_store.borrow_mut();
        store.transfer_quota(
            &AuthPrincipal(caller.clone()),
            &TransferQuotaDetails {
                to,
                quota_type,
                diff,
            },
        )
    });
    match result {
        Ok(_) => BooleanActorResponse::Ok(true),
        Err(e) => BooleanActorResponse::new(Err(e)),
    }
}

#[update(name = "transfer_from_quota")]
#[candid_method(update, rename = "transfer_from_quota")]
async fn transfer_from_quota(request: TransferFromQuotaRequest) -> BooleanActorResponse {
    let lo_quota_type = request.quota_type.into();
    debug!(
        "transfer_from_quota: from: {}, to: {} quota_type: {} diff: {}",
        request.from, request.to, lo_quota_type, request.diff
    );
    // must_not_anonymous(&request.to)?;
    // must_not_anonymous(&request.from)?;
    assert!(request.diff > 0);

    let from = AuthPrincipal(request.from.clone());
    let to = AuthPrincipal(request.to.clone());

    let result = STATE.with(|s| {
        let mut store = s.user_quota_store.borrow_mut();
        let quota_count = store.get_quota(&from, &lo_quota_type).unwrap_or(0);
        if quota_count < request.diff {
            return Err(NamingError::InsufficientQuota);
        }

        store.sub_quota(&from, &lo_quota_type, request.diff)?;
        store.add_quota(to, lo_quota_type, request.diff);
        info!(
            "transfer quota: {} from user {} to user {}, diff: {}",
            request.quota_type, &request.from, &request.to, request.diff
        );
        Ok(true)
    });
    BooleanActorResponse::new(result)
}

#[query(name = "remove_approval")]
#[candid_method(query, rename = "remove_approval")]
async fn remove_approval(name: String, owner: Principal) -> ICNSActorResult<BooleanActorResponse> {
    debug!("remove_approval: name: {}, owner: {}", name, owner);
    remove_approval(name, owner);
    Ok(BooleanActorResponse::Ok(true))
}

#[update(name = "approve")]
#[candid_method(update, rename = "approve")]
async fn approve(name: String, to: Principal) -> BooleanActorResponse {
    // if MOCK_RESULT_SUCCESS {
    //     Ok(BooleanActorResponse::Ok(true))
    // } else {
    //     Err(ErrorInfo {
    //         code: 0,
    //         message: "mock error".to_string(),
    //     })
    // }
    let caller = api::caller();
    debug!("approve: name={}, to={}", name, to);
    set_approval(&FirstLevelName::from(name), &to);
    BooleanActorResponse::Ok(true)
}

#[update(name = "transfer_from")]
#[candid_method(update, rename = "transfer_from")]
async fn transfer_from(name: String) -> BooleanActorResponse {
    let caller = &api::caller();
    let owner = &api::id();
    debug!(
        "transfer_from: caller: {:?} name: {:?} owner: {:?}",
        caller.to_text(),
        name,
        owner.to_text()
    );

    if is_approved_to(&name.clone(), &caller.clone()) == false {
        return BooleanActorResponse::new(Err(NamingError::PermissionDenied));
    }

    STATE.with(|state| {
        let mut registries = state.borrow().registries.borrow_mut();
        //get registries by name and replace value with caller, if not exist then return error
        if let Some(registry) = registries.get_mut(&name) {
            *registry = caller.clone();
        } else {
            return BooleanActorResponse::new(Err(NamingError::RegistrationNotFound));
        }
        BooleanActorResponse::Ok(true)
    })
}

#[update(name = "transfer")]
#[candid_method(update, rename = "transfer")]
async fn transfer(name: String, new_owner: Principal) -> BooleanActorResponse {
    let caller = &api::caller();

    debug!(
        "transfer: caller: {:?} name: {:?} new_owner: {:?}",
        caller.to_text(),
        name,
        new_owner.to_text()
    );
    let check_owner = is_name_owner(&FirstLevelName::from(name.clone()), caller);
    match check_owner {
        Ok(_) => {
            STATE.with(|state| {
                let mut registries = state.borrow().registries.borrow_mut();
                //get registries by name and replace value with caller, if not exist then return error
                if let Some(registry) = registries.get_mut(&name) {
                    *registry = new_owner.clone();
                } else {
                    return BooleanActorResponse::new(Err(NamingError::RegistrationNotFound));
                }
                BooleanActorResponse::Ok(true)
            })
        }
        Err(e) => BooleanActorResponse::new(Err(e)),
    }
}

candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}
