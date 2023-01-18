use crate::*;

pub fn close_hex_block(client: &Client, pi: Pubkey, hex_block: Pubkey) -> ClientResult<()> {
    let close_hex_block_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pi, false),
            AccountMeta::new(hex_block, false),
        ],
        data: pisolana::instruction::CloseHexBlock {}.data(),
    };

    send_and_confirm_tx(
        &client,
        [close_hex_block_ix].to_vec(),
        None,
        "close_hex_block".to_string(),
    )?;

    Ok(())
}
