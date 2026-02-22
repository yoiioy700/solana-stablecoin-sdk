# CHAPTER 6: Solana/Anchor Basics

## Understanding Anchor Framework

### 1. Anchor Project Structure
```
anchor-basics/
├── Anchor.toml          # Project config (program ID, clusters)
├── Cargo.toml           # Rust workspace config
├── programs/
│   └── anchor-basics/   # Your program
│       └── src/lib.rs   # Main program code
├── tests/               # TypeScript tests (or Rust tests)
└── target/              # Build artifacts
```

### 2. Core Anchor Concepts

#### Program Macro
`#[program]` - Marks the module containing instruction handlers

#### Accounts Macro  
`#[derive(Accounts)]` - Defines account validation structure

#### derive_id! Macro
Generates the program's public key

### 3. Account Types

| Type | Purpose | Example |
|------|---------|---------|
| `Account<T>` | Account with data of type T | `Account<User>` |
| `Signer` | Transaction signer | `Signer` |
| `SystemAccount` | System program account | `SystemAccount` |
| `UncheckedAccount` | Raw account (careful!) | `UncheckedAccount` |
| `Program` | Other programs | `Program<Token>` |

### 4. Program Derived Addresses (PDAs)
PDAs are accounts owned by the program, derived from seeds.
```rust
let (pda, bump) = Pubkey::find_program_address(
    &[b"seed", user.key().as_ref()],
    program_id
);
```

### 5. Cross Program Invocation (CPI)
Calling other programs from your program:
```rust
token::transfer(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        },
    ),
    amount,
)?;
```

### 6. Error Handling in Anchor
```rust
#[error_code]
pub enum MyError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid account")]
    InvalidAccount,
}

// Usage
require!(balance >= amount, MyError::InsufficientFunds);
```

## Solana Stablecoin Standard Requirements

From the bounty:
- SDK for building stablecoins on Solana
- Peg mechanisms
- Collateral management
- Oracle integration
- Transfer functionality
- Mint/Burn capabilities

### Key Components Needed:
1. **Token Account Management** - SPL Token accounts
2. **Oracle Integration** - Pyth/Switchboard for price feeds
3. **Collaterization** - Tracking backing assets
4. **Mint/Burn Logic** - Based on collateral ratio
5. **Access Control** - Admin, freeze/unfreeze
