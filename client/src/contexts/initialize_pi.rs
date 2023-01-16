use crate::*;

pub fn initialize_pi(client: &Client, pi: Pubkey, pi_id: u64, bump: u8) -> ClientResult<()> {
    let initiallize_pi_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pi, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: pisolana::instruction::InitializePi { pi_id, bump }.data(),
    };

    send_and_confirm_tx(
        &client,
        [initiallize_pi_ix].to_vec(),
        None,
        "initialize_pi".to_string(),
    )?;

    Ok(())
}
