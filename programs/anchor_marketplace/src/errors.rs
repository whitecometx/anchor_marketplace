use anchor_lang::error_code;

#[error_code]
    pub enum MarketplaceError {
        #[msg("Name too long")]
        NameTooLong,
    }