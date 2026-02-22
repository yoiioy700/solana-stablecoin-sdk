# Solana Stablecoin Standard (SSS) SDK

A modular stablecoin SDK with standardized presets for Solana. Built for Superteam Brazil Bounty.

**Deadline:** March 24, 2026 (17 days remaining)  
**Repository:** github.com/yoiioy700/solana-stablecoin-standard  
**Reference:** github.com/solanabr/solana-vault-standard

---

## Architecture

### Three Layers

```
┌─────────────────────────────────────────────────────────────┐
│ Layer 3 - Standard Presets                                  │
│ ┌─────────────┐  ┌─────────────────┐                         │
│ │   SSS-1     │  │     SSS-2       │                         │
│ │  Minimal    │  │   Compliant     │                         │
│ │             │  │  + Blacklist    │                         │
│ │             │  │  + Seizure      │                         │
│ └─────────────┘  └─────────────────┘                         │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│ Layer 2 - Modules                                             │
│ ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       │
│ │ Compliance   │  │   Privacy    │  │   Oracle     │       │
│ │ - Transfer   │  │ - Confident. │  │ - Pyth       │       │
│ │   hook       │  │   transfers  │  │ - Switchboard│       │
│ │ - Blacklist  │  │ - Allowlists │  │              │       │
│ └──────────────┘  └──────────────┘  └──────────────┘       │
└─────────────────────────────────────────────────────────────┘
┌─────────────────────────────────────────────────────────────┐
│ Layer 1 - Base SDK                                            │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ Token Creation  │  Role Management  │  CLI + TS SDK      │ │
│ │ - Mint authority│  - Master auth    │  - Initialize      │ │
│ │ - Freeze auth   │  - Minter quota   │  - Mint/Burn       │ │
│ │ - Metadata      │  - Blacklister    │  - Freeze/Thaw     │ │
│ └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

---

## Standards

### SSS-1: Minimal Stablecoin
- Mint authority
- Freeze authority  
- Metadata
- Basic role management

### SSS-2: Compliant Stablecoin
- SSS-1 + Compliance Module
- Permanent delegate
- Transfer hook (blacklist check)
- Blacklist enforcement
- Seizure capability

---

## Folder Structure

```
solana-stablecoin-standard/
├── programs/
│   ├── sss-1/                    # Minimal stablecoin program
│   ├── sss-2-modules/           # Compliance module (transfer hook)
│   └── transfer-hook/           # Separate blacklist enforcement program
├── sdk/
│   ├── core/                    # Base SDK (TypeScript)
│   ├── compliance/              # Compliance module SDK
│   └── privacy/               # Privacy module SDK
├── backend/
│   ├── mint-burn-service/      # Fiat-to-stablecoin lifecycle
│   ├── event-indexer/          # On-chain event monitoring
│   └── compliance-service/     # Blacklist management (SSS-2)
├── cli/
│   ├── admin/                  # Admin operations CLI
│   └── deploy/                 # Deployment scripts
├── rust-learning/              # Learning materials (Ch 1-4)
├── anchor-basics/              # Chapter 6 learning
└── README.md                   # This file
```

---

## Progress Tracker

### Week 1 (Feb 23 - Mar 1): Foundation
- [ ] Day 1: SSS-1 program structure (initialize, mint, burn)
- [ ] Day 2: Role-based access control
- [ ] Day 3: Freeze/thaw, pause/unpause
- [ ] Day 4: Transfer hook for SSS-2
- [ ] Day 5: Blacklist PDA management
- [ ] Day 6: Seizure via permanent delegate
- [ ] Day 7: Core SDK TypeScript

### Week 2 (Mar 2 - Mar 9): Backend & Integration
- [ ] Day 8: Mint-burn service
- [ ] Day 9: Event indexer
- [ ] Day 10: Compliance service (SSS-2)
- [ ] Day 11: Admin CLI
- [ ] Day 12-13: Testing & integration
- [ ] Day 14: Documentation

### Week 3 (Mar 10 - Mar 20): Polish & Submit
- [ ] Day 15-17: Final testing, bug fixes
- [ ] Day 18: README, docs, examples
- [ ] Day 19: Submit

---

## Quick Start

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
```

### Build
```bash
cd programs/sss-1
anchor build
```

### Test
```bash
anchor test
```

---

## Key Features

### Role-Based Access Control
- Master authority (can transfer authority)
- Minter (with quotas)
- Burner
- Blacklister (SSS-2 only)
- Pauser
- Seizer (SSS-2 only)

### Compliance Module (SSS-2)
Every transfer is checked against blacklist PDA:
```rust
if blacklist.contains(from) || blacklist.contains(to) {
    return Err(ErrorCode::Blacklisted);
}
```

### Seizure Capability
Permanent delegate can seize tokens from any account:
```rust
pub fn seize(ctx: Context<Seize>, amount: u64) -> Result<()> {
    require!(config.enable_permanent_delegate, ErrorCode::NotSSS2);
    // Transfer from target to authority
}
```

---

## Resources

- [Solana Vault Standard Reference](https://github.com/solanabr/solana-vault-standard)
- [Anchor Lang Documentation](https://docs.rs/anchor-lang/latest/anchor_lang/)
- [SPL Token 2022 Extensions](https://solana.com/developers/guides/token-extensions)

---

**Built with ⚓ Anchor + 🦀 Rust for Solana Stablecoin Standard**
