use crate::*;

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
    #[inline]
    pub fn pda(id: u64) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_PI, &id.to_be_bytes()], &crate::ID)
    }

    pub fn sum(&self, j: u64, n: u64) -> f64 {
        let mut s = 0.0;
        let mut k = 0;
        let mut r: u64;
        while k <= n {
            r = 8 * k + j;
            s = (s + self.pow_mod(16, n - k, r) as f64 / r as f64) % 1.0;
            k += 1;
        }

        let mut t = 0.0;
        k = n + 1;
        let mut i = -1;
        let mut newt: f64;
        for _ in 1..10 {
            newt = t + 16_f64.powi(i) / (8 * k + j) as f64;
            if t == newt {
                break;
            } else {
                t = newt;
            }
            k += 1;
            i = i - 1;
        }
        return s + t;
    }

    pub fn pi(&mut self, n: u64) {
        let x = (4.0 * self.sum(1, n) - 2.0 * self.sum(4, n) - self.sum(5, n) - self.sum(6, n))
            .rem_euclid(1.0);
        self.res = format!("{:014x}", (x * 16_f64.powi(14)) as u128);
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
        self.n = 1;
        self.plus = true;
        self.id = id;
        self.res = String::from("3.0");
        self.bump = bump;
        Ok(())
    }
}
