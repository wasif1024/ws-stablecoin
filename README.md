# WS Stablecoin

A Solana-based stablecoin protocol built with Anchor framework that enables users to mint stablecoins by depositing SOL as collateral. This project implements a collateralized debt position (CDP) system with liquidation mechanisms to maintain protocol stability. The protocol uses Pyth Network oracle for real-time price feeds to ensure accurate collateral valuation and liquidation calculations.

## Features

- **Deposit & Mint**: Deposit SOL collateral and mint stablecoins in a single transaction
- **Redeem & Withdraw**: Repay stablecoin debt by burning tokens and withdraw SOL collateral with health factor validation
- **Pyth Oracle Integration**: Real-time price feeds from Pyth Network for accurate collateral valuation
- **Liquidation System**: Fully automated liquidation with health factor checks, bonus calculations, and debt repayment
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
  - USD to SOL conversion for liquidation calculations

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

- `redeem_collateral_and_burn_tokens`: Repay stablecoin debt and withdraw SOL collateral
  - Burn stablecoins to reduce `amount_minted` (debt)
  - Withdraw SOL collateral from PDA account
  - Health factor validation ensures position remains safe after withdrawal
  - Requires Pyth price update account for real-time collateral valuation

- `liquidate`: Liquidate undercollateralized positions
  - Only works when health factor < minimum health factor
  - Liquidator pays stablecoin debt (burns tokens)
  - Liquidator receives SOL collateral + liquidation bonus (10% default)
  - Automatically updates collateral account balance and debt
  - Requires Pyth price update account for accurate price calculations

## Tech Stack

- **Framework**: [Anchor](https://www.anchor-lang.com/) 0.30.1
- **Blockchain**: Solana
- **Language**: Rust (programs), TypeScript (tests)
- **Testing**: Anchor test framework with Pyth Solana Receiver SDK integration
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
│           │   │   ├── redeem_collatoral_and_burn_tokens.rs
│           │   │   ├── liquidate.rs
│           │   │   └── utils.rs   # Utility functions (SOL withdrawal, token burn)
│           │   └── util.rs        # Shared utility functions (health factor, price calculations)
│           └── states/            # Account state definitions
│               └── state.rs       # Collateral and Configuration structs
├── tests/                          # TypeScript integration tests
│   └── ws_stablecoin.ts           # Test suite covering all instructions
└── frontend/                       # Frontend application
```

## How It Works

### Deposit & Mint Flow

1. **User deposits SOL**: SOL is transferred to a PDA-controlled SOL account
2. **Price verification**: Pyth oracle provides current SOL/USD price
3. **Collateral calculation**: System calculates collateral value based on oracle price
4. **Stablecoin minting**: Stablecoins are minted to user's associated token account
5. **Position tracking**: Collateral account tracks user's SOL balance and minted amount

### Redeem & Withdraw Flow

1. **User repays debt**: Burns stablecoins to reduce `amount_minted` (debt)
2. **Health factor validation**: System calculates health factor to ensure position remains safe
3. **Collateral withdrawal**: SOL is transferred from PDA account back to user's wallet
4. **Position update**: Collateral account balance (`lamport_balance`) and debt (`amount_minted`) are updated
5. **Safety check**: Withdrawal only proceeds if health factor remains >= 1.0

### Liquidation Flow

1. **Health factor check**: System verifies health factor < minimum health factor (default: 1.0)
2. **Price calculation**: Uses Pyth oracle to convert stablecoin debt amount to SOL value
3. **Bonus calculation**: Calculates liquidation bonus (10% of SOL value by default)
4. **Liquidator pays debt**: Liquidator burns stablecoins to pay user's debt
5. **Liquidator receives collateral**: SOL is transferred to liquidator (debt amount + bonus)
6. **Position update**: Collateral account balance and debt are automatically updated
7. **Post-liquidation check**: Health factor is recalculated to verify position improvement

### Security Features

- **PDA-based accounts**: All protocol accounts use Program Derived Addresses for enhanced security
- **Overcollateralization**: Default 200% collateral requirement prevents undercollateralization
- **Oracle price feeds**: Real-time price data ensures accurate valuations
- **Program-controlled mint**: Stablecoin mint authority is controlled by the program PDA
- **Health factor validation**: All withdrawals are validated to ensure positions remain safe
- **Safe debt repayment**: Users can only withdraw collateral if health factor stays above threshold
- **Automated liquidation**: Under-collateralized positions are automatically liquidatable by anyone
- **Liquidation bonus**: Incentivizes liquidators to maintain protocol health (10% default)
- **Error handling**: Comprehensive error codes for price validation and health factor checks

## Getting Started

### Prerequisites

- Rust (latest stable)
- Solana CLI
- Anchor Framework
- Node.js and Yarn
- Pyth Solana Receiver SDK (for testing with oracle price feeds)

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

The test suite includes comprehensive integration tests covering all protocol instructions:

- **Initialize Config**: Tests protocol initialization and stablecoin mint creation
- **Deposit & Mint**: Tests SOL collateral deposit and stablecoin minting (1 SOL deposit, 1 stablecoin mint)
- **Redeem & Withdraw**: Tests debt repayment and collateral withdrawal (0.5 SOL withdrawal, 0.5 stablecoin burn)
- **Update Config**: Tests admin configuration updates (sets min health factor to 100 for liquidation test)
- **Liquidation**: Tests liquidation of undercollateralized positions (0.5 stablecoin debt liquidation)
- **Config Reset**: Resets min health factor back to 1 after liquidation test

**Test Setup:**
- Uses Pyth Solana Receiver SDK for oracle price feeds
- Tests run on localnet with Pyth price feed integration
- All transactions use `confirmed` commitment and skip preflight for faster execution

### Deploy

```bash
anchor deploy
```

## Error Handling

The protocol includes comprehensive error handling with the following custom errors:

- `PriceNotAvailable`: Pyth oracle price feed is not available
- `PriceNotValid`: Price from oracle is invalid (e.g., price <= 0)
- `HealthFactorTooLow`: Health factor is below the minimum threshold
- `AboveMinimumHealthFactor`: Cannot liquidate - health factor is above minimum (position is safe)

## Development

This project is configured for Solana devnet. Update the cluster and wallet settings in `Anchor.toml` for different environments.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**MIT License**

Copyright (c) 2024 WS Stablecoin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Disclaimer

This is a bootcamp/educational project. Do not use in production without proper security audits and testing.

