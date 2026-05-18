# OpenCred Deployment Scripts

This directory contains **placeholder** scripts for future Soroban deployment workflows.
No real deployment logic is implemented yet - all Soroban CLI commands are commented out.

---

## Scripts

| Script | Purpose |
|---|---|
| `deploy.sh` | Build, optimize, and deploy contracts to a Soroban network |
| `initialize.sh` | Call the contract's `init()` entrypoint after deployment |

---

## Prerequisites (for when real deployment is implemented)

1. **Rust + wasm32 target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Soroban CLI**
   ```bash
   cargo install --locked soroban-cli
   ```

3. **Environment configuration**
   ```bash
   cp .env.example .env
   # Edit .env with your real values
   ```

---

## Environment Variables

All scripts source `.env` from the project root. See [`.env.example`](../.env.example) for the full list.

| Variable | Description |
|---|---|
| `SOROBAN_RPC_URL` | RPC endpoint (testnet or mainnet) |
| `SOROBAN_NETWORK_PASSPHRASE` | Stellar network passphrase |
| `SOROBAN_NETWORK` | Network name used by Soroban CLI |
| `DEPLOYER_ACCOUNT` | Signing account name or address |
| `DEPLOYER_SECRET_KEY` | Secret key (CI only - use GitHub Secrets) |
| `CREDENTIAL_REGISTRY_CONTRACT_ID` | Populated after first deployment |
| `IPFS_GATEWAY` | Public IPFS gateway URL |
| `IPFS_PINNING_API_KEY` | Optional pinning service API key |

---

## Intended Deployment Flow

```
make build
    └── cargo build --target wasm32-unknown-unknown --release

soroban contract optimize --wasm <...>
    └── Reduces on-chain WASM binary size

scripts/deploy.sh
    ├── Builds WASM
    ├── Deploys to Soroban via soroban contract deploy
    └── (Optional) Pins WASM to IPFS

scripts/initialize.sh
    ├── Calls contract init() entrypoint
    └── Verifies on-chain state
```

---

## CI Integration

The deploy step in `.github/workflows/ci.yml` is also a placeholder.
It only runs on pushes to `main` (not PRs) and currently prints a status message.

When real deployment is ready:
1. Add `SOROBAN_RPC_URL`, `DEPLOYER_SECRET_KEY`, etc. to **GitHub Secrets**
2. Uncomment the Soroban CLI steps in `ci.yml`
3. Uncomment the commands in `deploy.sh` and `initialize.sh`

---

## Security Notes

- **Never** commit real secret keys or RPC credentials to this repository.
- The `.env` file is listed in `.gitignore` and must stay there.
- In CI, pass secrets via `${{ secrets.YOUR_SECRET }}` - never via hardcoded values.