use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};

pub mod state;
pub mod instructions;
pub mod error;
pub mod events;

use instructions::*;
use state::*;
use error::*;
use events::*;

declare_id!("SSS1111111111111111111111111111111111111111");

#[program]
pub mod sss_one {
    use super::*;

    // ========================================================================
    // INITIALIZATION
    // ========================================================================
    /// Initialize a new SSS-1 minimal stablecoin
    pub fn initialize(
        ctx: Context<Initialize>,
        name: String,
        symbol: String,
        uri: String,
        decimals: u8,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.name = name.clone();
        config.symbol = symbol.clone();
        config.uri = uri.clone();
        config.decimals = decimals;
        config.mint = ctx.accounts.mint.key();
        config.authority = ctx.accounts.authority.key();
        config.minter = ctx.accounts.authority.key();
        config.burner = ctx.accounts.authority.key();
        config.pauser = ctx.accounts.authority.key();
        config.is_paused = false;
        config.total_supply = 0;
        config.initialized = true;
        
        // Initialize mint authority
        let cpi_accounts = anchor_spl::token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::initialize_mint(
            cpi_ctx,
            decimals,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()), // freeze authority
        )?;
        
        emit!(StablecoinInitialized {
            name,
            symbol: symbol.clone(),
            decimals,
            mint: ctx.accounts.mint.key(),
            authority: ctx.accounts.authority.key(),
        });
        
        msg!("SSS-1 Stablecoin initialized: {}", symbol);
        Ok(())
    }

    // ========================================================================
    // MINT OPERATIONS
    // ========================================================================
    /// Mint new stablecoins to recipient
    pub fn mint(
        ctx: Context<MintOperation>,
        amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        
        // Checks
        require!(!config.is_paused, SSSOneError::ProgramPaused);
        require!(config.initialized, SSSOneError::NotInitialized);
        require!(
            ctx.accounts.minter.key() == config.minter,
            SSSOneError::UnauthorizedMinter
        );
        
        // Mint tokens
        let cpi_accounts = anchor_spl::token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::mint_to(cpi_ctx, amount)?;
        
        // Update total supply
        let config = &mut ctx.accounts.config;
        config.total_supply = config.total_supply.checked_add(amount)
            .ok_or(SSSOneError::Overflow)?;
        
        emit!(TokensMinted {
            recipient: ctx.accounts.recipient.key(),
            amount,
            new_supply: config.total_supply,
        });
        
        msg!("Minted {} tokens to {}", amount, ctx.accounts.recipient.key());
        Ok(())
    }

    /// Set new minter
    pub fn set_minter(
        ctx: Context<AuthorityManagement>,
        new_minter: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.minter = new_minter;
        emit!(MinterUpdated { new_minter });
        Ok(())
    }

    // ========================================================================
    // BURN OPERATIONS
    // ========================================================================
    /// Burn stablecoins from user's account
    pub fn burn(
        ctx: Context<BurnOperation>,
        amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        
        require!(!config.is_paused, SSSOneError::ProgramPaused);
        require!(config.initialized, SSSOneError::NotInitialized);
        require!(
            ctx.accounts.burner.key() == config.burner,
            SSSOneError::UnauthorizedBurner
        );
        
        // Burn tokens
        let cpi_accounts = anchor_spl::token::Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.burner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::burn(cpi_ctx, amount)?;
        
        // Update total supply
        let config = &mut ctx.accounts.config;
        config.total_supply = config.total_supply.checked_sub(amount)
            .ok_or(SSSOneError::Underflow)?;
        
        emit!(TokensBurned {
            from: ctx.accounts.token_account.key(),
            amount,
            new_supply: config.total_supply,
        });
        
        msg!("Burned {} tokens", amount);
        Ok(())
    }

    /// Set new burner
    pub fn set_burner(
        ctx: Context<AuthorityManagement>,
        new_burner: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.burner = new_burner;
        emit!(BurnerUpdated { new_burner });
        Ok(())
    }

    // ========================================================================
    // FREEZE/THAW OPERATIONS
    // ========================================================================
    /// Freeze a token account
    pub fn freeze_account(
        ctx: Context<FreezeThaw>,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(!config.is_paused, SSSOneError::ProgramPaused);
        
        let cpi_accounts = anchor_spl::token::FreezeAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.freezer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::freeze_account(cpi_ctx)?;
        
        emit!(AccountFrozen {
            account: ctx.accounts.token_account.key(),
        });
        
        msg!("Account frozen: {}", ctx.accounts.token_account.key());
        Ok(())
    }

    /// Thaw (unfreeze) a token account
    pub fn thaw_account(
        ctx: Context<FreezeThaw>,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        require!(!config.is_paused, SSSOneError::ProgramPaused);
        
        let cpi_accounts = anchor_spl::token::ThawAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.freezer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        anchor_spl::token::thaw_account(cpi_ctx)?;
        
        emit!(AccountThawed {
            account: ctx.accounts.token_account.key(),
        });
        
        msg!("Account thawed: {}", ctx.accounts.token_account.key());
        Ok(())
    }

    // ========================================================================
    // PAUSE/UNPAUSE
    // ========================================================================
    /// Pause all operations
    pub fn pause(ctx: Context<PauseManagement>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(!config.is_paused, SSSOneError::AlreadyPaused);
        config.is_paused = true;
        emit!(ProgramPaused { authority: ctx.accounts.pauser.key() });
        msg!("Program paused by {}", ctx.accounts.pauser.key());
        Ok(())
    }

    /// Unpause operations
    pub fn unpause(ctx: Context<PauseManagement>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        require!(config.is_paused, SSSOneError::NotPaused);
        config.is_paused = false;
        emit!(ProgramUnpaused { authority: ctx.accounts.pauser.key() });
        msg!("Program unpaused by {}", ctx.accounts.pauser.key());
        Ok(())
    }

    // ========================================================================
    // AUTHORITY MANAGEMENT
    // ========================================================================
    /// Transfer master authority
    pub fn transfer_authority(
        ctx: Context<AuthorityManagement>,
        new_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.authority = new_authority;
        emit!(AuthorityTransferred { new_authority });
        msg!("Authority transferred to {}", new_authority);
        Ok(())
    }

    /// Set pauser
    pub fn set_pauser(
        ctx: Context<AuthorityManagement>,
        new_pauser: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.pauser = new_pauser;
        emit!(PauserUpdated { new_pauser });
        Ok(())
    }
}