use crate::*;
use pisolana::HexBlock;

pub fn get_hex_block_account(client: &Client, digits_block: Pubkey) -> HexBlock {
    let buffer = client.get_account_data(&digits_block).unwrap();
    let mut arr: &[u8] = &buffer;
    pisolana::HexBlock::try_deserialize(&mut arr).unwrap()
}
