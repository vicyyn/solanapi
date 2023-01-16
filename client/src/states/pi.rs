use crate::*;
use pisolana::Pi;

pub fn get_pi_account(client: &Client, pi: Pubkey) -> Pi {
    let buffer = client.get_account_data(&pi).unwrap();
    let mut arr: &[u8] = &buffer;
    pisolana::Pi::try_deserialize(&mut arr).unwrap()
}
