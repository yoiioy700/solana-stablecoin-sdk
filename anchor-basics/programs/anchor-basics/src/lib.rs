use anchor_lang::prelude::*;

declare_id!("EFBeD3HwSuYeGhbCKDvuCEEtK5ggVxnKbEZ5xsciTzUR");

// ============================================================================
// CHAPTER 6: Solana/Anchor Basics
// Learning the Anchor framework for the Stablecoin SDK
// ============================================================================

#[program]
pub mod anchor_basics {
    use super::*;

    // ========================================================================
    // SECTION 1: Basic Instruction - Initialize
    // ========================================================================
    /// Initialize a simple counter account
    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.user.key();
        
        msg!("Counter initialized! Authority: {}", counter.authority);
        msg!("Program ID: {:?}", ctx.program_id);
        
        Ok(())
    }

    // ========================================================================
    // SECTION 2: Reading and Updating Account Data
    // ========================================================================
    /// Increment the counter
    pub fn increment(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        // Verify authority
        require!(
            counter.authority == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        
        counter.count += 1;
        msg!("Count incremented to: {}", counter.count);
        
        Ok(())
    }

    /// Decrement the counter
    pub fn decrement(ctx: Context<UpdateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        
        require!(
            counter.authority == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        
        // Check for underflow
        require!(counter.count > 0, ErrorCode::Underflow);
        
        counter.count -= 1;
        msg!("Count decremented to: {}", counter.count);
        
        Ok(())
    }

    // ========================================================================
    // SECTION 3: Program Derived Addresses (PDAs)
    // ========================================================================
    /// Initialize a PDA account
    pub fn initialize_pda(ctx: Context<InitializePda>, bump: u8) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.owner = ctx.accounts.user.key();
        user_account.bump = bump;
        user_account.balance = 0;
        
        msg!("PDA initialized for: {}", user_account.owner);
        msg!("PDA address: {}", ctx.accounts.user_account.key());
        msg!("Bump seed: {}", bump);
        
        Ok(())
    }

    /// Deposit lamports to PDA
    pub fn deposit(ctx: Context<DepositWithdraw>, amount: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        
        require!(
            user_account.owner == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );
        
        // Transfer lamports from user to PDA
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.user_account.key(),
            amount,
        );
        
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.user_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        
        user_account.balance += amount;
        msg!("Deposited {} lamports. New balance: {}", amount, user_account.balance);
        
        Ok(())
    }

    // ========================================================================
    // SECTION 4: Token Operations (SPL Tokens)
    // ========================================================================
    /// Initialize a token vault for a user
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = ctx.accounts.user.key();
        vault.token_account = ctx.accounts.user_token_account.key();
        vault.amount = 0;
        
        msg!("Vault initialized for: {}", vault.owner);
        msg!("Associated token account: {}", vault.token_account);
        
        Ok(())
    }

    /// Deposit tokens to vault
    pub fn deposit_tokens(ctx: Context<TokenOperations>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        // Transfer tokens from user to vault
        let cpi_accounts = anchor_spl::token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        anchor_spl::token::transfer(cpi_ctx, amount)?;
        
        vault.amount += amount;
        msg!("Deposited {} tokens. Total: {}", amount, vault.amount);
        
        Ok(())
    }

    // ========================================================================
    // SECTION 5: Stablecoin-Specific Patterns
    // ========================================================================
    /// Initialize stablecoin mint configuration
    pub fn initialize_stablecoin_config(
        ctx: Context<InitializeStablecoinConfig>,
        collateral_ratio: u16,      // e.g., 150 = 150%
        liquidation_threshold: u16,  // e.g., 120 = 120%
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.stablecoin_mint = ctx.accounts.stablecoin_mint.key();
        config.collateral_mint = ctx.accounts.collateral_mint.key();
        config.collateral_ratio = collateral_ratio;
        config.liquidation_threshold = liquidation_threshold;
        config.total_supply = 0;
        config.total_collateral = 0;
        config.is_active = true;
        
        msg!("Stablecoin config initialized:");
        msg!("  Authority: {}", config.authority);
        msg!("  Collateral ratio: {}%", collateral_ratio);
        msg!("  Liquidation threshold: {}%", liquidation_threshold);
        
        Ok(())
    }

    /// User opens a collateral position
    pub fn open_position(
        ctx: Context<OpenPosition>,
        collateral_amount: u64,
        mint_amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let position = &mut ctx.accounts.position;
        
        require!(config.is_active, ErrorCode::ProgramPaused);
        require!(mint_amount > 0, ErrorCode::InvalidAmount);
        
        // Store position data
        position.owner = ctx.accounts.user.key();
        position.collateral_amount = collateral_amount;
        position.minted_amount = mint_amount;
        position.created_at = Clock::get()?.unix_timestamp;
        position.is_liquidated = false;
        
        // Update global state
        let config = &mut ctx.accounts.config;
        config.total_supply += mint_amount;
        config.total_collateral += collateral_amount;
        
        msg!("Position opened:");
        msg!("  Owner: {}", position.owner);
        msg!("  Collateral: {}", collateral_amount);
        msg!("  Minted: {}", mint_amount);
        msg!("  Collateral ratio target: {}%", config.collateral_ratio);
        
        Ok(())
    }
}

