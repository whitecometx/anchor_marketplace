use anchor_lang::prelude::*;

declare_id!("2XD5GJoQ9JJ3n92XVjdpTeCd7TmBDx26TjDp8vUBPn3S");

mod context;
use context::*;
mod state;
use state::*;
mod errors;
use errors::*;

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(mut ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)?;
        Ok(())
    }
    pub fn listing(ctx: Context<List>) -> Result<()> {
        Ok(())
    }
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        Ok(())
    }
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        Ok(())
    }
}

