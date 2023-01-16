use {
    anchor_lang::{prelude::*, InstructionData},
    clockwork_client::{Client, ClientResult},
    rand::Rng,
    solana_sdk::{instruction::Instruction, native_token::LAMPORTS_PER_SOL, signature::Keypair},
};

pub mod utils;
use anchor_lang::system_program;
pub use utils::*;

fn main() -> ClientResult<()> {
    let payer = Keypair::new();
    let client = Client::new(payer, "http://localhost:8899".into());
    client.airdrop(&client.payer_pubkey(), 1 * LAMPORTS_PER_SOL)?;

    let mut rng = rand::thread_rng();
    let pi_id: u64 = rng.gen();
    let pi = pisolana::Pi::pda(pi_id);

    // initialize pi
    let initiallize_pi_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pi.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: pisolana::instruction::InitializePi { pi_id, bump: pi.1 }.data(),
    };

    send_and_confirm_tx(
        &client,
        [initiallize_pi_ix].to_vec(),
        None,
        "initialize_pi".to_string(),
    )?;

    let mut current_digits_block = 0;
    let mut digits_block = pisolana::DigitsBlock::pda(pi_id, current_digits_block);

    // initialize digits
    let initialize_digits_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new_readonly(pi.0, false),
            AccountMeta::new(digits_block.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: pisolana::instruction::InitializeDigitsBlock {
            bump: digits_block.1,
        }
        .data(),
    };

    send_and_confirm_tx(
        &client,
        [initialize_digits_ix].to_vec(),
        None,
        "initialize_digits_block".to_string(),
    )?;

    for i in 0..10000 {
        print!("{} ", i);

        let buffer = client.get_account_data(&pi.0).unwrap();
        let mut arr: &[u8] = &buffer;
        let pi_data = pisolana::Pi::try_deserialize(&mut arr).unwrap();

        let buffer = client.get_account_data(&digits_block.0).unwrap();
        let mut arr: &[u8] = &buffer;
        let digits_block_data = pisolana::DigitsBlock::try_deserialize(&mut arr).unwrap();
        println!("{:?}", digits_block_data.res);

        if pi_data.current_digits_block > current_digits_block {
            current_digits_block += 1;
            digits_block = pisolana::DigitsBlock::pda(pi_id, current_digits_block);
            let initialize_digits_ix = Instruction {
                program_id: pisolana::ID,
                accounts: vec![
                    AccountMeta::new(client.payer_pubkey(), true),
                    AccountMeta::new_readonly(pi.0, false),
                    AccountMeta::new(digits_block.0, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: pisolana::instruction::InitializeDigitsBlock {
                    bump: digits_block.1,
                }
                .data(),
            };

            send_and_confirm_tx(
                &client,
                [initialize_digits_ix].to_vec(),
                None,
                "initialize_digits_block".to_string(),
            )?;
        }

        let calculate_pi_ix = Instruction {
            program_id: pisolana::ID,
            accounts: vec![
                AccountMeta::new(client.payer_pubkey(), true),
                AccountMeta::new(pi.0, false),
                AccountMeta::new(digits_block.0, false),
            ],
            data: pisolana::instruction::CalculatePi {}.data(),
        };

        send_and_confirm_tx(
            &client,
            [calculate_pi_ix].to_vec(),
            None,
            "calculate_pi".to_string(),
        )?;
    }

    Ok(())
}
