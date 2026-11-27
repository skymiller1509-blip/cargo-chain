# CargoChain Provenance Ledger — submission checklist

MetaEarth Hackathon 2026 (HARBOR), Track 1: On-chain finance and trading.
Team **Butterfly**. Hackathon page: https://www.mec.me/en-US/hackathon/harbor

## Deadlines (verified live 2026-11-23)

- Register by **27 Nov 2026 15:59 UTC**
- Submit by **21 Dec 2026 15:59 UTC**
- Registration: **done** (Eddie's word, 22 Nov 2026)

## Submission needs

- Project description
- Code repo
- Demo video, **3–5 minutes**
- Experience entry / local run instructions
- Run records (contract / transaction / call records on ME Network)

## Status

- [x] Contract built: CosmWasm (Rust), 10/10 tests pass, WASM built
      (sha256 4ab53ee792eeb36d84b252e7e2f734ef8dae8b5992db2c9df924e76c55b26732)
- [x] **Deployed live on ME testnet 24 Nov 2026** — code ID 38, contract
      `me1r50m5lafnmctat4xpvwdpzqndynlxt2skhr4fhzh76u0qar2y9hqg2s9ql`,
      shipment `CC-2026-001` with 3 checkpoints, `verify_shipment` true.
      Full record in `DEPLOYMENT.md`.
- [x] Repo created and published: https://github.com/skymiller1509-blip/cargo-chain (25 Nov 2026, 28 files)
- [x] Demo page live with the on-chain trail (`demo/index.html`, Earth theme + cute fonts, 26 Nov 2026); local `./demo.sh` simulation kept as the offline fallback, labelled SIMULATION
- [x] Video script written (`VIDEO-SCRIPT.md`)
- [ ] Demo video filmed — raw footage received 26 Nov 2026, final video build pending
- [ ] Submit by 21 Dec 2026 15:59 UTC

## Deploy — Eddie's steps, in order

1. Paste the testnet deployer address into the faucet at
   https://www.mec.me/en-US/faucet (2 MEC per claim, 1 claim per address
   per day; needs an ME ID sign-in on the web page).
   Deployer address: `me1x9tzpcpt662m2sakahxufelflftl80xrwdvn5t`
   (fresh testnet-only key, generated 2026-11-24; the mnemonic stays in
   its mode-600 file and is never shared).
2. Get the `med` CLI — build from https://github.com/openmetaearth/me-hub,
   or the Linux amd64 binary attached to the v2.0.18 release there.
   (Note: the v2.0.18 release has no downloadable binary found so far;
   building from source is the fallback.)
3. Tell Mabel the faucet claim is done — Mabel then runs the deploy
   (`deploy-testnet.sh`: store → instantiate → register → 3 checkpoints
   → verify, chain-id `mechain_400-1`).

## Mabel's steps

- [x] Contract, tests, WASM, simulation demo
- [x] Video script + this checklist
- [ ] Deploy to the beta testnet once the faucet claim is done
- [ ] Re-record note: after deploy, film the live-testnet version of the video
- [ ] After the repo exists: verify the repo + Pages demo link
- [ ] Submit by 21 Dec 2026 15:59 UTC (Eddie files the form; Mabel prepares it)

## Honest limits (say these in the submission, do not hide them)

- Not deployed on-chain yet. The demo runs the real contract code on an
  in-memory test chain and is labelled SIMULATION.
- `verifyShipment()` proves the on-chain trail has >= 2 checkpoints. It
  does not prove the physical cargo moved — no oracle or IoT feed is
  wired in. Anyone can read the trail; only the registrant/admin can add
  to it.
- No real funds involved: the beta testnet uses test MEC with no real value.
  Never deploy to mainnet without a separate review.

## Payout / KYC

- Prize details as listed on the hackathon page (100,000 USDT pool).
  No payout route or KYC step verified yet — check the hackathon page
  before submitting and record it here.
