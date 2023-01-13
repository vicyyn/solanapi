use crate::*;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use std::str::FromStr;

#[account]
pub struct Pi {
    pub n: u128,
    pub plus: bool,
    pub bump: u8,
    pub id: u64,
    pub res: String,
}

pub const SEED_PI: &[u8] = b"pi";

impl Pi {
    pub fn pda(id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_PI, &id.to_be_bytes()], &crate::ID)
    }

    pub fn calculate_pi(&mut self, x: i64) {
        let one: BigDecimal = BigDecimal::from_str("1.0").unwrap();
        let two: BigDecimal = BigDecimal::from_str("2.0").unwrap();
        let three: BigDecimal = BigDecimal::from_str("3.0").unwrap();
        let four: BigDecimal = BigDecimal::from_str("4.0").unwrap();

        let mut output = BigDecimal::from_str(&self.res).unwrap();
        let mut n = BigDecimal::from_u128(self.n).unwrap();
        let mut plus = self.plus;

        for _ in 0..x {
            if plus {
                output = output + (&four / self.multiply_consecutive(&n, &one, &two, &three));
            } else {
                output = output - (&four / self.multiply_consecutive(&n, &one, &two, &three));
            }

            plus = !plus;
            n += &two;
        }
        self.res = output.to_string();
        self.n = n.to_u128().unwrap();
        self.plus = plus;
    }

    pub fn multiply_consecutive(
        &self,
        n: &BigDecimal,
        one: &BigDecimal,
        two: &BigDecimal,
        three: &BigDecimal,
    ) -> BigDecimal {
        (n + one) * (n + two) * (n + three)
    }
}

pub trait PiAccount {
    fn new(&mut self, id: u64, bump: u8) -> Result<()>;
}

impl PiAccount for Account<'_, Pi> {
    fn new(&mut self, id: u64, bump: u8) -> Result<()> {
        self.n = 1;
        self.plus = true;
        self.id = id;
        self.res = String::from("3.0");
        self.bump = bump;
        Ok(())
    }
}
