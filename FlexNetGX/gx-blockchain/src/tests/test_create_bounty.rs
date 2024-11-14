// gx-blockchain/src/tests/test_create_bounty.rs
use super::common::TestContext;
use crate::{
    instruction::BountyInstruction,
    state::{BountyAccount, BountyStatus},
};
use solana_program::system_program;
use solana_sdk::{signature::Keypair, transaction::Transaction};

#[tokio::test]
async fn test_create_bounty_success() {
    let context = TestContext::new().await;
    let creator = Keypair::new();
    let (bounty_keypair, bounty_account) = context.create_bounty_account(
        context.rent.minimum_balance(std::mem::size_of::<BountyAccount>()),
    );

    // Add accounts to test environment
    context.banks_client
        .add_packable_account(
            &creator.pubkey(),
            100_000_000, // enough for rent + bounty amount
            &Account::default(),
            &system_program::id(),
        )
        .await;

    context.banks_client
        .add_packable_account(
            &bounty_keypair.pubkey(),
            bounty_account.lamports,
            &bounty_account,
            &context.program_id,
        )
        .await;

    // Create instruction
    let instruction = BountyInstruction::CreateBounty {
        amount: 50_000_000,
        description_hash: [1; 32],
    };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
    );

    transaction.sign(
        &[&context.payer, &creator, &bounty_keypair],
        context.recent_blockhash,
    );

    // Process transaction
    context.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify bounty account
    let bounty_account = context.banks_client
        .get_account(bounty_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let bounty = BountyAccount::try_from_slice(&bounty_account.data).unwrap();
    assert_eq!(bounty.creator, creator.pubkey());
    assert_eq!(bounty.amount, 50_000_000);
    assert_eq!(bounty.status, BountyStatus::Open);
    assert_eq!(bounty.hunter, None);
}