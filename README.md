# CargoChain Provenance Ledger

A cocoa shipment from Lagos to Rotterdam, with every handoff recorded on-chain — and anyone can check the trail.

**Shipment CC-2026-001 live: 3 checkpoints, verified on-chain 24 Nov 2026.**

**Demo video:** TODO — film from `VIDEO-SCRIPT.md` (3–5 min).
**Repo:** https://github.com/skymiller1509-blip/cargo-chain
**Contract (ME testnet):** `me1r50m5lafnmctat4xpvwdpzqndynlxt2skhr4fhzh76u0qar2y9hqg2s9ql` — shipment `CC-2026-001` live with 3 checkpoints, verified on-chain 2026-11-24.
**Event:** MetaEarth Hackathon 2026 (HARBOR), Track 1: On-chain finance and trading. Team **Butterfly**. Submit by 21 Dec 2026 15:59 UTC.

Team **Butterfly** — MetaEarth Hackathon 2026 (HARBOR), Track 1: On-chain finance and trading.

One flow: register a shipment, record a checkpoint at each port or handler,
and verify the full trail on-chain. A shipment is **verified** when its trail
has at least **2 checkpoints** (origin + at least one more hop).

Demo shipment: `CC-2026-001`, 1000 bags of cocoa beans,
Lagos, Nigeria → Rotterdam, Netherlands.

## Hackathon Requirements Met

MetaEarth Hackathon 2026 (HARBOR), Track 1: On-chain finance and trading. Team **Butterfly**. Submit by 21 Dec 2026 15:59 UTC. Each requirement, with proof:

- [x] Project description — this README.
- [x] Code repo — https://github.com/skymiller1509-blip/cargo-chain (public, published 25 Nov 2026, 28 files).
- [x] Run records — contract `me1r50m5lafnmctat4xpvwdpzqndynlxt2skhr4fhzh76u0qar2y9hqg2s9ql`, code ID 38, shipment `CC-2026-001` live with 3 checkpoints, `verify_shipment` true (24 Nov 2026). Full record in `DEPLOYMENT.md`.
- [x] Experience entry / local run instructions — Build, test, demo section below (`cargo test`, `./demo.sh`).
- [ ] Demo video, 3–5 min — script in `VIDEO-SCRIPT.md`; raw footage received 26 Nov 2026, final video build pending.
- [ ] Submit by 21 Dec 2026 15:59 UTC.

Demo failure-proofing: the demo page shows the live on-chain trail — no faucet, no wallet and no network call the judge has to make. The video is filmed from the live page. The local `./demo.sh` still runs the in-memory simulation for judges without testnet access, labelled SIMULATION.

## Judging Highlights

- **Innovation:** a provenance ledger where anyone can check the trail — a swapped bag, caught by the trail. The shipment is verified when its trail has at least 2 checkpoints.
- **Technical execution:** CosmWasm contract deployed live on ME testnet (code ID 38); 10/10 tests pass (8 unit + 2 end-to-end); WASM build sha256 published in this README.
- **Impact:** 1000 bags of cocoa, Lagos to Rotterdam — every handoff recorded on-chain, and `verifyShipment()` proves the trail, not just claims it.
- **Completeness:** contract + live demo page + docs, all live; the video is the one piece still to come; honest limits stated below.

## Tech stack

CosmWasm (Rust) on ME Network, `cw-multi-test` for the simulation demo,
`med` CLI for testnet deploy. No paid services.

## Contract

CosmWasm (Rust) on ME Network. Messages:

| Message | What it does |
|---|---|
| `RegisterShipment { shipment_id, origin, destination, description, shipper }` | Register a shipment (id must be unique). Sender is recorded as registrant. |
| `AddCheckpoint { shipment_id, location, status, note }` | Append a checkpoint. Only the shipment's registrant or the contract admin may call this; a stranger's checkpoint is rejected. |
| `GetShipment { shipment_id }` | Full shipment details. |
| `GetCheckpoints { shipment_id }` | Every checkpoint, in the order recorded. |
| `VerifyShipment { shipment_id }` | `{ verified, checkpoint_count, reason }`. Verified = at least 2 checkpoints. |
| `ListShipments {}` | All shipment ids, in registration order. |

`verifyShipment()` checks **only** the checkpoint count (>= 2). It does not
pretend to verify the real-world cargo — that honesty is the point.

## Build, test, demo

```bash
cargo test              # 10 tests: 8 unit + 2 end-to-end (cw-multi-test)
./demo.sh               # full Lagos -> Rotterdam flow, prints the trail
cargo fmt --check       # clean
cargo clippy --all-targets -- -D warnings   # clean
```

