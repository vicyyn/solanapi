use crate::*;

pub fn close_pi(client: &Client, pi: Pubkey) -> ClientResult<()> {
    let close_pi_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pi, false),
        ],
        data: pisolana::instruction::ClosePi {}.data(),
    };

    send_and_confirm_tx(
        &client,
        [close_pi_ix].to_vec(),
        None,
        "close_pi".to_string(),
    )?;

    Ok(())
}