// ============================================================================
// ACCOUNT STRUCTURES (Validation & Constraints)
// ============================================================================

/// Account for initializing a counter
#[derive(Accounts)]
pub struct InitializeCounter<'a, 'b> {
    #[account(
        init,
        payer = user,
        space = 8 + Counter::SIZE,  // 8 for discriminator
    )]
    pub counter: Account<'a, Counter>,
    
    #[account(mut)]
    pub user: Signer<'b>,
    
    pub system_program: Program<'b, System>,
}

/// Account for updating counter (increment/decrement)
#[derive(Accounts)]
pub struct UpdateCounter<'a, 'b> {
    #[account(mut)]
    pub counter: Account<'a, Counter>,
    
    pub user: Signer<'b>,
}

/// Account for initializing a PDA
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializePda<'a, 'b> {
    #[account(
        init,
        payer = user,
        space = 8 + UserAccount::SIZE,
        seeds = [b"user_account", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'a, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'b>,
    
    pub system_program: Program<'b, System>,
}

/// Account for deposit/withdraw operations
#[derive(Accounts)]
pub struct DepositWithdraw<'a, 'b> {
    #[account(
        mut,
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'a, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'b>,
    
    pub system_program: Program<'b, System>,
}

/// Account for initializing token vault
#[derive(Accounts)]
pub struct InitializeVault<'a, 'b> {
    #[account(
        init,
        payer = user,
        space = 8 + Vault::SIZE,
    )]
    pub vault: Account<'a, Vault>,
    
    /// User's associated token account
    pub user_token_account: Account<'b, anchor_spl::token::TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'b>,
    
    pub system_program: Program<'b, System>,
}

/// Account for token operations
#[derive(Accounts)]
pub struct TokenOperations<'a, 'b> {
    #[account(mut)]
    pub vault: Account<'a, Vault>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key()
    )]
    pub user_token_account: Account<'b, anchor_spl::token::TokenAccount>,
    
    #[account(mut)]
    pub vault_token_account: Account<'b, anchor_spl::token::TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'b>,
    
    pub token_program: Program<'b, anchor_spl::token::Token>,
}

/// Account for stablecoin config initialization
#[derive(Accounts)]
pub struct InitializeStablecoinConfig<'a, 'b> {
    #[account(
        init,
        payer = authority,
        space = 8 + StablecoinConfig::SIZE,
    )]
    pub config: Account<'a, StablecoinConfig>,
    
    /// The stablecoin mint
    pub stablecoin_mint: Account<'b, anchor_spl::token::Mint>,
    
    /// The collateral token mint
    pub collateral_mint: Account<'b, anchor_spl::token::Mint>,
    
    #[account(mut)]
    pub authority: Signer<'b>,
    
    pub system_program: Program<'b, System>,
}

/// Account for opening a collateral position
#[derive(Accounts)]
pub struct OpenPosition<'a, 'b> {
    #[account(mut)]
    pub config: Account<'a, StablecoinConfig>,
    
    #[account(
        init,
        payer = user,
        space = 8 + Position::SIZE,
        seeds = [b"position", config.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub position: Account<'a, Position>,
    
    #[account(mut)]
    pub user: Signer<'b>,
    
    pub system_program: Program<'b, System>,
}

// ============================================================================
// ACCOUNT DATA STRUCTURES
// ============================================================================

/// Simple counter account
#[account]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
}

impl Counter {
    pub const SIZE: usize = 8 + 32;  // u64 + Pubkey
}

/// PDA account for storing user data
#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub bump: u8,
    pub balance: u64,
}

impl UserAccount {
    pub const SIZE: usize = 32 + 1 + 8;  // Pubkey + u8 + u64
}

/// Token vault account
#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub amount: u64,
}

impl Vault {
    pub const SIZE: usize = 32 + 32 + 8;  // Pubkey + Pubkey + u64
}

/// Stablecoin configuration
#[account]
pub struct StablecoinConfig {
    pub authority: Pubkey,
    pub stablecoin_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub collateral_ratio: u16,        // e.g., 150 = 150%
    pub liquidation_threshold: u16, // e.g., 120 = 120%
    pub total_supply: u64,
    pub total_collateral: u64,
    pub is_active: bool,
}

impl StablecoinConfig {
    pub const SIZE: usize = 32 + 32 + 32 + 2 + 2 + 8 + 8 + 1;
}

/// User's collateral position
#[account]
pub struct Position {
    pub owner: Pubkey,
    pub collateral_amount: u64,
    pub minted_amount: u64,
    pub created_at: i64,
    pub is_liquidated: bool,
}

impl Position {
    pub const SIZE: usize = 32 + 8 + 8 + 8 + 1;
}

// ============================================================================
// ERROR CODES
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Insufficient funds")]
    InsufficientFunds,
    
    #[msg("Arithmetic overflow")]
    Overflow,
    
    #[msg("Arithmetic underflow")]
    Underflow,
    
    #[msg("Position would be undercollateralized")]
    Undercollateralized,
    
    #[msg("Program is paused")]
    ProgramPaused,
    
    #[msg("Position is already liquidated")]
    AlreadyLiquidated,
    
    #[msg("Invalid amount")]
    InvalidAmount,
}