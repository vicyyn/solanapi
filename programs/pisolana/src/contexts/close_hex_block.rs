use crate::*;

#[derive(Accounts)]
pub struct CloseHexBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=HexBlock::pda(pi.id,pi.current_hex_block).0, close=payer)]
    pub hex_block: Account<'info, HexBlock>,
}

impl<'info> CloseHexBlock<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self { pi, hex_block, .. } = self;
        require!(
            pi.current_hex_block == hex_block.block_id,
            CustomError::InvalidHexBlockProvided
        );
        require!(pi.minted == true, CustomError::PiAlreadyMinted);

        pi.decrement_current_hex_block();
        Ok(())
    }
}
