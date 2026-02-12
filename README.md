<p align="center">
  <img src="./assets/logo.svg" alt="CrazySol Logo" width="200" height="200" />
</p>

# 🧪 CrazySol — Solana On-Chain Economy Simulation Game

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Built with Anchor](https://img.shields.io/badge/Built%20with-Anchor%200.31-blueviolet)](https://www.anchor-lang.com/)
[![Solana](https://img.shields.io/badge/Solana-Mainnet-green)](https://solana.com)

**CrazySol** is a Solana smart contract (program) for a pyramid economy simulation game, originally hosted at [crazysol.io](https://crazysol.io) (now discontinued). Built with the **Anchor framework** in Rust, the program implements a gamified economic system where players ("Scientists") inject SOL into a shared "Reactor," earn "Pills" via a bonding curve, accumulate yield over time, and benefit from a multi-level referral system.

> **⚠️ Important:** This repository contains **only the on-chain Solana program**. The frontend dApp is **not included** and is not open-source.

---

## Table of Contents

- [Overview](#overview)
- [Game Concept](#game-concept)
- [Architecture](#architecture)
  - [On-Chain State Accounts](#on-chain-state-accounts)
  - [Instruction Set](#instruction-set)
  - [Utility Modules](#utility-modules)
- [Detailed Function Reference](#detailed-function-reference)
  - [Admin Instructions](#admin-instructions)
  - [Player Instructions](#player-instructions)
  - [View Instructions](#view-instructions)
- [Bonding Curve & Tokenomics](#bonding-curve--tokenomics)
- [Event System](#event-system)
- [Error Codes](#error-codes)
- [Project Structure](#project-structure)
- [Build & Deploy](#build--deploy)
- [License](#license)

---

## Overview

| Property | Value |
|---|---|
| **Program ID** | `8RRuAhgwCR5WmMG7ZkGt4YUFTuB2cYiQtWKhQwv6xFDa` |
| **Framework** | Anchor 0.31.1 |
| **Language** | Rust (edition 2021) |
| **Version** | 2.0.0 |
| **License** | Apache-2.0 |
| **Original Website** | [crazysol.io](https://crazysol.io) *(discontinued)* |

---

## Game Concept

CrazySol is themed around a fictional laboratory setting. The game mechanics are wrapped in a scientific metaphor:

- **Scientists** — Players who register on-chain and participate in the economy.
- **Reactor** — The shared SOL pool that backs the game's economy.
- **Pills** — An internal unit earned through a bonding curve when SOL is injected. Pills generate yield over time.
- **pCRAZY** — A pre-TGE (Token Generation Event) points system rewarding engagement (injections, mutations, streaks, referrals).
- **Injection** — Depositing SOL into the Reactor to earn Pills.
- **Distillation** — Claiming accumulated SOL yield (with a 25% pill vaporization penalty).
- **Mutation** — Reinvesting accumulated yield back into Pills instead of withdrawing.
- **Incubation Period** — A daily check-in streak system that rewards pCRAZY tokens.
- **Innoculation** — A one-time admin event that transitions the game into a post-TGE phase, disabling certain pre-launch reward mechanisms.

The referral system supports up to **3 levels deep** (8%, 3%, 1% of the deposit), incentivizing recruitment.

---

## Architecture

### On-Chain State Accounts

All state accounts are derived as PDAs (Program Derived Addresses) with deterministic seeds.

#### `LaboratoryState`
The global configuration singleton for the entire program.

| Field | Type | Description |
|---|---|---|
| `director` | `Pubkey` | Admin authority (can be transferred) |
| `big_pharma` | `Pubkey` | Fee recipient wallet |
| `containment_tax_bps` | `u16` | Platform fee in basis points |
| `is_initialized` | `bool` | Initialization flag |
| `emergency_lockdown` | `bool` | Pause switch for all user operations |
| `innoculation_happened` | `bool` | Post-TGE phase flag |
| `reaction_formula` | `ReactionFormula` | Bonding curve and yield parameters |
| `reserved` | `[u8; 128]` | Reserved space for future upgrades |

**Seed:** `"laboratory-state"`

#### `ReactorState`
The shared liquidity pool state.

| Field | Type | Description |
|---|---|---|
| `sol_reserves` | `u64` | Total SOL held in the reactor |
| `owned_pcrazy` | `u64` | Available pCRAZY tokens for distribution |
| `pill_supply` | `u128` | Total circulating pill supply |
| `external_reactor` | `Pubkey` | External wallet receiving injected SOL |
| `reserved` | `[u8; 128]` | Reserved space for future upgrades |

**Seed:** `"reactor-state"`

#### `ScientistState`
Per-player account storing all individual data.

| Field | Type | Description |
|---|---|---|
| `owner` | `Pubkey` | Player's wallet address |
| `username` | `String` | Display name (max 128 chars) |
| `recruiter` | `Option<Pubkey>` | Referrer's wallet (if any) |
| `owned_pill` | `u128` | Player's pill balance |
| `owned_pcrazy` | `u64` | Player's pCRAZY balance |
| `is_first_injection_done` | `bool` | First injection bonus tracker |
| `is_first_mutation_done` | `bool` | First mutation bonus tracker |
| `sol_injected` | `u64` | Lifetime SOL deposited |
| `last_streak_timestamp` | `i64` | Last daily check-in time |
| `current_streak` | `u32` | Consecutive daily check-in count |
| `last_distillation_timestamp` | `i64` | Last yield accrual snapshot |
| `distillable_yield` | `u64` | Pending claimable SOL yield |
| `test_subjects_count` | `u32` | Direct referrals (level 1) |
| `specimens_count` | `u32` | Indirect referrals (level 2) |
| `samples_count` | `u32` | Indirect referrals (level 3) |
| `earned_sol` | `u64` | Lifetime SOL earned |
| `earned_sol_from_research` | `u64` | Lifetime SOL earned from referrals |
| `reserved` | `[u8; 128]` | Reserved space for future upgrades |

**Seed:** `"scientist-state" + owner_pubkey`

#### `ExperimentState`
Global analytics and statistics tracker.

| Field | Type | Description |
|---|---|---|
| `total_dev_fees_collected` | `u64` | Total platform fees in lamports |
| `total_pills_vaporized` | `u128` | Total pills burned via distillation |
| `total_research_fees_collected` | `u64` | Total referral fees distributed |
| `total_yield_distilled` | `u64` | Total SOL claimed by players |
| `total_mutations_performed` | `u64` | Total SOL reinvested via mutation |
| `total_sol_injected` | `u64` | Total SOL deposited into the game |
| `total_scientists` | `u64` | Total registered players |
| `total_scientists_recruited` | `u64` | Total players who joined via referral |
| `reserved` | `[u8; 128]` | Reserved space for future upgrades |

**Seed:** `"experiment-state"`

---

### Instruction Set

The program exposes 17 instructions divided into three categories:

#### Admin Instructions (Director Only)
| Instruction | Description |
|---|---|
| `initialize` | Deploy and configure the Laboratory, Reactor, and Experiment accounts |
| `innoculate` | Trigger the TGE event (irreversible, disables pre-launch rewards) |
| `switch_emergency_lockdown` | Pause/unpause all player-facing operations |
| `replace_director` | Transfer admin authority to a new wallet |
| `update_big_pharma` | Change the fee recipient address |
| `update_containment_tax_bps` | Adjust the platform fee (max 1%) |
| `update_reaction_formula` | Modify bonding curve and yield parameters |
| `increase_pcrazy_liquidity` | Add pCRAZY tokens to the distribution pool |
| `deposit_from_external_reactor` | Deposit SOL back into the Reactor from the external wallet |
| `give_public_funding` | Airdrop pCRAZY to a specific scientist |

#### Player Instructions
| Instruction | Description |
|---|---|
| `register_scientist` | Create a player account with optional referral link |
| `inject` | Deposit SOL → receive Pills via bonding curve |
| `distill` | Claim accumulated SOL yield (burns 25% of pills) |
| `mutate` | Reinvest yield into more Pills (compounding) |
| `incubation_period` | Claim daily streak pCRAZY reward |

#### View Instructions
| Instruction | Description |
|---|---|
| `get_distillable_output` | Calculate current claimable yield for a scientist |
| `get_pill_potency` | Get current pill price from the bonding curve |

---

### Utility Modules

#### `centrifuge.rs` — Bonding Curve Engine
Implements the core pricing function using **U256 arithmetic** for overflow-safe computation. Calculates how many Pills a given SOL deposit buys based on the current supply and the linear bonding curve formula.

#### `incubate_serum.rs` — Yield Accumulation
Extension method on `ScientistState` that computes and accrues pending yield based on elapsed time, pill holdings, and the configured reward rate. Enforces a minimum daily yield floor.

#### `security.rs` — Guard Functions
Provides reusable access-control checks:
- `require_initialized` / `require_not_initialized`
- `require_operational` (checks emergency lockdown)
- `require_innoculation_not_happened`

#### `time.rs` — Time Utilities
Helper function to check if 24 hours have passed since a given timestamp.

---

## Detailed Function Reference

### Admin Instructions

#### `initialize`
Deploys all three global PDA accounts and configures the initial game parameters.

**Parameters:**
- `big_pharma: Pubkey` — Fee recipient wallet
- `containment_tax_bps: u16` — Platform fee in basis points
- `owned_pcrazy: u64` — Initial pCRAZY supply for distribution
- `external_reactor: Pubkey` — Wallet that receives injected SOL

**Default Reaction Formula:**
| Parameter | Value | Description |
|---|---|---|
| `bonding_curve_m` | 30 | Linear slope of the bonding curve |
| `bonding_curve_p0` | 100,000 | Base price (lamports per pill unit) |
| `reward_rate_per_pill` | 2,222 | Yield rate per pill per second |
| `min_daily_yield_bps` | 800 | Minimum daily yield floor (8%) |
| `first_injection_bonus` | 10 | 10% bonus pills on first injection |
| `first_mutation_bonus` | 15 | 15% bonus pills on first mutation |

---

#### `innoculate`
One-time irreversible transition that marks the TGE (Token Generation Event) as complete. After innoculation, pCRAZY rewards for injections, mutations, and referrals are disabled. The daily streak system is also disabled.

---

#### `switch_emergency_lockdown`
Toggles the emergency pause state. When enabled, all player-facing instructions (`inject`, `distill`, `mutate`, `register_scientist`, `incubation_period`) are blocked.

---

#### `replace_director`
Transfers admin authority to a new wallet. Only callable by the current director.

---

#### `update_containment_tax_bps`
Adjusts the platform fee percentage. Capped at 100 bps (1%).

---

#### `update_reaction_formula`
Hot-swaps all bonding curve and yield parameters in a single transaction.

---

#### `deposit_from_external_reactor`
Allows the designated external reactor wallet to deposit SOL back into the on-chain Reactor PDA, replenishing the yield pool.

---

#### `give_public_funding`
Director-only airdrop of pCRAZY tokens from the Reactor pool to a specific scientist. Validates the target scientist PDA before transferring.

---

### Player Instructions

#### `register_scientist`
Creates a new `ScientistState` PDA for the calling wallet.

**Parameters:**
- `username: String` — Display name (max 128 characters)
- `recruiter: Option<Pubkey>` — Optional referrer wallet

**Referral Chain Updates:** When a recruiter is specified, the instruction walks up to 3 levels of the referral chain, incrementing `test_subjects_count` (level 1), `specimens_count` (level 2), and `samples_count` (level 3) on each ancestor's state. The direct recruiter also receives a pCRAZY bonus (100 tokens) if before innoculation.

---

#### `inject`
Core deposit function. Accepts SOL from the player and converts it to Pills via the bonding curve.

**Parameters:**
- `deposit: u64` — Amount in lamports (minimum 0.05 SOL / 50,000,000 lamports)

**Flow:**
1. Accrues any pending yield via `incubate_serum`
2. Deducts the containment tax → sent to `big_pharma`
3. Distributes referral rewards up the chain (8% / 3% / 1%)
4. Sends remaining SOL to the `external_reactor`
5. Calculates new Pills via the bonding curve
6. Applies first-injection bonus (10% extra pills) if applicable
7. Awards pCRAZY tokens (50) if before innoculation
8. Updates all relevant statistics

---

#### `distill`
Withdraws accumulated SOL yield from the Reactor.

**Flow:**
1. Accrues pending yield
2. Caps withdrawal to available Reactor balance (minus rent-exempt minimum)
3. Deducts containment tax → sent to `big_pharma`
4. Transfers net yield to the player
5. **Burns 25% of the player's pill holdings** (vaporization penalty)
6. Updates global statistics

---

#### `mutate`
Compounds accumulated yield by converting it back into Pills instead of withdrawing SOL.

**Flow:**
1. Accrues pending yield
2. Converts entire pending yield to Pills via the bonding curve
3. Applies first-mutation bonus (15% extra pills) if applicable
4. Awards pCRAZY tokens (10) if before innoculation
5. Resets `distillable_yield` to 0

---

#### `incubation_period`
Daily check-in mechanism that rewards pCRAZY tokens based on streak length.

**Mechanics:**
- Base reward: 25 pCRAZY
- Streak multiplier: `base_reward × streak_count`
- Must wait at least 24h between claims
- Streak resets if more than 48h pass between claims
- Only available before innoculation

---

### View Instructions

#### `get_distillable_output`
Calculates and returns the current claimable yield for a scientist without modifying state. Takes into account elapsed time, pill holdings, reward rate, and the minimum daily yield floor.

#### `get_pill_potency`
Returns the current price of one pill unit based on the linear bonding curve: `price = p0 + m × current_supply`.

---

## Bonding Curve & Tokenomics

The pill pricing follows a **linear bonding curve** defined by:

```
price(s) = p0 + m × s
```

Where `s` is the current pill supply, `p0` is the base price, and `m` is the slope.

The number of pills received for a given deposit is computed using U256 arithmetic to avoid overflow:

```
Δs = (√((p0 + m×s)² + 2×m×deposit) - (p0 + m×s)) / m
```

**Yield Generation:**
- Pills generate yield continuously based on `reward_rate_per_pill × elapsed_seconds`
- A minimum daily yield floor (configurable in bps) ensures a baseline return rate
- Yield accrues to `distillable_yield` and can be claimed (`distill`) or compounded (`mutate`)

**Deflationary Mechanism:**
- Each `distill` operation burns 25% of the scientist's pill holdings
- This reduces the global pill supply, increasing the price for future injections

---

## Event System

Every state-mutating instruction emits a typed Anchor event for off-chain indexing:

| Event | Emitted By |
|---|---|
| `InitializeEvent` | `initialize` |
| `InnoculateEvent` | `innoculate` |
| `RegisterScientistEvent` | `register_scientist` |
| `InjectEvent` | `inject` |
| `DistillEvent` | `distill` |
| `MutateEvent` | `mutate` |
| `IncubationPeriodEvent` | `incubation_period` |
| `DepositFromExternalReactorEvent` | `deposit_from_external_reactor` |
| `IncreasepCRAZYLiquidityEvent` | `increase_pcrazy_liquidity` |
| `GiveSomePcrazyEvent` | `give_public_funding` |
| `SwitchEmergencyLockdownEvent` | `switch_emergency_lockdown` |
| `ReplaceDirectorEvent` | `replace_director` |
| `UpdateBigPharmaEvent` | `update_big_pharma` |
| `UpdateContainmentTaxBpsEvent` | `update_containment_tax_bps` |
| `UpdateReactionFormulaEvent` | `update_reaction_formula` |

---

## Error Codes

| Error | Description |
|---|---|
| `AlreadyInitialized` | Laboratory already initialized |
| `InjectionTooSmall` | Deposit below 0.05 SOL minimum |
| `CooldownActive` | 24h cooldown not yet elapsed |
| `CurrentlyPaused` | Emergency lockdown is active |
| `InsufficientpCRAZY` | Not enough pCRAZY in the reactor pool |
| `InvalidFeePercentage` | Fee exceeds allowed maximum |
| `InvalidPubkey` | Provided pubkey is not on the ed25519 curve |
| `InvalidRecruiter` | Recruiter validation failed |
| `InvalidScientistAddress` | Scientist PDA mismatch |
| `InvalidScientist` | Scientist state deserialization failed |
| `MissingAccount` | Required remaining account not provided |
| `NoYield` | No accumulated yield to claim or compound |
| `Overflow` | Arithmetic overflow detected |
| `InnoculationAlreadyHappened` | Innoculation event already triggered |
| `Unauthorized` | Caller is not the director |
| `UsernameTooLong` | Username exceeds 128 characters |

---

## Project Structure

```
crazysol/
├── Anchor.toml                          # Anchor workspace configuration
├── Cargo.toml                           # Rust workspace manifest
├── Cargo.lock
├── package.json
├── yarn.lock
├── tsconfig.json
├── LICENSE                              # Apache-2.0
├── migrations/
│   └── deploy.ts                        # Anchor migration script
└── programs/
    └── crazysol/
        ├── Cargo.toml                   # Program crate manifest (v2.0.0)
        ├── Xargo.toml
        └── src/
            ├── lib.rs                   # Program entrypoint & instruction routing
            ├── constants.rs             # Game constants (rewards, percentages, timings)
            ├── errors.rs                # Custom error enum
            ├── events.rs                # Anchor event definitions
            ├── states/
            │   ├── mod.rs
            │   ├── laboratory_state.rs  # Global config (LaboratoryState + ReactionFormula)
            │   ├── reactor_state.rs     # Shared pool state
            │   ├── scientist_state.rs   # Per-player state
            │   └── experiment_state.rs  # Global analytics
            ├── instructions/
            │   ├── mod.rs
            │   ├── initialize.rs        # Deploy global accounts
            │   ├── innoculate.rs        # Trigger TGE event
            │   ├── register_scientist.rs# Player registration + referral chain
            │   ├── inject.rs            # SOL deposit → Pills
            │   ├── distill.rs           # Claim yield (with pill burn)
            │   ├── mutate.rs            # Compound yield → Pills
            │   ├── incubation_period.rs # Daily streak rewards
            │   ├── deposit_from_external_reactor.rs
            │   ├── give_public_funding.rs
            │   ├── increase_prcrazy_liquidity.rs
            │   ├── switch_emergency_lockdown.rs
            │   ├── replace_director.rs
            │   ├── update_big_pharma.rs
            │   ├── update_containment_tax_bps.rs
            │   ├── update_reaction_formula.rs
            │   ├── get_distillable_output.rs
            │   └── get_pill_potency.rs
            └── utils/
                ├── mod.rs
                ├── centrifuge.rs        # Bonding curve math (U256)
                ├── incubate_serum.rs    # Yield accrual logic
                ├── security.rs          # Access control guards
                └── time.rs              # Time helper functions
```

---

## Build & Deploy

### Prerequisites

- [Rust](https://rustup.rs/) (edition 2021)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.17+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v0.31.1)
- [Yarn](https://yarnpkg.com/)

### Build

```bash
anchor build
```

### Test (localnet)

```bash
anchor test
```

### Deploy

```bash
anchor deploy
```

> **Note:** The program is configured for `localnet` by default in `Anchor.toml`. Update the `[provider]` section for devnet/mainnet deployment.

---

## License

This project is licensed under the [Apache License 2.0](LICENSE).

```
Copyright 2026 Nayrosk
```
