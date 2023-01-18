use crate::*;

#[derive(Accounts)]
pub struct InitializeHexBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(init,payer=payer, space = 8 + 8 + 1 + 4 + (MAX_HEX_PER_BLOCK / 2), seeds=[SEED_HEX_BLOCK,&pi.id.to_be_bytes(), &pi.current_hex_block.to_be_bytes()], bump)]
    pub hex_block: Account<'info, HexBlock>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeHexBlock<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self { hex_block, pi, .. } = self;
        require!(pi.minted == false, CustomError::PiAlreadyMinted);
        require!(
            pi.last_block_initialized == false,
            CustomError::HexBlockAlreadyInitialized
        );
        pi.set_last_block_initialized();
        hex_block.new(pi.current_hex_block, bump)?;
        Ok(())
    }
}
