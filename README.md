
# 🌳 FractMarket Soroban Contracts

This repo contains the Soroban smart contracts for FractMarket.

## Contracts

- `fractional_property`: Mint and manage fractional NFTs for each property.
- `fract_dao`: DAO contract to manage governance using property fractions.
- `dividend_vault`: Manage revenue and distribute dividends.

## Deployment (Futurenet)

1. Install Soroban CLI:

```bash
cargo install --locked soroban-cli
```

2. Configure Futurenet:

```bash
soroban config network add futurenet \
  --rpc-url https://rpc-futurenet.stellar.org:443 \
  --network-passphrase "Test SDF Future Network ; October 2022"
```

3. Create a deployer identity:

```bash
soroban config identity generate deployer
```

4. Build and deploy:

```bash
soroban contract build --path contracts/fractional_property
soroban contract deploy --network futurenet --source deployer --wasm contracts/fractional_property/target/wasm32-unknown-unknown/release/fractional_property.wasm

soroban contract build --path contracts/fract_dao
soroban contract deploy --network futurenet --source deployer --wasm contracts/fract_dao/target/wasm32-unknown-unknown/release/fract_dao.wasm

soroban contract build --path contracts/dividend_vault
soroban contract deploy --network futurenet --source deployer --wasm contracts/dividend_vault/target/wasm32-unknown-unknown/release/dividend_vault.wasm
```

Happy building 🛠️🚀
