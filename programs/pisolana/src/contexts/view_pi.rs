use crate::*;

#[derive(Accounts)]
pub struct ViewPi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[SEED_PI,&pi.id.to_be_bytes()], bump)]
    pub pi: Account<'info, Pi>,
}

impl<'info> ViewPi<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self { pi, .. } = self;
        msg!("{:02X?}", pi.res);
        Ok(())
    }
}
