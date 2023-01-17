use {
    anchor_lang::{prelude::*, InstructionData},
    clockwork_client::{Client, ClientResult},
    rand::Rng,
    solana_sdk::{
        instruction::Instruction, native_token::LAMPORTS_PER_SOL, signature::Keypair,
        system_program,
    },
    std::io::stdin,
};

pub mod contexts;
pub mod states;
pub mod utils;

pub use contexts::*;
pub use states::*;
pub use utils::*;

fn main() -> ClientResult<()> {
    let payer = Keypair::new();
    let client = Client::new(payer, "http://localhost:8899".into());
    client.airdrop(&client.payer_pubkey(), 1 * LAMPORTS_PER_SOL)?;
    let mut rng = rand::thread_rng();

    let pi_id: u64 = rng.gen();
    let pi = pisolana::Pi::pda(pi_id);
    let mut hex_block = pisolana::HexBlock::pda(pi_id, 0);

    initialize_pi(&client, pi.0, pi_id, pi.1)?;
    initialize_hex_block(&client, pi.0, hex_block.0, hex_block.1)?;

    println!("please provide the number of hex to calculate : ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    let pi_hex: u64 = input.trim().parse().expect("Failed to parse input as u64");

    loop {
        let pi_account = get_pi_account(&client, pi.0);
        let hex_block_account = get_hex_block_account(&client, hex_block.0);
        println!(
            "{:?} {}",
            hex_block_account.hex, pi_account.current_pi_iteration
        );

        if pi_account.current_hex_block > hex_block_account.block_id {
            hex_block = pisolana::HexBlock::pda(pi_id, pi_account.current_hex_block);
            initialize_hex_block(&client, pi.0, hex_block.0, hex_block.1)?;
        }

        if pi_hex - pi_account.current_pi_iteration == 0 {
            break;
        } else if pi_hex - pi_account.current_pi_iteration < 8 {
            calculate_pi(
                &client,
                pi.0,
                hex_block.0,
                (pi_hex - pi_account.current_pi_iteration) as u8,
            )?;
        } else {
            calculate_pi(&client, pi.0, hex_block.0, 10)?;
        }
    }

    Ok(())
}
