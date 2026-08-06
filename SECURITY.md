# SECURITY

## Watch-Only Scope

`liquid-witness` is designed for watch-only Liquid/AMP verification.

- It cannot move funds.
- It does not need a spending key.
- It rejects descriptor strings that appear to contain obvious private-key material.
- It cannot unblind outputs outside the descriptor scope it is given.

## Reporting

Please report security issues through GitHub private vulnerability reporting when available, or by opening a minimal issue that does not disclose exploitable details.

## Non-Goals

This repository does not provide custody, signing, wallet recovery, investment advice, audit assurance, or regulatory certification.
