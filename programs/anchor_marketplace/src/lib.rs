use anchor_lang::prelude::*;

declare_id!("2XD5GJoQ9JJ3n92XVjdpTeCd7TmBDx26TjDp8vUBPn3S");

pub mod context;
use context::*;
pub mod state;
pub mod errors;
use errors::*;

#[program]
pub mod anchor_marketplace {
    use super::*;
    pub fn initialize(mut ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)?;
        Ok(())
    }
    pub fn list(ctx: Context<List>, price: u64 ) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps);
        ctx.accounts.deposit_nft()?;
        Ok(())
    }
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        Ok(())
    }
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol();
        ctx.accounts.send_nft();
        ctx.accounts.close_mint_vault();

        Ok(())
    }
}

