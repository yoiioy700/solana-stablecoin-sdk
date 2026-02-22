# Solana Stablecoin Standard (SSS) SDK

A modular stablecoin SDK with standardized presets for Solana. Built for Superteam Brazil Bounty.

**Deadline:** March 24, 2026 (17 days remaining)  
**Bounty:** github.com/solanabr/solana-stablecoin-standard  
**Reference:** github.com/solanabr/solana-vault-standard

**⚠️ ORGANIZED INTO TWO SEPARATE FOLDERS:**

---

## 📁 Folder Structure

### `/submission/` - **ACTUAL BOUNTY SUBMISSION** 🎯
This is the production code for the bounty.
```
submission/
├── programs/          # On-chain programs
│   ├── sss-1/        # Main submission
│   └── sss-2-modules/# Compliance features
├── sdk/              # TypeScript SDK
├── backend/          # Backend services
├── cli/              # Admin CLI
└── tests/            # Integration tests
```

### `/rust-learning/` - **LEARNING MATERIALS** 📚
Chapter-by-chapter Rust learning exercises.
- Chapter 1: Variables & Types
- Chapter 2: Ownership & Borrowing
- Chapter 3: Structs, Enums, Pattern Matching
- Chapter 4: Collections & Error Handling

### `/anchor-basics/` - **ANCHOR LEARNING** ⚓
Chapter 6: Solana/Anchor basics.

---

## Quick Start (Submission)

```bash
cd submission

# Build
anchor build

# Test  
anchor test

# Deploy
anchor deploy --provider.cluster devnet
```

---

## Progress

### Week 1: Foundation (Feb 23 - Mar 1)
- [x] Day 4: SSS-1 program structure
- [ ] Day 5: SSS-1 tests + Devnet deploy
- [ ] Day 6: SSS-2 transfer hook
- [ ] Day 7: SSS-2 blacklist
- [ ] Day 8: Role management
- [ ] Day 9: Events + Indexing
- [ ] Day 10: Core SDK

### Week 2: Backend + Integration (Mar 2 - Mar 9)
- [ ] Mint-burn service
- [ ] Event indexer
- [ ] Compliance service
- [ ] Admin CLI
- [ ] Testing

### Week 3: Polish + Submit (Mar 10 - Mar 20)
- [ ] Documentation
- [ ] Final testing
- [ ] Submit PR

---

## Key Documentation

- [Submission README](submission/README.md) - Production code
- [Bounty Requirements](BOUNTY.md) - Full requirements
- [Architecture](ARCHITECTURE.md) - Design decisions

---

**Built with 🦀 Rust + ⚓ Anchor for Solana**
