use crate::*;
use pisolana::DigitsBlock;

pub fn get_digits_block_account(client: &Client, digits_block: Pubkey) -> DigitsBlock {
    let buffer = client.get_account_data(&digits_block).unwrap();
    let mut arr: &[u8] = &buffer;
    pisolana::DigitsBlock::try_deserialize(&mut arr).unwrap()
}
