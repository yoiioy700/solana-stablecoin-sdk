# Solana Stablecoin Standard (SSS) SDK

## Submission for Superteam Brazil Bounty

**Bounty:** Build the Solana Stablecoin Standard  
**Deadline:** March 24, 2026  
**Repository:** github.com/solanabr/solana-stablecoin-standard

---

## Submission Structure

```
submission/
├── Anchor.toml                    # Root Anchor config
├── Cargo.toml                     # Workspace config
├── README.md                      # This file
├── programs/                      # On-chain programs
│   ├── sss-1/                    # Minimal Stablecoin Program
│   │   ├── src/
│   │   │   ├── lib.rs            # Main program
│   │   │   ├── state.rs          # Account structs
│   │   │   ├── instructions.rs  # Account validations
│   │   │   ├── error.rs          # Error codes
│   │   │   └── events.rs         # Event definitions
│   │   └── Cargo.toml
│   └── sss-2-modules/            # Compliance Module
│       └── transfer-hook/        # Transfer hook program
├── sdk/                          # TypeScript SDK
│   └── core/                     # Core SDK
│       ├── src/
│       │   ├── index.ts
│       │   ├── client.ts         # SolanaStablecoin class
│       │   └── presets.ts        # SSS-1, SSS-2 presets
│       └── package.json
├── backend/                      # Backend services
│   ├── mint-burn-service/       # Fiat-to-stablecoin lifecycle
│   ├── event-indexer/           # On-chain event monitoring
│   └── compliance-service/      # Blacklist management (SSS-2)
└── cli/                          # Admin CLI
    └── admin/                    # sss-token CLI
        ├── src/
        └── package.json
```

---

## Standards

### SSS-1: Minimal Stablecoin
Basic stablecoin with essential features:
- ✅ Initialize config
- ✅ Mint
- ✅ Burn
- ✅ Freeze/Thaw
- ✅ Pause/Unpause
- ✅ Role management (Master, Minter, Burner, Pauser)

### SSS-2: Compliant Stablecoin
SSS-1 + Compliance features:
- 🚧 Permanent delegate
- 🚧 Transfer hook with blacklist enforcement
- 🚧 Add/Remove blacklist
- 🚧 Seize tokens

---

## Quick Start

### Build
```bash
cd submission
anchor build
```

### Test
```bash
anchor test
```

### Deploy
```bash
anchor deploy --provider.cluster devnet
```

---

## Progress

- ✅ SSS-1: Program structure complete
- 🚧 SSS-1: Tests
- 🚧 SSS-1: Devnet deployment
- 🚧 SSS-2: Transfer hook
- 🚧 SDK: Core functionality
- 🚧 Backend: Services
- 🚧 CLI: Operations

**Days remaining: 17**
