# Blockstream Adoption Runbook

This repo is designed to make one narrow claim credible:

`liquid-witness` can independently recompute a scoped Liquid testnet AMP asset observation with Blockstream LWK, without a spending key and without committing view-sensitive descriptor material.

## Adoption Path

1. Generate a throwaway Liquid testnet wallet.
2. Request an Issuer-Tracked AMP demo asset from `amp-demo.blockstream.com`.
3. Export the watch-only CT descriptor into a gitignored `.env`.
4. Confirm the txid on a Liquid testnet explorer.
5. Run `witness verify --live` against `elements-testnet.blockstream.info:50002`.
6. Commit only the CAB output if it contains no descriptor, xpub, xprv, mnemonic, seed, or unredacted GAID.

## What Blockstream Can Review Quickly

- LWK boundary: [crates/witness-lwk/src/scan.rs](../crates/witness-lwk/src/scan.rs)
- CAB serialization allowlist: [crates/witness-core/src/cab_bridge.rs](../crates/witness-core/src/cab_bridge.rs)
- Descriptor handling: [SECURITY.md](../SECURITY.md)
- Reproduction steps: [VERIFY.md](../VERIFY.md)
- Current limitations: [LIMITATIONS.md](../LIMITATIONS.md)

## Evidence Boundary

Do not tag a `v0.2.0` release or draft outreach copy until `examples/testnet-amp-scan/live-output.cab` exists and shows a real live `VERIFIED`, `MISMATCH`, or `INCOMPLETE` verdict from a throwaway amp-demo run.

The useful outcome is not necessarily a match. A `MISMATCH` or `INCOMPLETE` result is still signal if it is honestly explained and reproducible.

## Public Sharing Checklist

- Link to a versioned release, not the repo root.
- State the exact verdict and asset ID.
- Include the independent txid sanity check.
- State that the CT descriptor was never committed.
- State that mainnet, AMP2 depth, and batch scans remain open.
- Avoid audit, certification, regulatory, investment, or production-readiness claims.
