# CargoChain Provenance Ledger — demo video script

**Length:** 3–5 minutes (hackathon requirement). Target 4 minutes.
**Format:** phone screen recording, landscape, captions on. Read aloud in
plain words, at a steady pace. Narrate what is happening, not the labels
on screen.

> **SIMULATION — read this first.** This video shows the Lagos →
> Rotterdam flow running against the real contract code on an in-memory
> test chain (`./demo.sh`), NOT on ME Network yet. The contract is built,
> tested (10/10) and compiled to WASM, but it is **not deployed on-chain
> yet**: deploy waits on a faucet-funded testnet key (Eddie's step) and
> the `med` CLI. Say "simulation" out loud in the video and keep the
> SIMULATION label on screen whenever the demo page is shown. When the
> contract is deployed, re-record the flow against the live testnet and
> replace this video.

## The one sentence a judge should repeat

"A cocoa shipment from Lagos to Rotterdam, with every handoff recorded
on-chain — and anyone can check the trail."

## Hook options (pick ONE, say it aloud, keep the clearest)

1. "One thousand bags of cocoa leave Lagos. By the time they reach Rotterdam, who can prove where they have been?"
2. "This shipment has a trail you cannot rewrite — every port, every handoff, on-chain."
3. "A buyer in Rotterdam is about to pay for cocoa. First, she checks the trail."

## Shot-by-shot script

### 0:00–0:05 — HOOK (sound off must still work)

- **On screen (large text, bottom third):** "LAGOS → ROTTERDAM. EVERY HANDOFF ON-CHAIN."
- **Show:** the demo page already open on shipment CC-2026-001, the
  trail visible. No logo, no greeting.
- **Say:** "One thousand bags of cocoa leave Lagos for Rotterdam. This
  is the trail that proves where they have been."

### 0:05–0:25 — PROBLEM, one sentence, one named user

- **On screen:** the shipment card — 1000 bags of cocoa beans, Lagos →
  Rotterdam.
- **Say:** "Ada is a buyer in Rotterdam. She is about to pay for a
  thousand bags of cocoa, but the paper trail she was sent could have
  been written by anyone. She needs a trail nobody can rewrite after
  the fact."
- **Say:** "CargoChain records every handoff on-chain. This is a
  simulation of that flow — the contract code is real, the chain here
  is a test chain, and the on-chain deploy follows once the testnet
  key is funded."

### 0:25–2:45 — THE ONE LIVE FLOW (simulation, labelled SIMULATION)

1. **Register the shipment.** "First, the shipment is registered:
   CC-2026-001, 1000 bags of cocoa beans, Lagos to Rotterdam. The
   shipper who registers it is recorded — only they, or the contract
   admin, can add to its trail."
2. **Add the checkpoints, one by one.** "Lagos port — loaded. Then the
   next handoff, then the next. Each checkpoint is a location, a
   status and a note, stamped in order."
3. **Try a stranger's checkpoint — show it rejected.** "Now the hard
   part, shown honestly: a stranger tries to add a checkpoint to
   someone else's shipment, and the contract rejects it. Only the
   registrant can write to the trail. Anyone can read it."
4. **Verify the shipment.** "Verify: the trail has at least two
   checkpoints, so the shipment is verified. Two checkpoints is the
   rule — origin plus at least one more hop."
5. **Show the full trail.** "There it is — the whole trail, in order,
   from Lagos to Rotterdam. A buyer checks this before she pays."

### 2:45–3:15 — HOW IT WORKS (30 seconds)

- **Say:** "Underneath, this is a CosmWasm contract in Rust on ME
  Network — the documented contract path in the real ME docs. Register,
  add checkpoint, get checkpoints, verify. Ten tests pass, including
  two end-to-end tests on an in-memory chain, and the WASM is built."
- **Say:** "And the honest limit: verify checks the checkpoint count,
  not the physical cargo. It proves the trail exists — it does not
  pretend to prove the beans moved. No oracle is wired in yet."

### 3:15–4:00 — WHAT'S NEXT

- **Say:** "Next is the live deploy: fund the testnet key from the
  faucet, store and instantiate the contract on the ME beta testnet,
  and run this same Lagos to Rotterdam flow on-chain. After that, an
  oracle or IoT feed, so the trail records the real world, not just
  the claim. Team Butterfly, Track 1, MetaEarth HARBOR."
- **On screen (end card):** repo link (once created) + hackathon link.

## Recording notes (phone)

- Android screen recorder: swipe down twice, tap Screen record,
  microphone ON, wait for the countdown.
- **Do Not Disturb ON first** — no notification mid-take.
- Landscape, browser zoom 100–125%, full screen, bookmarks bar hidden,
  extra tabs closed. Run `./demo.sh` first so the flow is warm, then
  record the demo page.
- Keep the word SIMULATION visible on screen during the demo, and say
  it out loud at least twice.
- Record one long take (~90 seconds of the live flow), then trim in
  CapCut: a judge cut at 60–75 seconds and a social cut at 25–35 seconds
  from the same footage.
- Add captions — the video may be watched without sound.
- Never show a private key or mnemonic on screen. The deployer
  mnemonic stays in its mode-600 file and is never opened on camera.

## Before filming — checklist

- [ ] `./demo.sh` runs clean (10/10 tests) on the filming machine
- [ ] Demo page opens and the CC-2026-001 trail shows end to end
- [ ] The stranger-checkpoint rejection is visible in the demo
- [ ] SIMULATION label visible on screen
- [ ] 60-second insurance capture of the flow working, saved separately

## After the deploy — re-record

- [ ] Faucet funded, contract deployed on the ME beta testnet
- [ ] Re-record the same script against the live testnet, with the
      contract address and explorer link on screen
- [ ] Replace the simulation video in the submission with the live one
