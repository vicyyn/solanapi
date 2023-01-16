use crate::*;

pub fn initialize_digits_block(
    client: &Client,
    pi: Pubkey,
    digits_block: Pubkey,
    bump: u8,
) -> ClientResult<()> {
    let initialize_digits_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new_readonly(pi, false),
            AccountMeta::new(digits_block, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: pisolana::instruction::InitializeDigitsBlock { bump }.data(),
    };

    send_and_confirm_tx(
        &client,
        [initialize_digits_ix].to_vec(),
        None,
        "initialize_digits_block".to_string(),
    )?;

    Ok(())
}
