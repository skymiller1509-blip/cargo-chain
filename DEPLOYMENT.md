# CargoChain — MetaEarth beta testnet deployment record

Deployed 2026-11-24 (WAT) by Mabel. Testnet only. Chain: beta testnet,
chain-id `mechain_400-1`.

## Addresses

- Deployer: `me1x9tzpcpt662m2sakahxufelflftl80xrwdvn5t` (key `cargo-deploy` in the
  med keyring, keyring-backend test; mnemonic in `keys/deployer-mnemonic.txt`,
  mode 600, never publish)
- Code ID: **38**
- Contract (canonical, first instantiate):
  `me1r50m5lafnmctat4xpvwdpzqndynlxt2skhr4fhzh76u0qar2y9hqg2s9ql`
- A second instance was created by accident (the deploy script re-instantiates
  on every run; see note below):
  `me1ytn4v3pd9ecklqf32pusephxpnfvnev7657rprr3jh43rchygvmqfvzacj`
  It is empty; use the first contract for the demo and submission.

## Transactions (all code 0)

| What | Tx hash | Height | Gas used/wanted | Fee |
|---|---|---|---|---|
| Store code (code 38) | 098A37A38CFB515B66D52F6DF0479627752E0BC55A5A0E089C9332F8EF3E664D | 5493369 | 8,581,990 / 11,251,661 | 100,000,000 umec |
| Instantiate #1 | 03AE8F0733E5625FE897499761AFD7F45DD4E889350F739CE28D390FBCB6DAC8 | 5493373 | 203,628 / 276,973 | 10,000 umec |
| Instantiate #2 (duplicate) | 5BAEE7EB3C18AD7F5DD406BB4CFC542B411CD98AFE6B77B1BA49480D4DE09A72 | 5493391 | 203,619 / 276,973 | 10,000 umec |
| Register CC-2026-001 | C33E35DF0C13ABA33423B03D78EDAA4C6311201B8B3B8F2837DD6459B10D617E | 5493397 | 203,144 / 268,947 | 10,000 umec |
| Checkpoint 1 Apapa Port | F24CCB453AF92C199A189EDBD048202D04E2833BB2A61A1EAE60541ECA1FCC5C | 5493399 | 189,876 / 255,685 | 10,000 umec |
| Checkpoint 2 Mid-Atlantic | 7D81658376A775A518EE9656647C7611291CF2EE4D605E08699841B1D9249AA0 | 5493401 | 195,318 / 261,037 | 10,000 umec |
| Checkpoint 3 Rotterdam Port | 9327E41F47F2B10D4D4B97A6CE0C96B015AD9659E11DD8FB9A4256C7B0A557DB | 5493403 | 201,369 / 267,136 | 10,000 umec |

Balance: 200 MEC before, **99.956 MEC after** (200,000,000 -> 99,956,000 umec).
Store cost 100 MEC as documented; the six small txs 10,000 umec each.

## On-chain verification (2026-11-24, via med)

- `get_checkpoints` for CC-2026-001 returns all 3 checkpoints (Apapa Port
  departed, Mid-Atlantic in_transit, Rotterdam Port arrived), each recorded
  by the deployer.
- `verify_shipment` returns `verified: true`, `checkpoint_count: 3`,
  "Shipment has 3 checkpoints (minimum 2); trail is verifiable".
- Unit tests still 10/10 after the rebuild.

## Demo shipment

CC-2026-001, Lagos, Nigeria -> Rotterdam, Netherlands, "1000 bags of cocoa
beans", shipper Butterfly Exports Ltd.

## Explorer links

Explorer is a JS app; the transaction page shell returns HTTP 200 at
`https://www.explorer-testnet.me/transactions/<txhash>` (content renders
client-side, not confirmed via fetch). Contract page:
`https://www.explorer-testnet.me` (Smart Contract section).

## How it was deployed (notes for next time)

- `med` v2.0.18 was built from source (me-hub repo); the v2.0.18 release has
  no binary. Binary at `~/.local/bin/med`. Go 1.25.4 at `/usr/local/go`.
- med's CometBFT client ignores HTTPS_PROXY and dials the node directly,
  which fails from this VM. A local forwarder is required:
  `~/workspace/hackathons/metaearth/forwarder.go` (Go reverse proxy on
  127.0.0.1:26657 -> https://beta-hub-26657.explorer-testnet.me via the
  egress proxy). It must set the Host header to the target (otherwise the
  ELB 403s) and strip X-Forwarded-For (also 403s). Run with
  `MED_NODE=http://127.0.0.1:26657`. Restart: build it and
  `nohup /tmp/forwarder-bin > forwarder.log 2>&1 &` from
  `~/workspace/hackathons/metaearth`. /etc/hosts is read-only, so no
  hostname override is possible.
- The chain's wasmd v0.43 Gatekeeper rejects bulk-memory WASM ops
  (MemoryCopy). The stock `cargo build --release --target
  wasm32-unknown-unknown` binary was rejected twice. Fix that worked:
  rebuild with nightly + `-Z build-std=std` +
  `-Z build-std-features=compiler-builtins-mem` and
  `RUSTFLAGS="-C target-cpu=mvp -C link-arg=--allow-undefined"`.
  The `--allow-undefined` is needed because build-std makes rust-lld
  reject the CosmWasm host imports (db_read, db_write, ...). The rebuilt
  WASM (257,793 bytes) has zero bulk-memory ops in its code section.
  To reproduce: `rustup toolchain install nightly`, `rustup component add
  rust-src --toolchain nightly`, then the build above with `--lib`.
- `deploy-testnet.sh` re-instantiates on every run (that is how the duplicate
  contract was created) and its demo txs race the account sequence when run
  back-to-back (sync broadcast returns before inclusion; one demo run failed
  with "account sequence mismatch, expected 3, got 2"). The demo txs were
  run manually with sleeps between them instead. Before re-running the
  script, make it skip instantiate when CONTRACT_ADDR is already set.
- `deploy-testnet.sh` now reads the node from `MED_NODE` (default is the
  real RPC URL); run it with `MED_NODE=http://127.0.0.1:26657` on this VM.
