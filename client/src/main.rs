use {
    anchor_lang::{prelude::*, InstructionData},
    clockwork_sdk::client::{Client, ClientResult},
    solana_sdk::{instruction::Instruction, native_token::LAMPORTS_PER_SOL, signature::Keypair},
};

pub mod utils;
pub use utils::*;

fn main() -> ClientResult<()> {
    let payer = Keypair::new();
    let client = Client::new(payer, "https://api.devnet.solana.com".into());
    client.airdrop(&client.payer_pubkey(), 1 * LAMPORTS_PER_SOL)?;

    let initiallize_ix = Instruction {
        program_id: pisolana::ID,
        accounts: vec![AccountMeta::new(client.payer_pubkey(), true)],
        data: pisolana::instruction::Initialize {}.data(),
    };

    send_and_confirm_tx(
        &client,
        [initiallize_ix].to_vec(),
        None,
        "initialize".to_string(),
    )?;

    Ok(())
}
