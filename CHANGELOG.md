# CHANGELOG

## Unreleased

- Added feature-gated LWK Electrum scan plumbing for Liquid testnet.
- Added live-capture CLI metadata for txid and redacted GAID.
- Moved CT descriptor handling to `.env` / `WITNESS_CT_DESCRIPTOR` for documented flows.
- Removed descriptor scope from CAB serialization and added a regression test for descriptor leakage.
- Added CAB replay tests for committed demo output and future live output.
- Added Blockstream adoption runbook, live-scan issue template, and PR evidence checklist.
- Kept `v0.2.0` unreleased until the amp-demo manual inputs and real `live-output.cab` artifact exist.

## v0.1.0 - 2026-08-06

- Initial public release with fixture-backed `DEMO` CAB output.
- Added pure verdict logic, CAB-compatible serialization, CLI commands, docs, and CI.
