use anchor_lang::prelude::*;
use bigdecimal::BigDecimal;
use std::str::FromStr;

declare_id!("2MEDMx74giXqXYkcfmS8PJQTELbTriUh9aXqJxKnaGwT");

fn multiply_consecutive(n: BigDecimal) -> BigDecimal {
    (n.clone() + BigDecimal::from_str("1.0").unwrap())
        * (n.clone() + BigDecimal::from_str("2.0").unwrap())
        * (n.clone() + BigDecimal::from_str("3.0").unwrap())
}

// We're gonna calculate Pi using the Nilakantha series.
// According to the Nilakantha series
// π = 3 + 4/(2*3*4) - 4/(4*5*6) + 4/(6*7*8) - 4/(8*9*10) + 4/(10*11*12) - 4/(12*13*14)...

fn pi(x: i64) -> BigDecimal {
    let big_two = BigDecimal::from_str("2.0").unwrap();
    let big_four = BigDecimal::from_str("4.0").unwrap();

    let mut output = BigDecimal::from_str("3.0").unwrap();
    let mut n = BigDecimal::from_str("1.0").unwrap();
    let mut plus = true;

    for _ in 0..x {
        if plus {
            output = output + (big_four.clone() / multiply_consecutive(n.clone()));
        } else {
            output = output - (big_four.clone() / multiply_consecutive(n.clone()));
        }

        plus = !plus;
        n += big_two.clone();
    }

    output
}

#[program]
pub mod pisolana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let pi_c = pi(1);
        msg!("{}", pi_c);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,
}
