pub mod tokens {
    use solana_program::native_token::LAMPORTS_PER_SOL;

    pub const GAME_TOKEN_DECIMALS: u8 = 9;
    pub const REWARD_TOKEN_DECIMALS: u8 = 6;
    
    pub const INITIAL_MINT_AMOUNT: u64 = 1_000_000 * LAMPORTS_PER_SOL;
    pub const MIN_TOKEN_AMOUNT: u64 = 1_000;
    
    pub const GAME_TOKEN_SYMBOL: &str = "GAME";
    pub const REWARD_TOKEN_SYMBOL: &str = "RWD";
}