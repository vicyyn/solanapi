use {
    anchor_lang::{prelude::*, InstructionData},
    clockwork_client::{Client, ClientResult},
    rand::Rng,
    solana_sdk::{
        instruction::Instruction, native_token::LAMPORTS_PER_SOL, signature::Keypair,
        system_program,
    },
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
    let mut current_hex_block = 0;
    let mut hex_block = pisolana::HexBlock::pda(pi_id, current_hex_block);

    initialize_pi(&client, pi.0, pi_id, pi.1)?;
    initialize_hex_block(&client, pi.0, hex_block.0, hex_block.1)?;

    loop {
        let pi_account = get_pi_account(&client, pi.0);
        let digits_block_account = get_hex_block_account(&client, hex_block.0);
        println!(
            "{:02X?} {}",
            digits_block_account.hex, pi_account.current_pi_iteration
        );
        if pi_account.current_pi_iteration >= 10000 {
            break;
        }

        if pi_account.current_hex_block > current_hex_block {
            current_hex_block += 1;
            hex_block = pisolana::HexBlock::pda(pi_id, current_hex_block);
            initialize_hex_block(&client, pi.0, hex_block.0, hex_block.1)?;
        }

        calculate_pi(&client, pi.0, hex_block.0, 4)?;
    }

    Ok(())
}
