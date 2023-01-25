use crate::*;
use anchor_lang::solana_program::system_program;
use clockwork_sdk::{
    state::{AccountMetaData, InstructionData, Thread, ThreadResponse},
    utils::PAYER_PUBKEY,
};

#[derive(Accounts)]
pub struct ClockworkCalculatePi<'info> {
    #[account(mut,address=Pi::pda(pi.id).0)]
    pub pi: Account<'info, Pi>,
    #[account(mut,address=HexBlock::pda(pi.id,pi.current_hex_block).0)]
    pub hex_block: Account<'info, HexBlock>,
    #[account(mut,signer,address = Thread::pubkey(pi.key(),"pi_thread".to_string()))]
    pub clockwork_thread: Box<Account<'info, Thread>>,
}

impl<'info> ClockworkCalculatePi<'_> {
    pub fn process(&mut self) -> Result<ThreadResponse> {
        let Self {
            pi,
            hex_block,
            clockwork_thread,
        } = self;
        require!(pi.minted == false, CustomError::PiAlreadyMinted);

        let hex_left = MAX_HEX_PER_BLOCK - pi.current_pi_iteration as usize % MAX_HEX_PER_BLOCK;
        if hex_left < 10 {
            pi.pi(hex_block, hex_left as u8)
        } else {
            pi.pi(hex_block, 10);
        }

        if pi.current_hex_block > hex_block.block_id {
            let new_hex_block = HexBlock::pda(pi.id, pi.current_hex_block);

            return Ok(ThreadResponse {
                next_instruction: Some(InstructionData {
                    program_id: crate::ID,
                    accounts: vec![
                        AccountMetaData::new(PAYER_PUBKEY, true),
                        AccountMetaData::new(clockwork_thread.key(), true),
                        AccountMetaData::new(pi.key(), false),
                        AccountMetaData::new(new_hex_block.0, false),
                        AccountMetaData::new_readonly(system_program::ID, false),
                    ],
                    data: clockwork_sdk::utils::anchor_sighash("clockwork_initialize_hex_block")
                        .into(),
                }),
                ..ThreadResponse::default()
            });
        }

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
