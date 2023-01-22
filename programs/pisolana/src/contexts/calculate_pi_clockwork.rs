use crate::*;
use clockwork_sdk::{
    self,
    state::{AccountMetaData, InstructionData, Thread, ThreadResponse},
};

#[derive(Accounts)]
pub struct CalculatePiClockwork<'info> {
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=HexBlock::pda(pi.id,pi.current_hex_block).0)]
    pub hex_block: Account<'info, HexBlock>,
    #[account(signer,address = Thread::pubkey(pi.key(),"pi_thread".to_string()))]
    pub clockwork_thread: Box<Account<'info, Thread>>,
}

impl<'info> CalculatePiClockwork<'_> {
    pub fn process(&mut self) -> Result<ThreadResponse> {
        let Self {
            pi,
            hex_block,
            clockwork_thread,
        } = self;
        require!(
            pi.current_hex_block == hex_block.block_id,
            CustomError::InvalidHexBlockProvided
        );

        require!(pi.minted == false, CustomError::PiAlreadyMinted);

        require!(
            pi.last_block_initialized == true,
            CustomError::NextBlockNotInitialized
        );

        pi.pi(hex_block, 10);
        Ok(ThreadResponse {
            next_instruction: Some(InstructionData {
                program_id: crate::ID,
                accounts: vec![
                    AccountMetaData::new(pi.key(), false),
                    AccountMetaData::new(hex_block.key(), false),
                    AccountMetaData::new(clockwork_thread.key(), true),
                ],
                data: clockwork_sdk::utils::anchor_sighash("calculate_pi_clockwork").into(),
            }),
            ..ThreadResponse::default()
        })
    }
}
