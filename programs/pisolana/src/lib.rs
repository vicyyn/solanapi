use {anchor_lang::prelude::*, std::mem::size_of};

pub mod contexts;
pub mod errors;
pub mod states;

pub use contexts::*;
pub use errors::*;
pub use states::*;

declare_id!("HWXZbmAgYoumCyejC1raxNyLbiV3g7xqfuNfEWmyXCKP");

#[program]
pub mod pisolana {
    use super::*;

    pub fn initialize_pi(ctx: Context<InitializePi>, pi_id: u64, bump: u8) -> Result<()> {
        ctx.accounts.process(pi_id, bump)
    }

    pub fn initialize_hex_block(ctx: Context<InitializeHexBlock>, bump: u8) -> Result<()> {
        ctx.accounts.process(bump)
    }

    pub fn calculate_pi(ctx: Context<CalculatePi>, digits_to_add: u8) -> Result<()> {
        ctx.accounts.process(digits_to_add)
    }

    pub fn view_pi(ctx: Context<ViewPi>) -> Result<()> {
        ctx.accounts.process()
    }
}
