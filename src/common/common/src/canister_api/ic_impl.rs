use candid::Principal;

use crate::constants::*;
use crate::named_canister_ids::CanisterNames;
use crate::types::CanisterId;

use super::*;

#[derive(Debug)]
pub struct DFTApi(pub CanisterId);

impl Default for DFTApi {
    fn default() -> Self {
        DFTApi(get_named_canister_id(CanisterNames::DFTCanister(
            CanisterId(Principal::anonymous()),
        )))
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
    ) -> ActorResult<DFTTransactionResponse> {
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
    ) -> ActorResult<DFTTransactionResponse> {
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

/// 10B cycles corresponds to 1 SDR cent. Assuming we can create 1 signature per
/// second, that would come to  26k SDR per month if we spent the whole time
/// creating signatures. At 13 nodes and 2k SDR per node per month this would
/// cover the cost of the subnet.
pub const ECDSA_SIGNATURE_FEE: u64 = 10_000_000_000;

#[cfg_attr(coverage_nightly, no_coverage)]
#[async_trait]
impl IICManagementAPI for ICManagementAPI {
    async fn create_canister(&self, args: CreateCanisterArgs) -> ActorResult<CanisterIdRecord> {
        #[derive(CandidType)]
        struct In {
            settings: Option<CanisterSettings>,
        }
        let in_arg = In {
            settings: Some(args.settings),
        };
        call_canister_as_result(CanisterNames::ICManagement, "create_canister", (in_arg,)).await
    }

    async fn canister_status(
        &self,
        id_record: CanisterIdRecord,
    ) -> ActorResult<CanisterStatusResponse> {
        call_canister_as_result(CanisterNames::ICManagement, "canister_status", (id_record,)).await
    }

    async fn canister_install(
        &self,
        canister_id: &Principal,
        wasm_module: Vec<u8>,
        args: Vec<u8>,
    ) -> ActorResult<()> {
        let install_config = CanisterInstall {
            mode: InstallMode::Install,
            canister_id: *canister_id,
            wasm_module: wasm_module.clone(),
            arg: args,
        };
        call_canister_as_result(
            CanisterNames::ICManagement,
            "install_code",
            (install_config,),
        )
        .await
    }

    async fn ecdsa_public_key(
        &self,
        get_public_key_req: ECDSAPublicKey,
    ) -> ActorResult<ECDSAPublicKeyReply> {
        call_canister_as_result(
            CanisterNames::ICManagement,
            "ecdsa_public_key",
            (get_public_key_req,),
        )
        .await
    }

    async fn sign_with_ecdsa(
        &self,
        sign_request: SignWithECDSA,
    ) -> ActorResult<SignWithECDSAReply> {
        call_canister_with_payment_as_result(
            CanisterNames::ICManagement,
            "sign_with_ecdsa",
            (sign_request,),
            ECDSA_SIGNATURE_FEE,
        )
        .await
    }
}
