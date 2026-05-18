set -euo pipefail

# ------------------------------------------------------------------
# 1. Load environment variables
# ------------------------------------------------------------------
if [ -f ".env" ]; then
  # shellcheck disable=SC1091
  source .env
  echo "[init] Loaded .env"
else
  echo "[init] WARNING: .env not found. Using environment variables from shell."
fi

echo "[init] Starting contract initialization..."
echo "[init] Contract ID : ${CREDENTIAL_REGISTRY_CONTRACT_ID:-<not set>}"
echo "[init] Deployer    : ${DEPLOYER_ACCOUNT:-<not set>}"

# ------------------------------------------------------------------
# 2. Initialize CredentialRegistry contract
# ------------------------------------------------------------------
echo "[init] Step 1/2 — Call contract init()"
# soroban contract invoke \
#   --id "$CREDENTIAL_REGISTRY_CONTRACT_ID" \
#   --source "$DEPLOYER_ACCOUNT" \
#   --rpc-url "$SOROBAN_RPC_URL" \
#   --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE" \
#   -- \
#   init \
#   --admin "$DEPLOYER_ACCOUNT"
#
# Arguments will depend on the final init() signature in lib.rs.

# ------------------------------------------------------------------
# 3. Verify initialization (optional sanity check)
# ------------------------------------------------------------------
echo "[init] Step 2/2 — Verify state"
# soroban contract invoke \
#   --id "$CREDENTIAL_REGISTRY_CONTRACT_ID" \
#   --source "$DEPLOYER_ACCOUNT" \
#   --rpc-url "$SOROBAN_RPC_URL" \
#   --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE" \
#   -- \
#   get_admin

echo "==========================================="
echo " Initialization placeholder complete."
echo " No real initialization was performed."
echo " Uncomment commands above when ready."
echo "==========================================="