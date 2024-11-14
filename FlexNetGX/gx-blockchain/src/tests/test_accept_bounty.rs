// gx-blockchain/src/tests/test_accept_bounty.rs
#[tokio::test]
async fn test_accept_bounty_success() {
    let context = TestContext::new().await;
    let hunter = Keypair::new();
    let (bounty_keypair, mut bounty_account) = context.create_bounty_account(
        context.rent.minimum_balance(std::mem::size_of::<BountyAccount>()),
    );

    // Create a bounty first
    let bounty = BountyAccount {
        creator: Pubkey::new_unique(),
        amount: 50_000_000,
        description_hash: [1; 32],
        status: BountyStatus::Open,
        hunter: None,
    };

    bounty.serialize(&mut bounty_account.data).unwrap();

    // Add accounts
    context.banks_client
        .add_packable_account(
            &hunter.pubkey(),
            100_000_000,
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

    // Create accept instruction
    let instruction = BountyInstruction::AcceptBounty;

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
    );

    transaction.sign(
        &[&context.payer, &hunter],
        context.recent_blockhash,
    );

    // Process transaction
    context.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Verify bounty state
    let bounty_account = context.banks_client
        .get_account(bounty_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let bounty = BountyAccount::try_from_slice(&bounty_account.data).unwrap();
    assert_eq!(bounty.status, BountyStatus::InProgress);
    assert_eq!(bounty.hunter, Some(hunter.pubkey()));
}