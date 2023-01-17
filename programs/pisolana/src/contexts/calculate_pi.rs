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
    pub fn process(&mut self, number_of_hex: u8) -> Result<()> {
        let Self { pi, hex_block, .. } = self;
        require!(
            pi.current_hex_block == hex_block.block_id,
            CustomError::InvalidHexBlockProvided
        );
        require!(
            (1..11).contains(&number_of_hex),
            CustomError::InvalidNumberOfHexProvided
        );

        require!(
            ((pi.current_pi_iteration as usize % MAX_PER_BLOCK) + number_of_hex as usize)
                <= MAX_PER_BLOCK,
            CustomError::HexBlockOverflow
        );

        pi.pi(hex_block, number_of_hex);
        Ok(())
    }
}
