use ic_cdk::api;

use super::*;
use crate::named_canister_ids::CanisterNames;
type CanisterID = Principal;

#[derive(Debug)]
pub struct DFTApi(CanisterID);

impl Default for DFTApi {
    fn default() -> Self {
        Self(CanisterID::anonymous())
    }
}

#[async_trait]
impl IDFTApi for DFTApi {
    async fn transfer_from(
        &self,
        spender_sub_account: Option<Subaccount>,
        from: String,
        to: String,
        value: Nat,
        nonce: Option<u64>,
    ) -> ActorResult<TransactionResponse> {
        call_canister_as_actor_result(
            CanisterNames::DFTCanister(self.0),
            "transferFrom",
            (spender_sub_account, from, to, value, nonce),
        )
        .await
    }

    async fn transfer(
        &self,
        from_sub_account: Option<Subaccount>,
        to: String,
        value: Nat,
        nonce: Option<u64>,
    ) -> ActorResult<TransactionResponse> {
        call_canister_as_actor_result(
            CanisterNames::DFTCanister(self.0),
            "transfer",
            (from_sub_account, to, value, nonce),
        )
        .await
    }

    async fn balance_of(&self, token_holder: String) -> ActorResult<Nat> {
        call_canister_as_result(
            CanisterNames::DFTCanister(self.0),
            "balanceOf",
            (token_holder,),
        )
        .await
    }
}

#[derive(Default)]
pub struct ICLedgerApi;

#[async_trait]
impl IICLedgerApi for ICLedgerApi {
    async fn transfer(&self, args: TransferArgs) -> ActorResult<TransferResult> {
        call_canister_as_result(CanisterNames::ICLedger, "transfer", (args,)).await
    }
}

#[derive(Default)]
pub struct ICManagementAPI;

#[cfg_attr(coverage_nightly, no_coverage)]
#[async_trait]
impl IICManagementAPI for ICManagementAPI {
    async fn create_canister(&self, args: CreateCanisterArgs) -> Result<CanisterIdRecord, String> {
        #[derive(CandidType)]
        struct In {
            settings: Option<CanisterSettings>,
        }
        let in_arg = In {
            settings: Some(args.settings),
        };

        let (create_result,): (CanisterIdRecord,) = match api::call::call_with_payment(
            Principal::management_canister(),
            "create_canister",
            (in_arg,),
            args.cycles,
        )
        .await
        {
            Ok(x) => x,
            Err((code, msg)) => {
                return Err(format!(
                    "An error happened during the call: {}: {}",
                    code as u8, msg
                ));
            }
        };

        Ok(create_result)
    }

    async fn canister_status(
        &self,
        id_record: CanisterIdRecord,
    ) -> Result<CanisterStatusResponse, String> {
        let res: Result<(CanisterStatusResponse,), _> = api::call::call(
            Principal::management_canister(),
            "canister_status",
            (id_record,),
        )
        .await;
        match res {
            Ok(x) => Ok(x.0),
            Err((code, msg)) => Err(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            )),
        }
    }

    async fn canister_install(
        &self,
        canister_id: &Principal,
        wasm_module: Vec<u8>,
        args: Vec<u8>,
    ) -> Result<(), String> {
        let install_config = CanisterInstall {
            mode: InstallMode::Install,
            canister_id: *canister_id,
            wasm_module: wasm_module.clone(),
            arg: args,
        };

        match api::call::call(
            Principal::management_canister(),
            "install_code",
            (install_config,),
        )
        .await
        {
            Ok(x) => x,
            Err((code, msg)) => {
                return Err(format!(
                    "An error happened during the call: {}: {}",
                    code as u8, msg
                ));
            }
        };
        Ok(())
    }
}
