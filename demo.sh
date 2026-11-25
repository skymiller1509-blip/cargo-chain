#!/bin/bash
# CargoChain demo: the full Port A -> Port B flow, run against the real
# contract code on an in-memory CosmWasm chain (cw-multi-test).
# No funds, no network, no keys needed.
# Usage: ./demo.sh
set -e
cd "$(dirname "$0")"
export PATH="$HOME/.cargo/bin:$PATH"
cargo test --test integration -- --nocapture
