# LIMITATIONS

v0.1.0 is intentionally conservative.

## Shipped Boundary

- Default public artifact is `DEMO`, backed by committed fixtures.
- CI does not perform live network calls.
- `witness-lwk` contains the LWK boundary, but no release claim depends on an unrecorded live testnet round trip.

## Not Covered

- Mainnet Liquid verification.
- Public reproducible AMP0 or AMP2 testnet asset verification.
- Multi-asset batch scans.
- Deep AMP2 holder-state extraction.
- Jade or hardware-signer workflows. This tool is watch-only and does not sign.
- Production audit, legal, regulatory, investment, or redemption assurance.

## Promotion Rule

A future release may use `VERIFIED` only when a complete scan against Liquid testnet or mainnet evidence is reproducible from committed instructions or recorded fixtures.
