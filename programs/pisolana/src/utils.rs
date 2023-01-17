use std::fmt::Debug;

use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct TwoHex(u8);

impl Debug for TwoHex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X?}", self.0)?;
        Ok(())
    }
}

impl TwoHex {
    pub fn new(first_hex: TwoHex, second_hex: TwoHex) -> Self {
        let merged = first_hex.get_first_hex().0 + second_hex.get_second_hex().0;
        return TwoHex(merged);
    }

    pub fn get_first_hex(&self) -> TwoHex {
        TwoHex(self.0 & 0b11110000)
    }

    pub fn set_first_hex(&mut self, hex: TwoHex) {
        self.0 = self.0 & 0b00001111;
        self.0 = self.0 | hex.get_first_hex().0;
    }

    pub fn get_second_hex(&self) -> TwoHex {
        TwoHex(self.0 & 0b00001111)
    }

    pub fn set_second_hex(&mut self, hex: TwoHex) {
        self.0 = self.0 & 0b11110000;
        self.0 = self.0 | hex.get_second_hex().0;
    }

    pub fn to_second_hex(&self) -> TwoHex {
        TwoHex(self.0 >> 4)
    }

    pub fn to_first_hex(&self) -> TwoHex {
        TwoHex(self.0 << 4)
    }

    pub fn get_hex_from_bytes(bytes: &[u8], number_of_hex: u8) -> Vec<TwoHex> {
        let mut res = vec![];
        let mut i = 0;
        let mut hex_left = number_of_hex;
        loop {
            match hex_left {
                0 => {
                    return res;
                }
                1 => {
                    res.push(TwoHex(bytes[i]).get_first_hex());
                    return res;
                }
                _ => {
                    res.push(TwoHex(bytes[i]));
                    i += 1;
                    hex_left -= 2;
                }
            }
        }
    }
}
