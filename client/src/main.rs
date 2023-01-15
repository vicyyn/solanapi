use {
    anchor_lang::{prelude::*, InstructionData},
    clockwork_sdk::client::{Client, ClientResult},
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
    let id: u64 = rng.gen();
    let pi = pisolana::Pi::pda(id);

    let initiallize_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pi.0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: pisolana::instruction::Initialize {
            pi_id: id,
            bump: pi.1,
        }
        .data(),
    };

    send_and_confirm_tx(
        &client,
        [initiallize_ix].to_vec(),
        None,
        "initialize".to_string(),
    )?;

    for i in 0..10000 {
        // if i % 5 == 0 {
        //     let view_pi_ix = Instruction {
        //         program_id: pisolana::ID,
        //         accounts: vec![
        //             AccountMeta::new(client.payer_pubkey(), true),
        //             AccountMeta::new(pi.0, false),
        //         ],
        //         data: pisolana::instruction::ViewPi {}.data(),
        //     };

        //     send_and_confirm_tx(&client, [view_pi_ix].to_vec(), None, "view_pi".to_string())?;
        // }
        let calculate_pi_ix = Instruction {
            program_id: pisolana::ID,
            accounts: vec![
                AccountMeta::new(client.payer_pubkey(), true),
                AccountMeta::new(pi.0, false),
            ],
            data: pisolana::instruction::CalculatePi {}.data(),
        };

        print!("{} ", i);
        send_and_confirm_tx(
            &client,
            [calculate_pi_ix].to_vec(),
            None,
            "calculate_pi".to_string(),
        )?;
    }

    Ok(())
}
