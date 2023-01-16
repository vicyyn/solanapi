use crate::*;

#[derive(Accounts)]
pub struct CalculatePi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=DigitsBlock::pda(pi.id,digits_block.block_id).0)]
    pub digits_block: Account<'info, DigitsBlock>,
}

impl<'info> CalculatePi<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self {
            pi, digits_block, ..
        } = self;
        require!(
            pi.current_digits_block == digits_block.block_id,
            CustomError::InvalidDigitsBlockProvided
        );
        pi.pi(digits_block);
        Ok(())
    }
}
