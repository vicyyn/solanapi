use crate::*;

#[account]
pub struct HexBlock {
    pub block_id: u64,
    pub hex: Vec<TwoHex>,
    pub bump: u8,
}

pub const SEED_HEX_BLOCK: &[u8] = b"hex_block";
pub const MAX_HEX_PER_BLOCK: usize = 8000;

impl HexBlock {
    pub fn pda(pi_id: u64, block_id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                SEED_HEX_BLOCK,
                &pi_id.to_be_bytes(),
                &block_id.to_be_bytes(),
            ],
            &crate::ID,
        )
    }

    pub fn extend_hex(&mut self, bytes: &[u8], number_of_hex: u8, current_pi_iteration: u64) {
        let pi_hex = TwoHex::get_hex_from_bytes(&bytes, number_of_hex);
        let push_last = number_of_hex % 2 == 0;

        if current_pi_iteration % 2 == 0 {
            self.extend_hex_even(pi_hex);
        } else {
            self.extend_hex_uneven(pi_hex, push_last);
        }
    }

    fn extend_hex_even(&mut self, new_hex: Vec<TwoHex>) {
        self.hex.extend(new_hex)
    }
    fn extend_hex_uneven(&mut self, new_hex: Vec<TwoHex>, push_last: bool) {
        let first_hex = self.hex.last().unwrap().get_first_hex();
        let second_hex = new_hex[0].get_first_hex().to_second_hex();
        let hex = TwoHex::new(first_hex, second_hex);
        let length = self.hex.len();
        self.hex[length - 1] = hex;

        for i in 0..new_hex.len() {
            let first_hex = new_hex[i].get_second_hex().to_first_hex();
            if i == new_hex.len() - 1 {
                if push_last {
                    self.hex.push(first_hex);
                }
                break;
            }
            let second_hex = new_hex[i + 1].get_first_hex().to_second_hex();
            let hex = TwoHex::new(first_hex, second_hex);
            self.hex.push(hex);
        }
    }
}

pub trait HexBlockAccount {
    fn new(&mut self, block_id: u64, bump: u8) -> Result<()>;
}

impl HexBlockAccount for Account<'_, HexBlock> {
    fn new(&mut self, block_id: u64, bump: u8) -> Result<()> {
        self.block_id = block_id;
        self.hex = vec![];
        self.bump = bump;
        Ok(())
    }
}
