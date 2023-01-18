use crate::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct MintPi<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = pi_mint,
        seeds=[SEED_PI_MINT,&pi.current_pi_iteration.to_be_bytes()],
        bump,
    )]
    pub pi_mint: Account<'info, Mint>,
    #[account(init,
        payer = payer,
        associated_token::mint = pi_mint,
        associated_token::authority = payer,
    )]
    pub pi_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> MintPi<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self {
            pi,
            pi_mint,
            pi_token_account,
            token_program,
            ..
        } = self;

        let cpi_accounts = MintTo {
            mint: pi_mint.to_account_info(),
            to: pi_token_account.to_account_info(),
            authority: pi_mint.to_account_info(),
        };
        let cpi_program = token_program.to_account_info();
        let seeds = &[
            SEED_PI_MINT,
            &pi.current_pi_iteration.to_be_bytes(),
            &[bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        mint_to(cpi_ctx, 1)?;

        Ok(())
    }
}
