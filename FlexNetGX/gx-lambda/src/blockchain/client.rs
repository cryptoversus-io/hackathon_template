use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use crate::models::BountyAccount;
use anyhow::{Result, anyhow};

pub struct SolanaClient {
    rpc_client: RpcClient,
    program_id: Pubkey,
    payer: Keypair,
}

impl SolanaClient {
    pub fn new() -> Result<Self> {
        let rpc_url = std::env::var("SOLANA_RPC_URL")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        
        let program_id = std::env::var("SOLANA_PROGRAM_ID")
            .map_err(|_| anyhow!("SOLANA_PROGRAM_ID not set"))?
            .parse()
            .map_err(|_| anyhow!("Invalid program ID"))?;

        // In production, you'd want to use proper key management
        let payer = Keypair::new();

        Ok(Self {
            rpc_client: RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed()),
            program_id,
            payer,
        })
    }

    pub async fn create_bounty(
        &self,
        amount: u64,
        description_hash: [u8; 32],
        creator: &Pubkey,
    ) -> Result<(String, Pubkey)> {
        let bounty_account = Keypair::new();
        let rent = self.rpc_client.get_minimum_balance_for_rent_exemption(
            std::mem::size_of::<BountyAccount>(),
        )?;

        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &self.payer.pubkey(),
                    &bounty_account.pubkey(),
                    rent,
                    std::mem::size_of::<BountyAccount>() as u64,
                    &self.program_id,
                ),
                Instruction::new_with_borsh(
                    self.program_id,
                    &BountyInstruction::CreateBounty {
                        amount,
                        description_hash,
                    },
                    vec![
                        AccountMeta::new(bounty_account.pubkey(), true),
                        AccountMeta::new_readonly(*creator, true),
                    ],
                ),
            ],
            Some(&self.payer.pubkey()),
        );

        let blockhash = self.rpc_client.get_latest_blockhash()?;
        transaction.sign(&[&self.payer, &bounty_account], blockhash);

        let signature = self.rpc_client
            .send_and_confirm_transaction_with_spinner(&transaction)?;

        Ok((signature.to_string(), bounty_account.pubkey()))
    }

    pub async fn submit_work(
        &self,
        bounty_pubkey: &Pubkey,
        submission_hash: [u8; 32],
        hunter: &Pubkey,
    ) -> Result<String> {
        let instruction = Instruction::new_with_borsh(
            self.program_id,
            &BountyInstruction::SubmitWork { submission_hash },
            vec![
                AccountMeta::new(*bounty_pubkey, false),
                AccountMeta::new_readonly(*hunter, true),
            ],
        );

        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&self.payer.pubkey()),
        );

        let blockhash = self.rpc_client.get_latest_blockhash()?;
        transaction.sign(&[&self.payer], blockhash);

        let signature = self.rpc_client
            .send_and_confirm_transaction_with_spinner(&transaction)?;

        Ok(signature.to_string())
    }

    pub async fn monitor_transaction(&self, signature: &str) -> Result<bool> {
        let sig = Signature::from_str(signature)
            .map_err(|_| anyhow!("Invalid signature"))?;

        let commitment = CommitmentConfig::finalized();
        let status = self.rpc_client
            .get_signature_status_with_commitment(&sig, commitment)?;

        match status {
            Some(Ok(_)) => Ok(true),
            Some(Err(e)) => Err(anyhow!("Transaction failed: {:?}", e)),
            None => Ok(false),
        }
    }

    pub async fn get_bounty_account(&self, pubkey: &Pubkey) -> Result<BountyAccount> {
        let account = self.rpc_client.get_account(pubkey)?;
        bincode::deserialize(&account.data)
            .map_err(|e| anyhow!("Failed to deserialize bounty account: {}", e))
    }
}
