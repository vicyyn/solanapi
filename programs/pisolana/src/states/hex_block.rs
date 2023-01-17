use crate::*;

#[account]
pub struct HexBlock {
    pub block_id: u64,
    pub hex: Vec<u8>,
    pub bump: u8,
}

pub const SEED_HEX_BLOCK: &[u8] = b"hex_block";
pub const MAX_PER_BLOCK: usize = 100;

impl HexBlock {
    pub fn pda(pi_id: u64, block_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                SEED_HEX_BLOCK,
                &pi_id.to_be_bytes(),
                &block_id.to_be_bytes(),
            ],
            &crate::ID,
        )
    }
}

pub trait HexBlockAccount {
    fn new(&mut self, block_id: u64, bump: u8) -> Result<()>;
}

impl HexBlockAccount for Account<'_, HexBlock> {
    fn new(&mut self, block_id: u64, bump: u8) -> Result<()> {
        self.block_id = block_id;
        self.hex = vec![];
        self.bump = bump;
        Ok(())
    }
}
