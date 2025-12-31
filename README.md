# WS Stablecoin

A Solana-based stablecoin protocol built with Anchor framework that enables users to mint stablecoins by depositing SOL as collateral. This project implements a collateralized debt position (CDP) system with liquidation mechanisms to maintain protocol stability. The protocol uses Pyth Network oracle for real-time price feeds to ensure accurate collateral valuation and liquidation calculations.

## Features

- **Deposit & Mint**: Deposit SOL collateral and mint stablecoins in a single transaction
- **Redeem & Withdraw**: Repay stablecoin debt and withdraw SOL collateral (in development)
- **Pyth Oracle Integration**: Real-time price feeds from Pyth Network for accurate collateral valuation
- **Liquidation System**: Automated liquidation with configurable thresholds (50% default) and bonuses (10% default)
- **Health Factor Monitoring**: Track collateralization ratios to prevent undercollateralization
- **Admin Configuration**: 
  - Initialize protocol configuration with custom parameters
  - Update minimum health factor dynamically
- **Token2022 Support**: Native integration with Solana's Token2022 program
- **PDA-Based Architecture**: Secure program-derived addresses for all protocol accounts
- **Overcollateralization**: Default 200% collateralization requirement for safety

## Architecture

### Core Components

- **Configuration Account** (PDA): Stores protocol-wide parameters including:
  - Authority address
  - Mint account for stablecoin
  - Liquidation threshold (default: 50%)
  - Liquidation bonus (default: 10%)
  - Minimum health factor (default: 1)
  - PDA bumps for security

- **Collateral Account** (PDA per user): Tracks individual user positions:
  - Depositor address
  - SOL account PDA for collateral storage
  - Token account for stablecoin balance
  - Current lamport balance (SOL deposited)
  - Total amount of stablecoins minted
  - Initialization status
  - PDA bumps for security

- **Mint Account** (PDA): Stablecoin mint with:
  - 9 decimals precision
  - Program-controlled authority
  - Freeze authority for security

- **Price Oracle**: Integrates with Pyth Network (PriceUpdateV2) to fetch real-time SOL/USD prices for:
  - Collateral valuation during deposit
  - Health factor calculations
  - Liquidation trigger conditions

### Key Parameters

- **Liquidation Threshold**: 50% (200% overcollateralization required)
- **Liquidation Bonus**: 10% incentive for liquidators
- **Min Health Factor**: 1 (configurable by admin)
- **Decimals**: 9 decimals for the stablecoin token

### Available Instructions

#### Admin Instructions
- `initialize_config`: Initialize the protocol configuration and create the stablecoin mint
- `update_config`: Update the minimum health factor (authority-only)

#### User Instructions
- `deposit_collateral_and_mint_token`: Deposit SOL collateral and mint stablecoins in one transaction
  - Requires Pyth price update account for collateral valuation
  - Automatically creates collateral account if needed
  - Creates associated token account if needed

- `redeem_collatoral_and_burn_tokens`: Repay stablecoin debt and withdraw SOL collateral (in development)
  - Burn stablecoins to reduce debt
  - Withdraw SOL collateral after debt repayment
  - Health factor validation ensures safe withdrawals

## Tech Stack

- **Framework**: [Anchor](https://www.anchor-lang.com/) 0.30.1
- **Blockchain**: Solana
- **Language**: Rust (programs), TypeScript (tests)
- **Token Standard**: Token2022 (SPL Token Interface)
- **Price Oracle**: [Pyth Network](https://pyth.network/) (pyth-solana-receiver-sdk 0.3.1)
- **Architecture**: PDA-based for enhanced security

## Project Structure

```
ws_stablecoin/
├── programs/
│   └── ws_stablecoin/
│       └── src/
│           ├── lib.rs              # Main program entry point
│           ├── constants.rs        # Protocol constants and seeds
│           ├── instructions/       # Instruction handlers
│           │   ├── admin/         # Admin instructions
│           │   │   ├── initialize_config.rs
│           │   │   └── update_config.rs
│           │   ├── deposit/       # Deposit instructions
│           │   │   ├── deposit_collateral_and_mint_token.rs
│           │   │   └── util.rs    # Utility functions (SOL deposit, token mint)
│           │   ├── withdraw/      # Withdraw instructions
│           │   │   └── redeem_collatoral_and_burn_tokens.rs
│           │   └── util.rs        # Shared utility functions (health factor, price calculations)
│           └── states/            # Account state definitions
│               └── state.rs       # Collateral and Configuration structs
├── tests/                          # TypeScript integration tests
└── frontend/                       # Frontend application
```

## How It Works

### Deposit & Mint Flow

1. **User deposits SOL**: SOL is transferred to a PDA-controlled SOL account
2. **Price verification**: Pyth oracle provides current SOL/USD price
3. **Collateral calculation**: System calculates collateral value based on oracle price
4. **Stablecoin minting**: Stablecoins are minted to user's associated token account
5. **Position tracking**: Collateral account tracks user's SOL balance and minted amount

### Redeem & Withdraw Flow (In Development)

1. **User repays debt**: Burns stablecoins to reduce `amount_minted`
2. **Health factor check**: Ensures position remains safe after withdrawal
3. **Collateral withdrawal**: SOL is transferred back to user's wallet
4. **Position update**: Collateral account balance and debt are updated

### Security Features

- **PDA-based accounts**: All protocol accounts use Program Derived Addresses for enhanced security
- **Overcollateralization**: Default 200% collateral requirement prevents undercollateralization
- **Oracle price feeds**: Real-time price data ensures accurate valuations
- **Program-controlled mint**: Stablecoin mint authority is controlled by the program PDA

## Getting Started

### Prerequisites

- Rust (latest stable)
- Solana CLI
- Anchor Framework
- Node.js and Yarn

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy

```bash
anchor deploy
```

## Development

This project is configured for Solana devnet. Update the cluster and wallet settings in `Anchor.toml` for different environments.

## License

[Add your license here]

## Disclaimer

This is a bootcamp/educational project. Do not use in production without proper security audits and testing.

