use std::fmt::Debug;

use async_trait::async_trait;
use candid::{CandidType, Nat, Principal};
use ic_cdk::call;
use log::{debug, error};
use serde::Deserialize;

use crate::errors::ActorResult;
use crate::types::ic_ledger_types::{Subaccount, TransferArgs, TransferResult};
use crate::types::ic_management_types::*;

pub type TransactionId = String;

#[derive(CandidType, Debug, Clone, Deserialize)]
pub struct DFTTransactionResponse {
    #[serde(rename = "txId")]
    pub tx_id: TransactionId,
    #[serde(rename = "blockHeight")]
    pub block_height: Nat,
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
    ) -> ActorResult<DFTTransactionResponse>;

    async fn transfer(
        &self,
        from_sub_account: Option<Subaccount>,
        to: String,
        value: Nat,
        created_at: Option<u64>,
    ) -> ActorResult<DFTTransactionResponse>;

    async fn balance_of(&self, token_holder: String) -> ActorResult<Nat>;
}

#[async_trait]
pub trait IICLedgerApi {
    async fn transfer(&self, args: TransferArgs) -> ActorResult<TransferResult>;
}

#[async_trait]
pub trait IICManagementAPI {
    async fn create_canister(&self, args: CreateCanisterArgs) -> ActorResult<CanisterIdRecord>;
    async fn canister_status(
        &self,
        id_record: CanisterIdRecord,
    ) -> ActorResult<CanisterStatusResponse>;
    async fn canister_install(
        &self,
        canister_id: &Principal,
        wasm_module: Vec<u8>,
        args: Vec<u8>,
    ) -> ActorResult<()>;
    async fn ecdsa_public_key(
        &self,
        get_public_key_req: ECDSAPublicKey,
    ) -> ActorResult<ECDSAPublicKeyReply>;
    async fn sign_with_ecdsa(&self, sign_request: SignWithECDSA)
        -> ActorResult<SignWithECDSAReply>;
}
