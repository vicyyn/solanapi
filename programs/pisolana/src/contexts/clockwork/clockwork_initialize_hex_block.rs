use crate::*;
use clockwork_sdk::state::{AccountMetaData, InstructionData, Thread, ThreadResponse};

#[derive(Accounts)]
pub struct ClockworkInitializeHexBlock<'info> {
    #[account(mut,signer,address = Thread::pubkey(pi.key(),"pi_thread".to_string()))]
    pub clockwork_thread: Box<Account<'info, Thread>>,
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(init,payer=clockwork_thread, space = 8 + 8 + 1 + 4 + (MAX_HEX_PER_BLOCK / 2), seeds=[SEED_HEX_BLOCK,&pi.id.to_be_bytes(), &pi.current_hex_block.to_be_bytes()], bump)]
    pub hex_block: Account<'info, HexBlock>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClockworkInitializeHexBlock<'_> {
    pub fn process(&mut self) -> Result<ThreadResponse> {
        let Self {
            hex_block,
            pi,
            clockwork_thread,
            ..
        } = self;
        require!(pi.minted == false, CustomError::PiAlreadyMinted);
        require!(
            pi.last_block_initialized == false,
            CustomError::HexBlockAlreadyInitialized
        );
        pi.set_last_block_initialized();
        hex_block.new(pi.current_hex_block, 0)?;
        Ok(ThreadResponse {
            next_instruction: Some(InstructionData {
                program_id: crate::ID,
                accounts: vec![
                    AccountMetaData::new(pi.key(), false),
                    AccountMetaData::new(hex_block.key(), false),
                    AccountMetaData::new(clockwork_thread.key(), true),
                ],
                data: clockwork_sdk::utils::anchor_sighash("clockwork_calculate_pi").into(),
            }),
            ..ThreadResponse::default()
        })
    }
}
