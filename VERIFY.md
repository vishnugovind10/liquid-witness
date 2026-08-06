# VERIFY

Run from the repository root.

## Local Gates

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

## Demo CAB Artifact

```bash
cargo run -p witness-cli -- verify \
  --claim tests/fixtures/demo-issuer-claim.json \
  --asset-id abababababababababababababababababababababababababababababababab \
  --descriptor "ct(elwpk([00000000/84h/1h/0h]tpub-demo/0/*))" \
  --fixture tests/fixtures/demo-observed-state.json \
  --out examples/testnet-amp-scan/output.cab
```

The command exits with code `3` because the artifact is `DEMO`.

Inspect it:

```bash
cargo run -p witness-cli -- verify-bundle --cab examples/testnet-amp-scan/output.cab
```

## Live Boundary Check

Without `--fixture`, v0.1.0 returns `INCOMPLETE` rather than pretending to have scanned a complete public AMP asset:

```bash
cargo run -p witness-cli -- scan \
  --asset-id abababababababababababababababababababababababababababababababab \
  --descriptor "ct(elwpk([00000000/84h/1h/0h]tpub-demo/0/*))"
```
