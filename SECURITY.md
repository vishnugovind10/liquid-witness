# SECURITY

## Watch-Only Scope

`liquid-witness` is designed for watch-only Liquid/AMP verification.

- It cannot move funds.
- It does not need a spending key.
- It rejects descriptor strings that appear to contain obvious private-key material.
- It cannot unblind outputs outside the descriptor scope it is given.

## Descriptor & Key Handling

`liquid-witness` is watch-only end to end. It never imports, derives, or
requires a spending key. The one input that is still view-sensitive is the
CT blinding descriptor used to scan and unblind a specific wallet's
confidential outputs. This is not a spending credential, but it does
reveal true amounts and asset types for that address, so it is handled as a
secret:

- Never passed as a bare CLI argument in documented examples; read from a
  gitignored `.env` instead
- Never included in committed CAB artifacts
- All example and reproduction runs in this repo use a throwaway testnet
  wallet generated solely for that run, holding no value beyond the demo
  asset itself

## Reporting

Please report security issues through GitHub private vulnerability reporting when available, or by opening a minimal issue that does not disclose exploitable details.

## Non-Goals

This repository does not provide custody, signing, wallet recovery, investment advice, audit assurance, or regulatory certification.
