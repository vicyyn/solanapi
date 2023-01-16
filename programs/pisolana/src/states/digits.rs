use crate::*;

#[account]
pub struct DigitsBlock {
    pub block_id: u64,
    pub res: Vec<u8>,
    pub bump: u8,
}

pub const SEED_DIGITS_BLOCK: &[u8] = b"digits_block";
pub const MAX_PER_BLOCK: usize = 100;

impl DigitsBlock {
    pub fn pda(pi_id: u64, block_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                SEED_DIGITS_BLOCK,
                &pi_id.to_be_bytes(),
                &block_id.to_be_bytes(),
            ],
            &crate::ID,
        )
    }
}

pub trait DigitsAccount {
    fn new(&mut self, block_id: u64, bump: u8) -> Result<()>;
}

impl DigitsAccount for Account<'_, DigitsBlock> {
    fn new(&mut self, block_id: u64, bump: u8) -> Result<()> {
        self.block_id = block_id;
        self.res = vec![];
        self.bump = bump;
        Ok(())
    }
}
