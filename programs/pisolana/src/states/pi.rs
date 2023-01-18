use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum Step {
    X1Left,
    X1Right,
    X2Left,
    X2Right,
    X3Left,
    X3Right,
    X4Left,
    X4Right,
    Final,
}

#[account]
pub struct Pi {
    pub id: u64,
    pub step: Step,
    pub current_pi_iteration: u64,
    pub current_hex_block: u64,
    pub minted: bool,
    pub r: u64,
    pub k: u64,
    pub s: f64,
    pub x: f64,
    pub bump: u8,
}

pub const SEED_PI: &[u8] = b"pi";
pub const SEED_PI_MINT: &[u8] = b"pi_mint";

impl Pi {
    pub fn pda(id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_PI, &id.to_be_bytes()], &crate::ID)
    }

    pub fn left_sum(&mut self, j: u64) -> (f64, bool) {
        let mut s = self.s;
        let mut k = self.k;
        let mut r: u64;
        while k <= self.current_pi_iteration {
            r = 8 * k + j;
            s = (s + self.pow_mod(16, self.current_pi_iteration - k, r) as f64 / r as f64) % 1.0;
            if k % 70 == 0 && k != 0 {
                self.k = k + 1;
                self.r = r;
                self.s = s;
                return (0.0, false);
            }
            k += 1;
        }

        self.s = 0.0;
        self.k = 0;
        self.r = 0;
        return (s, true);
    }

    pub fn right_sum(&self, j: u64) -> f64 {
        let mut t = 0.0;
        let mut k = self.current_pi_iteration + 1;
        let mut i = -1;
        let mut newt: f64;
        for _ in 0..10 {
            newt = t + 16_f64.powi(i) / (8 * k + j) as f64;
            if t == newt {
                break;
            } else {
                t = newt;
            }
            k += 1;
            i = i - 1;
        }
        return t;
    }

    // calcualte pi using BBP Formula.
    // https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula
    pub fn pi(&mut self, hex_block: &mut Account<HexBlock>, number_of_hex: u8) {
        match self.step {
            Step::X1Left => {
                msg!("X1LEFT");
                let sum = self.left_sum(1);
                if sum.1 {
                    self.x += 4.0 * sum.0;
                    self.next_step();
                }
            }
            Step::X1Right => {
                msg!("X1RIGHT");
                let sum = self.right_sum(1);
                self.x += 4.0 * sum;
                self.next_step();
            }
            Step::X2Left => {
                msg!("X2LEFT");
                let sum = self.left_sum(4);
                if sum.1 {
                    self.x -= 2.0 * sum.0;
                    self.next_step();
                }
            }
            Step::X2Right => {
                msg!("X2RIGHT");
                let sum = self.right_sum(4);
                self.x -= 2.0 * sum;
                self.next_step();
            }
            Step::X3Left => {
                msg!("X3LEFT");
                let sum = self.left_sum(5);
                if sum.1 {
                    self.x -= sum.0;
                    self.next_step();
                }
            }
            Step::X3Right => {
                msg!("X3RIGHT");
                let sum = self.right_sum(5);
                self.x -= sum;
                self.next_step();
            }
            Step::X4Left => {
                msg!("X4LEFT");
                let sum = self.left_sum(6);
                if sum.1 {
                    self.x -= sum.0;
                    self.next_step();
                }
            }
            Step::X4Right => {
                msg!("X4RIGHT");
                let sum = self.right_sum(6);
                self.x -= sum;
                self.next_step();
            }
            Step::Final => {
                msg!("FINAL");
                let x = self.x.rem_euclid(1.0);
                let bytes = &self
                    .remove_leading_zeros(((x * 16_f64.powi(14)) as u128).to_be_bytes().to_vec());

                hex_block.extend_hex(bytes, number_of_hex, self.current_pi_iteration);
                self.increment_current_pi_iteration(number_of_hex);
                self.update_current_hex_block(hex_block.hex.len());
                self.next_step();
                self.reset();
            }
        }
    }

    fn get_multipliers(&self) -> (f64, u64) {
        match self.step {
            Step::X1Left | Step::X1Right => return (4.0, 1),
            Step::X2Left | Step::X2Right => return (-2.0, 4),
            Step::X3Left | Step::X3Right => return (-1.0, 5),
            Step::X4Left | Step::X4Right => return (-1.0, 6),
            Step::Final => return (0.0, 0),
        }
    }

    fn update_current_hex_block(&mut self, current_hex_block_len: usize) {
        if current_hex_block_len >= MAX_PER_BLOCK {
            self.current_hex_block += 1;
        }
    }

    fn increment_current_pi_iteration(&mut self, number_of_hex: u8) {
        self.current_pi_iteration += number_of_hex as u64;
    }

    fn next_step(&mut self) {
        match self.step {
            Step::X1Left => self.step = Step::X1Right,
            Step::X1Right => self.step = Step::X2Left,
            Step::X2Left => self.step = Step::X2Right,
            Step::X2Right => self.step = Step::X3Left,
            Step::X3Left => self.step = Step::X3Right,
            Step::X3Right => self.step = Step::X4Left,
            Step::X4Left => self.step = Step::X4Right,
            Step::X4Right => self.step = Step::Final,
            Step::Final => self.step = Step::X1Left,
        }
    }

    fn remove_leading_zeros(&mut self, numbers: Vec<u8>) -> Vec<u8> {
        for i in 0..numbers.len() {
            if numbers[i] != 0 {
                return numbers[i..].to_vec();
            }
        }
        return vec![];
    }

    fn reset(&mut self) {
        self.s = 0.0;
        self.k = 0;
        self.r = 0;
        self.x = 0.0;
    }

    pub fn set_minted(&mut self) {
        self.minted = true;
    }

    pub fn pow_mod(&self, n: u64, m: u64, d: u64) -> u64 {
        if n < 100 && d < 400_000_000 {
            self.pow_mod_inner(n, m, d)
        } else {
            self.pow_mod_inner(n as u128, m as u128, d as u128) as u64
        }
    }

    fn pow_mod_inner<T>(&self, n: T, m: T, d: T) -> T
    where
        T: Copy
            + std::cmp::PartialEq
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Rem<Output = T>
            + std::convert::From<u64>,
    {
        if m == 0.into() {
            if d == 1.into() {
                0.into()
            } else {
                1.into()
            }
        } else if m == 1.into() {
            n % d
        } else {
            let k = self.pow_mod_inner(n, m / 2.into(), d);
            if m % 2.into() == 0.into() {
                (k * k) % d
            } else {
                (k * k * n) % d
            }
        }
    }
}

pub trait PiAccount {
    fn new(&mut self, id: u64, bump: u8) -> Result<()>;
}

impl PiAccount for Account<'_, Pi> {
    fn new(&mut self, id: u64, bump: u8) -> Result<()> {
        self.id = id;
        self.current_pi_iteration = 0;
        self.current_hex_block = 0;
        self.minted = false;
        self.s = 0.0;
        self.k = 0;
        self.r = 0;
        self.x = 0.0;
        self.bump = bump;
        Ok(())
    }
}
