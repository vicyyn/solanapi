use crate::*;

pub fn calculate_pi(
    client: &Client,
    pi: Pubkey,
    digits_block: Pubkey,
    digits_to_add: u8,
) -> ClientResult<()> {
    let calculate_pi_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pi, false),
            AccountMeta::new(digits_block, false),
        ],
        data: pisolana::instruction::CalculatePi { digits_to_add }.data(),
    };

    send_and_confirm_tx(
        &client,
        [calculate_pi_ix].to_vec(),
        None,
        "calculate_pi".to_string(),
    )?;

    Ok(())
}
