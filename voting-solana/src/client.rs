use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::Transaction,
};
use voting_errors::AppError;

use crate::SolanaConfig;

const MEMO_PROGRAM_ID: &str = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr";

pub struct SolanaClient {
    rpc_client: RpcClient,
    payer: Keypair,
}

impl SolanaClient {
    pub fn new(config: SolanaConfig) -> Result<Self, AppError> {
        let rpc_client = RpcClient::new_with_commitment(
            config.rpc_url,
            CommitmentConfig::confirmed(),
        );
        let payer = Keypair::new();
        Ok(Self { rpc_client, payer })
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    pub fn send_memo(&self, memo: &str) -> Result<Signature, AppError> {
        let memo_program_id: Pubkey = MEMO_PROGRAM_ID
            .parse()
            .map_err(|e| AppError::SolanaError(format!("Invalid memo program ID: {}", e)))?;

        let instruction = Instruction::new_with_bytes(
            memo_program_id,
            memo.as_bytes(),
            vec![AccountMeta::new(self.payer.pubkey(), true)],
        );

        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| AppError::SolanaError(format!("Failed to get blockhash: {}", e)))?;

        let message = Message::new(&[instruction], Some(&self.payer.pubkey()));
        let transaction = Transaction::new(&[&self.payer], message, recent_blockhash);

        let signature = self
            .rpc_client
            .send_and_confirm_transaction(&transaction)
            .map_err(|e| AppError::SolanaError(format!("Transaction failed: {}", e)))?;

        Ok(signature)
    }

    pub fn request_airdrop(&self, lamports: u64) -> Result<Signature, AppError> {
        self.rpc_client
            .request_airdrop(&self.payer.pubkey(), lamports)
            .map_err(|e| AppError::SolanaError(format!("Airdrop failed: {}", e)))
    }

    pub fn get_balance(&self) -> Result<u64, AppError> {
        self.rpc_client
            .get_balance(&self.payer.pubkey())
            .map_err(|e| AppError::SolanaError(format!("Failed to get balance: {}", e)))
    }
}
