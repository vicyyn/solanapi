use anchor_spl::associated_token::get_associated_token_address;

use {
    crate::*,
    anchor_spl::{associated_token, token},
    pisolana::SEED_PI_MINT,
};

pub fn mint_pi(client: &Client, pi: Pubkey, current_pi_iteration: u64) -> ClientResult<()> {
    let (pi_mint, bump) = Pubkey::find_program_address(
        &[SEED_PI_MINT, &current_pi_iteration.to_be_bytes()],
        &pisolana::ID,
    );

    let pi_token_account = get_associated_token_address(&client.payer_pubkey(), &pi_mint);

    let mint_pi_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pi, false),
            AccountMeta::new(pi_mint, false),
            AccountMeta::new(pi_token_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(token::ID, false),
            AccountMeta::new_readonly(associated_token::ID, false),
        ],
        data: pisolana::instruction::MintPi { bump }.data(),
    };

    send_and_confirm_tx(&client, [mint_pi_ix].to_vec(), None, "mint_pi".to_string())?;

    Ok(())
}
