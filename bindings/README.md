# Options Protocol TypeScript Bindings

TypeScript library for interacting with the **Multi-Pool American-Style Options Trading Platform** smart contract on [Stellar Soroban](https://soroban.stellar.org/).

## About the Options Protocol

This protocol implements a decentralized options trading platform with the following key features:

### 🏊 **Multi-Pool Architecture**

- Multiple isolated liquidity pools for different trading pairs (e.g., BTC/USDC, ETH/USDC)
- Each pool has its own stable token for premiums/settlements and underlying asset
- Pool-specific collateral management and risk isolation

### 📊 **American-Style Options**

- **Call Options**: Right to buy the underlying asset at a strike price
- **Put Options**: Right to sell the underlying asset at a strike price
- Exercise any time before expiration (American-style)
- Cash-settled using SEP-40 price oracles

### 💰 **Liquidity Provision**

- Provide liquidity to specific pools and earn from option premiums
- Receive LP shares proportional to your contribution
- Withdraw available liquidity (minus locked collateral)

### 🔧 **Key Functions**

- `add_liquidity_pool()` - Admin creates new trading pools
- `provide_liquidity()` - Users add funds to earn from premiums
- `buy_option()` - Purchase call/put options with 2% premium
- `exercise_option()` - Exercise profitable options before expiry
- `withdraw_liquidity()` - Remove funds when not locked as collateral

## Library Generation

This library was automatically generated using Soroban CLI:

```bash
# From the contract directory
make bindings

# Or manually:
stellar contract build
stellar contract bindings typescript \
  --wasm target/wasm32v1-none/release/options_contract.wasm \
  --output-dir bindings
```

## Development Workflow

The included `Makefile` provides common development tasks:

```bash
# Build the contract
make build

# Run tests
make test

# Format code
make fmt

# Generate TypeScript bindings
make bindings

# Deploy to testnet
make deploy

# Clean build artifacts
make clean
```

## Installation

### Option 1: Local Dependency

Add to your `package.json`:

```json
{
  "dependencies": {
    "options-protocol": "./path/to/bindings"
  }
}
```

### Option 2: Auto-Generation (Recommended)

Use a `postinstall` script to keep bindings up-to-date:

```json
{
  "scripts": {
    "postinstall": "stellar contract bindings typescript --wasm ./contract.wasm --output-dir ./node_modules/options-protocol"
  }
}
```

## Usage

```typescript
import { Contract, networks } from "options-protocol";

const contract = new Contract({
  ...networks.testnet,
  rpcUrl: "https://soroban-testnet.stellar.org",
});

// Add liquidity to a pool
await contract.provide_liquidity({
  pool_id: 1n,
  provider: userAddress,
  amount: 1000_0000000n, // 1000 tokens (7 decimal places)
});

// Buy a call option
await contract.buy_option({
  pool_id: 1n,
  buyer: userAddress,
  opt_type: { tag: "Call" },
  strike: 50000_0000000n, // $50,000 strike price
  expiry: expiryTimestamp,
  amount: 1_0000000n, // 1 unit of underlying asset
});

// Exercise if profitable
await contract.exercise_option({
  option_id: optionId,
});
```

## Network Configuration

The contract networks are configured in `src/index.ts`. Update with your deployed contract addresses:

```typescript
export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "YOUR_CONTRACT_ID_HERE",
  },
};
```

## Protocol Economics

- **Premiums**: 2% of (strike price × normalized amount)
- **Collateral**: Full strike value locked per option
- **Scaling**: All amounts use 7 decimal places (1e7)
- **Settlement**: Cash-settled using SEP-40 price feeds
