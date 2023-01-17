use crate::*;

pub fn initialize_hex_block(
    client: &Client,
    pi: Pubkey,
    hex_block: Pubkey,
    bump: u8,
) -> ClientResult<()> {
    let initialize_hex_block_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new_readonly(pi, false),
            AccountMeta::new(hex_block, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: pisolana::instruction::InitializeHexBlock { bump }.data(),
    };

    send_and_confirm_tx(
        &client,
        [initialize_hex_block_ix].to_vec(),
        None,
        "initialize_hex_block".to_string(),
    )?;

    Ok(())
}
