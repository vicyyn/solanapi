use crate::*;

#[derive(Accounts)]
#[instruction(id:u64)]
pub struct InitializePi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init,payer=payer, space = 8 + size_of::<Pi>(), seeds=[SEED_PI,&id.to_be_bytes()], bump)]
    pub pi: Account<'info, Pi>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePi<'_> {
    pub fn process(&mut self, id: u64, bump: u8) -> Result<()> {
        let Self { pi, .. } = self;
        pi.new(id, bump)?;
        Ok(())
    }
}
