set -euo pipefail

# ------------------------------------------------------------------
# 1. Load environment variables
# ------------------------------------------------------------------
if [ -f ".env" ]; then
  # shellcheck disable=SC1091
  source .env
  echo "[deploy] Loaded .env"
else
  echo "[deploy] WARNING: .env not found. Using environment variables from shell."
fi

echo "[deploy] Starting deployment process..."
echo "[deploy] Target network : ${SOROBAN_NETWORK:-<not set>}"
echo "[deploy] RPC URL        : ${SOROBAN_RPC_URL:-<not set>}"
echo "[deploy] Deployer       : ${DEPLOYER_ACCOUNT:-<not set>}"

# ------------------------------------------------------------------
# 2. Build contracts to WASM
# ------------------------------------------------------------------
echo "[deploy] Step 1/4 — Build"
# soroban contract build
# This compiles all workspace members to:
#   target/wasm32-unknown-unknown/release/<contract>.wasm

# ------------------------------------------------------------------
# 3. Optimize WASM binary (reduces on-chain footprint)
# ------------------------------------------------------------------
echo "[deploy] Step 2/4 — Optimize"
# soroban contract optimize \
#   --wasm target/wasm32-unknown-unknown/release/credential_registry.wasm

# ------------------------------------------------------------------
# 4. Deploy contract to Soroban network
# ------------------------------------------------------------------
echo "[deploy] Step 3/4 — Deploy"
# CONTRACT_ID=$(soroban contract deploy \
#   --wasm target/wasm32-unknown-unknown/release/credential_registry.optimized.wasm \
#   --source "$DEPLOYER_ACCOUNT" \
#   --rpc-url "$SOROBAN_RPC_URL" \
#   --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE")
#
# echo "[deploy] Contract deployed with ID: $CONTRACT_ID"
# Export so initialize.sh can use it in the same session:
# export CREDENTIAL_REGISTRY_CONTRACT_ID="$CONTRACT_ID"

# ------------------------------------------------------------------
# 5. Upload WASM hash to IPFS (optional — for verifiability)
# ------------------------------------------------------------------
echo "[deploy] Step 4/4 — IPFS upload (optional)"
# WASM_CID=$(ipfs add --quiet \
#   target/wasm32-unknown-unknown/release/credential_registry.optimized.wasm)
# echo "[deploy] WASM uploaded to IPFS: ${IPFS_GATEWAY}${WASM_CID}"

echo "==========================================="
echo " Deployment placeholder complete."
echo " No real deployment was performed."
echo " Uncomment commands above when ready."
echo "==========================================="