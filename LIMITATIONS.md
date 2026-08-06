# LIMITATIONS

v0.1.0 is intentionally conservative.

## Shipped Boundary

- Default committed artifact remains `DEMO`, backed by committed fixtures.
- `live-output.cab` is intentionally absent until amp-demo manual inputs have been captured.
- CI does not perform live network calls.
- `witness-lwk` contains a feature-gated LWK Electrum path for Liquid testnet capture.

## Not Covered

- Mainnet Liquid verification.
- Public reproducible AMP0 verification until `examples/testnet-amp-scan/live-output.cab` is committed from amp-demo.
- AMP2 testnet asset verification.
- Multi-asset batch scans.
- Deep AMP2 holder-state extraction.
- Jade or hardware-signer workflows. This tool is watch-only and does not sign.
- Production audit, legal, regulatory, investment, or redemption assurance.

## Promotion Rule

A future release may use `VERIFIED` only when a complete scan against Liquid testnet or mainnet evidence is reproducible from committed instructions or recorded fixtures. Do not tag v0.2.0 until the amp-demo asset ID, txid, claimed amount, and live CAB artifact are present.
