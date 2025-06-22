## Project Structure

This repository contains **Steptions** - a Multi-Pool American-Style Options Trading Platform built on Stellar using Soroban smart contracts. The platform enables pool-based, cash-settled options trading with integrated price oracles.

```text
.
├── contracts
│   └── steptions
│       ├── src
│       │   ├── lib.rs          # Main options contract implementation
│       │   └── test.rs         # Contract tests
│       └── Cargo.toml
├── bindings/                   # Generated TypeScript bindings
├── Cargo.toml
└── README.md
```

- **Contracts**: The `steptions` contract implements a sophisticated options trading system
- **Core Features**: Multi-pool liquidity management, American-style options, SEP-40 oracle integration
- **Bindings**: Auto-generated TypeScript interfaces for frontend integration
- **Workspace**: Uses a top-level `Cargo.toml` for dependency management

## Platform Features

### Multi-Pool Architecture

- **Liquidity Pools**: Separate pools for different asset pairs (e.g., BTC/USDC, ETH/USDC)
- **Pool Management**: Admin-controlled pool creation and status management
- **LP Tokens**: Proportional share system for liquidity providers

### Options Trading

- **American-Style Options**: Exercise anytime before expiration
- **Call & Put Options**: Support for both option types
- **Cash Settlement**: No physical asset delivery required
- **Pool-Based Collateral**: Liquidity pools back option contracts

### Oracle Integration

- **SEP-40 Price Feeds**: Real-time price data for accurate settlements
- **Multiple Assets**: Support for various underlying assets
- **Price Validation**: Built-in price feed verification

## Deployment

The contract is deployed on Stellar Testnet at:

```
CBBUY4X3NM6DCRRA52TX7SC3XBPQMGFEM3VORETJ3I4FPSU3MWAURO7G
```

## Library Generation

This library was automatically generated using Soroban CLI:

```bash
# From the contract directory
stellar contract build

# Generate TypeScript bindings
stellar contract bindings typescript \
    --wasm target/wasm32v1-none/release/steptions.wasm \
    --output-dir bindings
```

## Installation

### Option 1: Local Dependency

Add to your `package.json`:

```json
{
  "dependencies": {
    "steptions-bindings": "./path/to/bindings"
  }
}
```

### Option 2: Auto-Generation (Recommended)

Use a `postinstall` script to keep bindings up-to-date:

```json
{
  "scripts": {
    "postinstall": "stellar contract bindings typescript --wasm ./steptions.wasm --output-dir ./node_modules/steptions-bindings"
  }
}
```

## Usage

```typescript
import { Contract, networks } from "steptions-bindings";

const contract = new Contract({
  ...networks.testnet,
  rpcUrl: "https://soroban-testnet.stellar.org",
});

// Create a new liquidity pool
await contract.add_liquidity_pool({
  stable_token: "USDC_TOKEN_ADDRESS",
  underlying_asset: "BTC_TOKEN_ADDRESS",
  price_feed: "ORACLE_ADDRESS",
  name: "BTC/USDC Options Pool",
});

// Provide liquidity to a pool
await contract.provide_liquidity({
  pool_id: 0,
  provider: "USER_ADDRESS",
  amount: 1000_0000000, // 1000 USDC (7 decimal places)
});

// Buy a call option
await contract.buy_option({
  pool_id: 0,
  buyer: "USER_ADDRESS",
  opt_type: { tag: "Call" },
  strike: 50000_0000000, // $50,000 strike
  expiry: 1735689600, // Unix timestamp
  amount: 1_0000000, // 1 BTC worth
});
```

## Network Configuration

The contract networks are configured in `src/index.ts`:

```typescript
export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CBBUY4X3NM6DCRRA52TX7SC3XBPQMGFEM3VORETJ3I4FPSU3MWAURO7G",
  },
};
```

## Development

### Contract Architecture

The Steptions contract includes:

- **Pool Management**: Create and manage multiple trading pools
- **Liquidity Operations**: Provide/withdraw liquidity with LP token accounting
- **Options Trading**: Buy, exercise, and expire options
- **Oracle Integration**: Real-time price feeds via SEP-40
- **Event System**: Comprehensive event logging for all operations

### Key Functions

- `add_liquidity_pool()` - Admin creates new trading pools
- `provide_liquidity()` / `withdraw_liquidity()` - LP operations
- `buy_option()` - Purchase call/put options
- `exercise_option()` - Exercise options (American-style)
- `expire_option()` - Handle option expiration

To extend this platform:

1. Add new oracle integrations for additional assets
2. Implement advanced option strategies
3. Add governance mechanisms for fee structures
4. Integrate with DEX protocols for hedging

For testing and development:

```bash
stellar contract test
stellar contract deploy --network testnet
```
