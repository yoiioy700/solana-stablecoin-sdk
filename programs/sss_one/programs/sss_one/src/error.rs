use anchor_lang::prelude::*;

#[error_code]
pub enum SSSOneError {
    #[msg("Program is not initialized")]
    NotInitialized,
    
    #[msg("Program is already initialized")]
    AlreadyInitialized,
    
    #[msg("Program is paused")]
    ProgramPaused,
    
    #[msg("Program is already paused")]
    AlreadyPaused,
    
    #[msg("Program is not paused")]
    NotPaused,
    
    #[msg("Unauthorized: Not a valid authority")]
    UnauthorizedAuthority,
    
    #[msg("Unauthorized: Not a valid minter")]
    UnauthorizedMinter,
    
    #[msg("Unauthorized: Not a valid burner")]
    UnauthorizedBurner,
    
    #[msg("Unauthorized: Not a valid pauser")]
    UnauthorizedPauser,
    
    #[msg("Account is frozen")]
    AccountFrozen,
    
    #[msg("Account is not frozen")]
    AccountNotFrozen,
    
    #[msg("Invalid amount")]
    InvalidAmount,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Math overflow")]
    Overflow,
    
    #[msg("Math underflow")]
    Underflow,
    
    #[msg("Invalid metadata")]
    InvalidMetadata,
    
    #[msg("Token name too long")]
    NameTooLong,
    
    #[msg("Token symbol too long")]
    SymbolTooLong,
    
    #[msg("Token URI too long")]
    URITooLong,
}
