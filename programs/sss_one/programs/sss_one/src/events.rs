use anchor_lang::prelude::*;

// ========================================================================
// INITIALIZATION EVENTS
// ========================================================================
#[event]
pub struct StablecoinInitialized {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub mint: Pubkey,
    pub authority: Pubkey,
}

// ========================================================================
// MINT EVENTS
// ========================================================================
#[event]
pub struct TokensMinted {
    pub recipient: Pubkey,
    pub amount: u64,
    pub new_supply: u64,
}

#[event]
pub struct MinterUpdated {
    pub new_minter: Pubkey,
}

// ========================================================================
// BURN EVENTS
// ========================================================================
#[event]
pub struct TokensBurned {
    pub from: Pubkey,
    pub amount: u64,
    pub new_supply: u64,
}

#[event]
pub struct BurnerUpdated {
    pub new_burner: Pubkey,
}

// ========================================================================
// FREEZE/THAW EVENTS
// ========================================================================
#[event]
pub struct AccountFrozen {
    pub account: Pubkey,
}

#[event]
pub struct AccountThawed {
    pub account: Pubkey,
}

// ========================================================================
// PAUSE EVENTS
// ========================================================================
#[event]
pub struct ProgramPaused {
    pub authority: Pubkey,
}

#[event]
pub struct ProgramUnpaused {
    pub authority: Pubkey,
}

// ========================================================================
// AUTHORITY EVENTS
// ========================================================================
#[event]
pub struct AuthorityTransferred {
    pub new_authority: Pubkey,
}

#[event]
pub struct PauserUpdated {
    pub new_pauser: Pubkey,
}
