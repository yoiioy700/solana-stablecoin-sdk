# Solana Stablecoin Standard SDK - Project Log

## Overview
Building a Solana Stablecoin Standard SDK for Superteam Brazil Bounty ($5,000 prize pool)

**Deadline:** March 24, 2026 (19 days remaining)  
**Bounty URL:** https://superteam.fun/earn/listing/build-the-solana-stablecoin-standard-bounty

## Learning Plan

### Phase 1: Rust Basics (Days 1-2)
- [ ] Day 1: Variables, ownership, structs, basic syntax
- [ ] Day 2: Enums, pattern matching, collections, error handling

### Phase 2: Solana/Anchor (Days 3-5)
- [ ] Day 3: Accounts, PDAs, program structure
- [ ] Day 4: CPIs, testing framework
- [ ] Day 5: Hello World deployment

### Phase 3: Stablecoin Implementation (Days 6-15)
- [ ] Core stablecoin mechanics
- [ ] Peg mechanisms
- [ ] Oracle integration
- [ ] Collateral models

### Phase 4: Polish & Submit (Days 16-19)
- [ ] Testing & audit prep
- [ ] Documentation
- [ ] Submission

## Daily Log

### Day 0 - Feb 22, 2026
**Time:** 14:50 - 15:30 (Europe/Berlin)  
**Focus:** Environment setup

#### Completed
- [x] Rust installed (v1.93.1)
- [x] Cargo installed (v1.93.1)
- [x] Solana CLI installed (v3.0.15)
- [x] Anchor CLI installed (v0.30.1) via AVM (v0.32.1)

#### Commands Used
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
```

#### Next Steps
Start Day 1: Rust Book Chapters 1-4

---

**Current Status:** Environment ready ✅  
**Days Remaining:** 19
