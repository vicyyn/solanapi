use crate::*;

#[account]
pub struct Pi {
    pub s: f64,
    pub k: f64,
    pub id: u64,
    pub bump: u8,
}

pub const SEED_PI: &[u8] = b"pi";

impl Pi {
    pub fn pda(id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_PI, &id.to_be_bytes()], &crate::ID)
    }

    pub fn calculate_pi(&mut self) {
        for i in 0..400 {
            if i % 2 == 0 {
                self.s += 4.0 / self.k;
            } else {
                self.s -= 4.0 / self.k
            }
            self.k += 2.0
        }
        self.s = self.s;
        self.k = self.k;
    }
}

pub trait PiAccount {
    fn new(&mut self, id: u64, bump: u8) -> Result<()>;
}

impl PiAccount for Account<'_, Pi> {
    fn new(&mut self, id: u64, bump: u8) -> Result<()> {
        self.s = 0.0;
        self.k = 1.0;
        self.id = id;
        self.bump = bump;
        Ok(())
    }
}
