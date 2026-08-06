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

Without `--fixture` or `--live`, the command returns `INCOMPLETE` rather than pretending to have scanned a complete public AMP asset:

```bash
cargo run -p witness-cli -- scan \
  --asset-id abababababababababababababababababababababababababababababababab \
  --descriptor "ct(elwpk([00000000/84h/1h/0h]tpub-demo/0/*))"
```

## Live Testnet Capture Procedure

Manual prerequisites:

1. Create a Liquid testnet wallet in Blockstream Green or with LWK tooling.
2. Create a Managed Assets Account and record the GAID.
3. Use `amp-demo.blockstream.com` to request the Issuer-Tracked demo asset.
4. Record the asset ID, receiving CT address, claimed amount, txid, and watch-only CT descriptor.
5. Confirm the txid on a Liquid testnet explorer before running `liquid-witness`.

Create a claim file matching the scoped amount you expect the descriptor to observe:

```json
{
  "asset_id": "<amp-demo-asset-id>",
  "total_supply": 1000,
  "holders": [
    {
      "category": "descriptor-scope",
      "amount": 1000
    }
  ]
}
```

Capture the live CAB:

```bash
cargo run -p witness-cli --features live-lwk -- verify --live \
  --claim path/to/amp-demo-claim.json \
  --asset-id <amp-demo-asset-id> \
  --descriptor "<watch-only-ct-descriptor>" \
  --network testnet \
  --txid <explorer-confirmed-txid> \
  --gaid-redacted "<gaid-prefix>...<gaid-suffix>" \
  --out examples/testnet-amp-scan/live-output.cab
```

Only commit `live-output.cab` after the command has genuinely scanned `elements-testnet.blockstream.info:50002` and the output shows `LIVE_OR_REPLAY` evidence.
