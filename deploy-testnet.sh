#!/bin/bash
# CargoChain testnet deploy: ME Network beta testnet (chain-id mechain_400-1).
#
# PREREQUISITES (Eddie's steps, not done yet):
#   1. A funded testnet key. Get test MEC from the faucet:
#      https://www.mec.me/en-US/faucet
#      (needs an ME ID sign-in; the faucet page is a web step).
#   2. The `med` CLI (ME Hub binary, app name "med").
#      Build from https://github.com/openmetaearth/me-hub, or use the
#      Linux amd64 static binary attached to the v2.0.18 release there.
#      This VM has no Go toolchain and the release API returned no
#      downloadable asset at build time, so `med` is NOT installed here yet.
#
# Nothing in this script spends real funds: the beta testnet uses test MEC
# with no real value. Never run these commands against mainnet.
#
# Usage:
#   Step 1 (store the code, get a code_id):
#       KEY_NAME=mykey ./deploy-testnet.sh
#   Step 2 (instantiate + run the Lagos -> Rotterdam demo on-chain):
#       KEY_NAME=mykey CODE_ID=4 ./deploy-testnet.sh
#   (KEY_NAME is the key already imported in the med keyring and funded
#    from the faucet. CONTRACT_ADDR is printed at the end of step 2;
#    save it for the submission's Run Records.)

set -e
cd "$(dirname "$0")"

NODE="${MED_NODE:-https://beta-hub-26657.explorer-testnet.me}"
# MED_NODE override: on VMs where med's CometBFT client cannot reach the
# node directly (it ignores HTTPS_PROXY), point it at a local forwarder,
# e.g. MED_NODE=http://127.0.0.1:26657 (see ../forwarder.go).
CHAIN_ID="mechain_400-1"
KEY_NAME="${KEY_NAME:?Set KEY_NAME to your funded testnet key name}"
WASM="target/wasm32-unknown-unknown/release/cargo_chain.wasm"

echo "== Key and balance =="
med keys show "$KEY_NAME" --keyring-backend test
ADDR=$(med keys show "$KEY_NAME" -a --keyring-backend test)
med q bank balances "$ADDR" --node "$NODE"

if [ -z "${CODE_ID:-}" ]; then
  echo "== Store the WASM (returns a code_id) =="
  med tx wasm store "$WASM" \
    --from "$KEY_NAME" --keyring-backend test \
    --node "$NODE" --chain-id "$CHAIN_ID" \
    --fees 100000000umec --gas auto --gas-adjustment 1.3 -y
  echo "== Find the new code_id (highest number) =="
  med q wasm list-code --node "$NODE"
  echo "Re-run with CODE_ID=<new code id> to instantiate and demo."
  exit 0
fi

echo "== Instantiate (code_id $CODE_ID) =="
med tx wasm instantiate "$CODE_ID" '{}' \
  --label "cargo-chain" \
  --admin "$ADDR" \
  --from "$KEY_NAME" --keyring-backend test \
  --node "$NODE" --chain-id "$CHAIN_ID" \
  --fees 10000umec --gas auto -y
echo "Get the contract address from the output above (_contract_address), then:"
echo "  export CONTRACT_ADDR=me1..."
echo "Or query it: med q wasm list-contract-by-code $CODE_ID --node $NODE"

CONTRACT_ADDR="${CONTRACT_ADDR:?Set CONTRACT_ADDR from the instantiate output, then re-run with CODE_ID and CONTRACT_ADDR set}"
echo "== Demo on-chain: register CC-2026-001 (Lagos -> Rotterdam) =="
med tx wasm execute "$CONTRACT_ADDR" \
  '{"register_shipment":{"shipment_id":"CC-2026-001","origin":"Lagos, Nigeria","destination":"Rotterdam, Netherlands","description":"1000 bags of cocoa beans","shipper":"Butterfly Exports Ltd"}}' \
  --from "$KEY_NAME" --keyring-backend test \
  --node "$NODE" --chain-id "$CHAIN_ID" --fees 10000umec --gas auto -y

echo "== Checkpoints: Apapa Port -> Mid-Atlantic -> Rotterdam Port =="
for CP in \
  '{"add_checkpoint":{"shipment_id":"CC-2026-001","location":"Apapa Port, Lagos","status":"departed","note":"Loaded onto MV Butterfly"}}' \
  '{"add_checkpoint":{"shipment_id":"CC-2026-001","location":"Mid-Atlantic","status":"in_transit","note":"Position report, day 9"}}' \
  '{"add_checkpoint":{"shipment_id":"CC-2026-001","location":"Rotterdam Port","status":"arrived","note":"Unloaded, customs cleared"}}' ; do
  med tx wasm execute "$CONTRACT_ADDR" "$CP" \
    --from "$KEY_NAME" --keyring-backend test \
    --node "$NODE" --chain-id "$CHAIN_ID" --fees 10000umec --gas auto -y
done

echo "== Read the trail back =="
med q wasm contract-state smart "$CONTRACT_ADDR" '{"get_checkpoints":{"shipment_id":"CC-2026-001"}}' --node "$NODE"
echo "== Verify (needs >= 2 checkpoints) =="
med q wasm contract-state smart "$CONTRACT_ADDR" '{"verify_shipment":{"shipment_id":"CC-2026-001"}}' --node "$NODE"
echo "Done. Contract: $CONTRACT_ADDR"
echo "Explorer: https://www.explorer-testnet.me"
