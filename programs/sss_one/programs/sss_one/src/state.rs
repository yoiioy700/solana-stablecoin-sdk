use anchor_lang::prelude::*;

/// Configuration account for the SSS-1 stablecoin
#[account]
pub struct StablecoinConfig {
    /// Token metadata
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
    
    /// Token mint address
    pub mint: Pubkey,
    
    /// Authority roles
    pub authority: Pubkey,
    pub minter: Pubkey,
    pub burner: Pubkey,
    pub pauser: Pubkey,
    
    /// State
    pub is_paused: bool,
    pub initialized: bool,
    
    /// Total supply tracking
    pub total_supply: u64,
}

impl StablecoinConfig {
    pub const SIZE: usize = 
        4 + 32 +    // name (String with max 32 chars)
        4 + 10 +    // symbol (String with max 10 chars)
        4 + 200 +   // uri (String with max 200 chars)
        1 +         // decimals
        32 +        // mint
        32 * 4 +    // authority, minter, burner, pauser
        1 +         // is_paused
        1 +         // initialized
        8;          // total_supply
}
