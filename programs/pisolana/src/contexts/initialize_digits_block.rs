use crate::*;

#[derive(Accounts)]
pub struct InitializeDigitsBlock<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(init,payer=payer, space = 8 + 8 + 1 + 4 + MAX_PER_BLOCK, seeds=[SEED_DIGITS_BLOCK,&pi.id.to_be_bytes(), &pi.current_digits_block.to_be_bytes()], bump)]
    pub digits_block: Account<'info, DigitsBlock>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeDigitsBlock<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self {
            digits_block, pi, ..
        } = self;
        digits_block.new(pi.current_digits_block, bump)?;
        Ok(())
    }
}
