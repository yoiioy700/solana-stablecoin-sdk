use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::state::*;

// ========================================================================
// INITIALIZATION
// ========================================================================
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + StablecoinConfig::SIZE,
    )]
    pub config: Account<'info, StablecoinConfig>,
    
    #[account(
        init,
        payer = authority,
        mint::decimals = 0, // Will be set in initialize
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    
    /// CHECK: This is the mint authority PDA
    #[account(
        seeds = [b"mint_authority", config.key().as_ref()],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// ========================================================================
// MINT OPERATIONS
// ========================================================================
#[derive(Accounts)]
pub struct MintOperation<'info> {
    #[account(mut)]
    pub config: Account<'info, StablecoinConfig>,
    
    #[account(
        mut,
        constraint = mint.key() == config.mint,
    )]
    pub mint: Account<'info, Mint>,
    
    /// CHECK: Mint authority PDA
    #[account(
        seeds = [b"mint_authority", config.key().as_ref()],
        bump,
    )]
    pub mint_authority: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Recipient owner
    pub recipient: UncheckedAccount<'info>,
    
    pub minter: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

// ========================================================================
// BURN OPERATIONS
// ========================================================================
#[derive(Accounts)]
pub struct BurnOperation<'info> {
    #[account(mut)]
    pub config: Account<'info, StablecoinConfig>,
    
    #[account(
        mut,
        constraint = mint.key() == config.mint,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = token_account.mint == config.mint,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    pub burner: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

// ========================================================================
// FREEZE/THAW
// ========================================================================
#[derive(Accounts)]
pub struct FreezeThaw<'info> {
    pub config: Account<'info, StablecoinConfig>,
    
    #[account(
        constraint = mint.key() == config.mint,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = token_account.mint == config.mint,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Freezer (must be freeze authority)
    pub freezer: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

// ========================================================================
// PAUSE MANAGEMENT
// ========================================================================
#[derive(Accounts)]
pub struct PauseManagement<'info> {
    #[account(mut)]
    pub config: Account<'info, StablecoinConfig>,
    
    #[account(
        constraint = pauser.key() == config.pauser,
    )]
    pub pauser: Signer<'info>,
}

// ========================================================================
// AUTHORITY MANAGEMENT
// ========================================================================
#[derive(Accounts)]
pub struct AuthorityManagement<'info> {
    #[account(mut)]
    pub config: Account<'info, StablecoinConfig>,
    
    #[account(
        constraint = authority.key() == config.authority,
    )]
    pub authority: Signer<'info>,
}
