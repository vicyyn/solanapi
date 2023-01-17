use crate::*;

#[derive(Accounts)]
pub struct CalculatePi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=HexBlock::pda(pi.id,hex_block.block_id).0)]
    pub hex_block: Account<'info, HexBlock>,
}

impl<'info> CalculatePi<'_> {
    pub fn process(&mut self, digits_to_add: u8) -> Result<()> {
        let Self { pi, hex_block, .. } = self;
        require!(
            pi.current_hex_block == hex_block.block_id,
            CustomError::InvalidHexBlockProvided
        );
        require!(
            (1..5).contains(&digits_to_add),
            CustomError::InvalidNumberOfHexProvided
        );
        pi.pi(hex_block, digits_to_add);
        Ok(())
    }
}
