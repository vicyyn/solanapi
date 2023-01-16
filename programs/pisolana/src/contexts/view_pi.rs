use crate::*;

#[derive(Accounts)]
pub struct ViewPi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[SEED_PI,&pi.id.to_be_bytes()], bump)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=DigitsBlock::pda(pi.id,digits_block.block_id).0)]
    pub digits_block: Account<'info, DigitsBlock>,
}

impl<'info> ViewPi<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self { digits_block, .. } = self;
        msg!("{:02X?}", digits_block.res);
        Ok(())
    }
}
