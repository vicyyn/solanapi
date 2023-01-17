use crate::*;

#[derive(Accounts)]
pub struct ViewPi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[SEED_PI,&pi.id.to_be_bytes()], bump)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=HexBlock::pda(pi.id,hex_block.block_id).0)]
    pub hex_block: Account<'info, HexBlock>,
}

impl<'info> ViewPi<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self { hex_block, .. } = self;
        msg!("{:02X?}", hex_block.hex);
        Ok(())
    }
}
