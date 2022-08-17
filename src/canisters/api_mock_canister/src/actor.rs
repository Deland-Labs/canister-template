use crate::errors::{BooleanActorResponse, MockError, ServiceResult};
use crate::mock_utils::FirstLevelName;
use crate::state::{
    canister_module_init, is_approved_to, is_name_owner, must_not_anonymous, set_approval,
    validate_name, State, STATE,
};
use candid::{candid_method, CandidType, Deserialize, Principal};
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

#[query(name = "get_names")]
#[candid_method(query, rename = "get_names")]
fn get_names() -> ServiceResult<GetNamesResponse> {
    STATE.with(|state| {
        let map = state.borrow().registries.borrow_mut();
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
        return BooleanActorResponse::new(Err(MockError::PermissionDenied));
    }

    STATE.with(|state| {
        let mut registries = state.borrow().registries.borrow_mut();
        //get registries by name and replace value with caller, if not exist then return error
        if let Some(registry) = registries.get_mut(&name) {
            *registry = caller.clone();
        } else {
            return BooleanActorResponse::new(Err(MockError::RegistrationNotFound));
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
                    return BooleanActorResponse::new(Err(MockError::RegistrationNotFound));
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
