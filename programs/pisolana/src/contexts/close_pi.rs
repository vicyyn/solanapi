use crate::*;

#[derive(Accounts)]
pub struct ClosePi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0,close=payer)]
    pub pi: Account<'info, Pi>,
}

impl<'info> ClosePi<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self { pi, .. } = self;
        require!(pi.minted == true, CustomError::PiAlreadyMinted);
        require!(
            pi.current_hex_block == 0,
            CustomError::CannotClosePiBeforeHexBlock
        );
        Ok(())
    }
}
