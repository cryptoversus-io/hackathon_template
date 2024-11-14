// gx-blockchain/src/tests/common.rs
use solana_program::{
    account_info::AccountInfo,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_program,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub struct TestContext {
    pub program_id: Pubkey,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub rent: Rent,
    pub banks_client: BanksClient,
}

impl TestContext {
    pub async fn new() -> Self {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "gx_blockchain",
            program_id,
            processor!(crate::processor::Processor::process),
        );

        let (banks_client, payer, recent_blockhash) = program_test.start().await;
        let rent = banks_client.get_rent().await.unwrap();

        Self {
            program_id,
            payer,
            recent_blockhash,
            rent,
            banks_client,
        }
    }

    pub fn create_bounty_account(&self, lamports: u64) -> (Keypair, Account) {
        let bounty_keypair = Keypair::new();
        let space = std::mem::size_of::<BountyAccount>();
        
        let account = Account::new(
            lamports,
            space,
            &self.program_id,
        );

        (bounty_keypair, account)
    }
}