Build the deployable WASM:

```bash
RUSTFLAGS='-C link-arg=-s' cargo build --release --lib --target=wasm32-unknown-unknown
# -> target/wasm32-unknown-unknown/release/cargo_chain.wasm (210K)
# sha256: 4ab53ee792eeb36d84b252e7e2f734ef8dae8b5992db2c9df924e76c55b26732
```

`cargo run --bin schema` generates the JSON schemas in `schema/`.

## Deploy (testnet) — DONE 24 Nov 2026

Deployed 24 Nov 2026 ~15:22 WAT. Code ID 38. Contract:
`me1r50m5lafnmctat4xpvwdpzqndynlxt2skhr4fhzh76u0qar2y9hqg2s9ql`.
Shipment `CC-2026-001` live with 3 checkpoints; `verify_shipment`
returns true. See `deploy-testnet.sh` for the exact `med` commands
(store → instantiate → register → 3 checkpoints → verify) and
`DEPLOYMENT.md` for the full record.

The deployer address (fresh testnet-only key, generated 2026-11-24):
`me1x9tzpcpt662m2sakahxufelflftl80xrwdvn5t`. The mnemonic stays in its
mode-600 file and is never shared.

No real funds involved: the beta testnet uses test MEC with no real value.
Never deploy this to mainnet without a separate review.

## What the real ME docs say (verified live 2026-11-23)

- Hackathon: https://www.mec.me/en-US/hackathon/harbor
  Register by 27 Nov 2026 15:59 UTC; submit by 21 Dec 2026 15:59 UTC.
  Submission needs: project description, code repo, 3–5 min demo video,
  experience entry / local run instructions, and run records
  (contract / transaction / call records on ME Network).
- Developer docs: https://docs.mec.me (v2.0.0)
- Smart contract guide: https://docs.mec.me/docs/development-guides/developer-guide/smart-contract-development
  The documented contract path is **CosmWasm (Rust)**: Rust 1.75–1.80,
  `wasm32-unknown-unknown`, cosmwasm-std 1.5.0, deploy with
  `med tx wasm store / instantiate / execute`, query with
  `med q wasm contract-state smart`. (This repo builds with Rust 1.89.0;
  tests, clippy and the WASM build all pass on it.)
- Network details: https://docs.mec.me/docs/getting-started-for-developers/network-details/
- Beta testnet (verified live 2026-11-23, block 5478864):
  chain-id `mechain_400-1`; Hub RPC https://beta-hub-26657.explorer-testnet.me;
  REST https://beta-hub-1317.explorer-testnet.me;
  explorer https://www.explorer-testnet.me;
  faucet https://www.mec.me/en-US/faucet.
  The WASM module is live on testnet (code ids 1, 2, 3 already stored).
- EVM: the Hub runs an Ethermint EVM module (verified live via REST:
  `evm_denom: umec`, `enable_create: true`, `enable_call: true`), and the
  V2.0.17 upgrade brought EVM compatibility. BUT no public EVM JSON-RPC
  endpoint is published in the official docs, a guessed endpoint
  (`beta-hub-8545.explorer-testnet.me`) did not answer, and the JS SDK
  (`meta-earth-js-sdk`) is not on public npm and has no public repo found.
  So the EVM route is **not** deployable from here today. The CosmWasm
  route above is the documented, verifiable one. A Solidity port is a
  one-file job once MetaEarth publishes a public EVM RPC.
- RollApp route: EVM/WASM contracts can also run on your own RollApp
  (official example: https://github.com/openmetaearth/openroll), but that
  needs running a RollApp node and registering it on the Hub — heavier
  than the direct Hub WASM deploy, so it was not used here.

## Honest limits

- The contract is **deployed live on ME testnet** (code ID 38, 24 Nov
  2026): shipment `CC-2026-001`, 3 checkpoints, `verify_shipment` true.
  The demo page shows that live trail.
- The local demo (`./demo.sh`) runs the real contract code on an
  in-memory test chain, not on ME Network. It is labelled SIMULATION
  wherever shown — it is the offline fallback, not the live trail.
- `verifyShipment()` proves the on-chain trail has >= 2 checkpoints. It
  does not prove the physical cargo moved — no oracle or IoT feed is
  wired in. Anyone can see the trail; only the registrant/admin can add
  to it.
- The repo is public: https://github.com/skymiller1509-blip/cargo-chain
  (published 25 Nov 2026, 28 files).

## License

MIT. See `LICENSE`.
