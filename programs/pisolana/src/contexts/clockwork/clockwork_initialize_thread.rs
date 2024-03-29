use crate::*;
use anchor_lang::solana_program::instruction::Instruction;

use clockwork_sdk::{
    state::{Thread, ThreadSettings, Trigger},
    ThreadProgram,
};

#[derive(Accounts)]
pub struct ClockworkInitializeThread<'info> {
    pub payer: Signer<'info>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=HexBlock::pda(pi.id,pi.current_hex_block).0)]
    pub hex_block: Account<'info, HexBlock>,
    #[account(mut,address = Thread::pubkey(pi.key(),"pi_thread".to_string()))]
    pub clockwork_thread: SystemAccount<'info>,
    #[account(address = ThreadProgram::id())]
    pub thread_program: Program<'info, ThreadProgram>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClockworkInitializeThread<'_> {
    pub fn process(&mut self) -> Result<()> {
        let Self {
            payer,
            thread_program,
            system_program,
            clockwork_thread,
            pi,
            hex_block,
            ..
        } = self;

        let calculate_pi_clockwork_ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(pi.key(), false),
                AccountMeta::new(hex_block.key(), false),
                AccountMeta::new(clockwork_thread.key(), true),
            ],
            data: clockwork_sdk::utils::anchor_sighash("clockwork_calculate_pi").into(),
        };

        clockwork_sdk::cpi::thread_create(
            CpiContext::new_with_signer(
                thread_program.to_account_info(),
                clockwork_sdk::cpi::ThreadCreate {
                    authority: pi.to_account_info(),
                    payer: payer.to_account_info(),
                    system_program: system_program.to_account_info(),
                    thread: clockwork_thread.to_account_info(),
                },
                &[&[SEED_PI, &pi.id.to_be_bytes(), &[pi.bump]]],
            ),
            "pi_thread".to_string(),
            calculate_pi_clockwork_ix.into(),
            Trigger::Immediate {},
        )?;

        clockwork_sdk::cpi::thread_update(
            CpiContext::new_with_signer(
                thread_program.to_account_info(),
                clockwork_sdk::cpi::ThreadUpdate {
                    authority: pi.to_account_info(),
                    system_program: system_program.to_account_info(),
                    thread: clockwork_thread.to_account_info(),
                },
                &[&[SEED_PI, &pi.id.to_be_bytes(), &[pi.bump]]],
            ),
            ThreadSettings {
                rate_limit: Some(32),
                trigger: Some(Trigger::Immediate {}),
                kickoff_instruction: None,
                fee: None,
            },
        )?;

        Ok(())
    }
}
