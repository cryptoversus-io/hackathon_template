use solana_program::pubkey::Pubkey;

pub const PROGRAM_SEED: &str = "flexnet-gx";
pub const ACCOUNT_SEED: &str = "account";

pub fn derive_program_address(seed: &str, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[seed.as_bytes()],
        program_id,
    )
}

pub fn get_program_address(program_id: &Pubkey) -> (Pubkey, u8) {
    derive_program_address(PROGRAM_SEED, program_id)
}

pub fn get_account_address(program_id: &Pubkey) -> (Pubkey, u8) {
    derive_program_address(ACCOUNT_SEED, program_id)
}