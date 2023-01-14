use crate::*;

#[derive(Accounts)]
pub struct CalculatePi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
}

impl<'info> CalculatePi<'_> {
    pub fn process(&mut self, digit: u64) -> Result<()> {
        let Self { pi, .. } = self;
        pi.pi(digit);
        Ok(())
    }
}
