use async_trait::async_trait;
use candid::{Nat, Principal};
use mockall::{mock, predicate::*};
use rstest::*;

use common::{
    canister_api::*,
    errors::ActorResult,
    types::{ic_ledger_types::*, ic_management_types::*},
};

mock! {
    pub DFTApi { }
    #[async_trait]
    impl IDFTApi for DFTApi {
      async fn transfer_from( &self, spender_sub_account: Option<Subaccount>, from: String, to: String, value: Nat,created_at: Option<u64>) -> ActorResult<DFTTransactionResponse>;
      async fn transfer(&self,from_sub_account: Option<Subaccount>,to: String,value: Nat,created_at: Option<u64>) -> ActorResult<DFTTransactionResponse>;
      async fn balance_of(&self, token_holder: String) -> ActorResult<Nat>;
    }
}

#[fixture]
pub fn mock_dft_api() -> MockDFTApi {
    MockDFTApi::new()
}

mock! {
    pub ICLedgerApi { }
    #[async_trait]
    impl IICLedgerApi for ICLedgerApi {
        async fn transfer(&self, args: TransferArgs) -> ActorResult<TransferResult>;
    }
}

#[fixture]
pub fn mock_ic_ledger_api() -> MockICLedgerApi {
    MockICLedgerApi::new()
}

mock! {
    pub ICManagementAPI { }
    #[async_trait]
    impl IICManagementAPI for ICManagementAPI {
        async fn create_canister(&self, args: CreateCanisterArgs) -> ActorResult<CanisterIdRecord>;
        async fn canister_status(&self, id_record: CanisterIdRecord) -> ActorResult<CanisterStatusResponse>;
        async fn canister_install(&self, canister_id: &Principal,wasm_module: Vec<u8>,args: Vec<u8>) -> ActorResult<()>;
        async fn ecdsa_public_key(&self, get_public_key_req: ECDSAPublicKey) -> ActorResult<ECDSAPublicKeyReply>;
        async fn sign_with_ecdsa(&self, sign_request: SignWithECDSA) -> ActorResult<SignWithECDSAReply>;
    }
}

#[fixture]
pub fn mock_ic_management_api() -> MockICManagementAPI {
    MockICManagementAPI::new()
}
