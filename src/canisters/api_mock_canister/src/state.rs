use crate::errors::{MockError, ServiceResult};
use crate::mock_utils::{normalize_name, FirstLevelName, NameParseResult};
use candid::Principal;
use common::ic_logger::ICLogger;
use common::named_canister_ids::{ensure_current_canister_id_match, CanisterNames};
use common::types::AuthPrincipal;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Once;
thread_local! {
    pub static STATE : State = State::default();
}

static INIT: Once = Once::new();

pub(crate) fn canister_module_init() {
    INIT.call_once(|| {
        ICLogger::init("MockSampleCanister");
    });
    ensure_current_canister_id_match(CanisterNames::MockSampleCanister);
}

#[derive(Default)]
pub struct State {
    // NOTE: When adding new persistent fields here, ensure that these fields
    // are being persisted in the `replace` method below.
    pub(crate) registries: RefCell<HashMap<String, Principal>>,
    pub(crate) approvals: RefCell<HashMap<String, Principal>>,
}

pub fn is_name_owner(name: &FirstLevelName, caller: &Principal) -> ServiceResult<Principal> {
    STATE.with(|s| {
        let store = s.registries.borrow();
        let registration = store.get(name.0.get_name());
        if registration.is_none() {
            return Err(MockError::RegistrationNotFound);
        }
        let registration = registration.unwrap();
        let owner = registration.clone();

        if !owner.eq(caller) {
            return Err(MockError::PermissionDenied);
        }

        Ok(owner)
    })
}

pub fn validate_name(name: &str) -> ServiceResult<FirstLevelName> {
    assert!(!name.is_empty());
    let name = normalize_name(name);
    let result = NameParseResult::parse(&name);
    if result.get_level_count() != 2 {
        return Err(MockError::InvalidName {
            reason: "it must be second level name".to_string(),
        });
    }

    let first = result.get_current_level().unwrap();
    if first.len() > 63 {
        return Err(MockError::InvalidName {
            reason: "second level name must be less than 64 characters".to_string(),
        });
    }

    if !first.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(MockError::InvalidName {
            reason: "name must be alphanumeric or -".to_string(),
        });
    }
    Ok(FirstLevelName(result))
}

pub fn approve(caller: &Principal, name: &str, to: Principal) -> ServiceResult<bool> {
    let name = validate_name(&name.clone())?;
    must_not_anonymous(&caller)?;

    is_name_owner(&name, &caller)?;
    set_approval(&name, &to);
    Ok(true)
}

pub fn must_not_anonymous(caller: &Principal) -> ServiceResult<AuthPrincipal> {
    if *caller == Principal::anonymous() {
        return Err(MockError::Unauthorized);
    }
    Ok(AuthPrincipal(caller.clone()))
}

pub fn set_approval(name: &FirstLevelName, approved_to: &Principal) {
    STATE.with(|s| {
        let mut approvals = s.approvals.borrow_mut();
        if approved_to == &Principal::anonymous() {
            approvals.remove(name.0.get_name());
        } else {
            approvals.insert(name.to_string(), approved_to.clone());
        }
    });
}

pub fn is_approved_to(name: &str, approved_to: &Principal) -> bool {
    STATE.with(|s| {
        let approvals = s.approvals.borrow();
        let approval = approvals.get(name);
        if approval.is_none() {
            return false;
        }
        let approval = approval.unwrap();
        approval.eq(approved_to)
    })
}

pub fn remove_approval(name: &str) {
    STATE.with(|s| {
        let mut approvals = s.approvals.borrow_mut();
        approvals.remove(name);
    });
}
