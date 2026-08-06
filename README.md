# liquid-witness

> `liquid-witness` is a live-chain companion for `liquid-proofpack`: it is designed to recompute Liquid/AMP asset supply and holder state from scoped watch-only evidence, then emit a CAB-compatible verdict without asking for spending keys or a master blinding key.

[![CI](https://github.com/vishnugovind10/liquid-witness/actions/workflows/ci.yml/badge.svg)](https://github.com/vishnugovind10/liquid-witness/actions/workflows/ci.yml)

## Problem

AMP enforces transfer controls and holder verification off-chain. Liquid Confidential Transactions blind amounts and asset types on-chain. That leaves an awkward verification gap: an allocator, auditor, or protocol reviewer needs stronger evidence than an issuer spreadsheet, but full key disclosure is too broad.

[`liquid-proofpack`](https://github.com/vishnugovind10/liquid-proofpack) defines the CAB v0.1 evidence format and offline structural verifier. `liquid-witness` is the live-backend side of that design. It keeps verdict logic, CAB serialization, and the LWK scan boundary separate so the claim comparison can be tested without network access.

## Architecture

```text
issuer claim / CAB bundle
        |
        v
+------------------+       +---------------------+
| witness-cli      | ----> | witness-core        |
| scan / verify    |       | verdict + CAB JSON  |
+------------------+       +---------------------+
        |
        v
+------------------+
| witness-lwk      |
| watch-only scan  |
+------------------+
        |
        v
Liquid testnet Electrum / Esplora
```

Workspace layout:

```text
crates/witness-core   pure verdict logic, recomputation, CAB bridge
crates/witness-lwk    only crate allowed to touch lwk_wollet/LWK scan boundaries
crates/witness-cli    witness binary
tests/fixtures        committed replay fixtures; no live network in CI
examples              DEMO CAB artifact and liquid-proofpack bundle workflow
```

## Scope & Honesty

| Verdict | Meaning | Exit |
| --- | --- | --- |
| `VERIFIED` | A complete live or replay observation matched the issuer claim. | `0` |
| `MISMATCH` | A complete observation contradicted the issuer claim. | `1` |
| `INCOMPLETE` | The scan ran but did not cover the full claim. | `2` |
| `DEMO` | The command ran against committed fixtures only. It is not pass/fail evidence. | `3` |

v0.1.0 ships a compiled LWK boundary and a fixture-backed demonstration path. It does not claim a reproducible public AMP testnet `VERIFIED` result because no public asset descriptor was validated during this build.

## Install

```bash
cargo install --path crates/witness-cli
```

## Quickstart

Run the committed fixture path:

```bash
cargo run -p witness-cli -- verify \
  --claim tests/fixtures/demo-issuer-claim.json \
  --asset-id abababababababababababababababababababababababababababababababab \
  --descriptor "ct(elwpk([00000000/84h/1h/0h]tpub-demo/0/*))" \
  --fixture tests/fixtures/demo-observed-state.json \
  --out examples/testnet-amp-scan/output.cab
```

Expected boundary:

```text
exit code 3
verdict DEMO
```

Inspect a CAB-compatible bundle:

```bash
cargo run -p witness-cli -- verify-bundle --cab examples/testnet-amp-scan/output.cab
```

## What This Demonstrates

- CAB-compatible JSON output with stable verdict and exit-code semantics.
- Pure recomputation logic for total supply and holder-category distribution.
- A watch-only descriptor guard that rejects obvious signer material.
- An isolated `witness-lwk` crate where real Liquid/LWK scan work belongs.
- CI-safe replay fixtures that do not make live-chain claims.

## What This Does Not Demonstrate

- No mainnet support in v0.1.0.
- No validated public AMP2 scan depth.
- No Jade or hardware-signer path; this tool does not sign.
- No audit opinion, legal assurance, investment recommendation, or production attestation.
- No `VERIFIED` verdict unless a complete Liquid/Electrum or replay observation actually supports it.

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

## Sources

- Blockstream LWK and `lwk_wollet` are the intended live scan boundary.
- `liquid-proofpack` is the CAB format companion this repository targets.
- Liquid Confidential Transactions and AMP define the protocol context; this repository only verifies scoped evidence it is given.

## Citation

See [CITATION.cff](CITATION.cff).
