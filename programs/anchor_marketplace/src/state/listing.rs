use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub maker: pubkey,
    pub mint: pubkey, 
    pub price: u64,
    pub bump: u8,
}
impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1 + 8; // 32 bytes- pubkeys, 8 bytes- u64 & 8 bytes- de
}