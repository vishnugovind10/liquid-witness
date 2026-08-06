# SPEC

## Inputs

`liquid-witness` accepts either:

- an issuer claim JSON file with `asset_id`, `total_supply`, and holder-category totals, or
- a CAB-compatible bundle emitted by `liquid-proofpack` or a prior `liquid-witness` run.

The scan path also requires a watch-only CT descriptor. Runtime code rejects descriptor strings containing obvious private-key markers such as `xprv`, `tprv`, or `seed`.

## Recompute Algorithm

1. Confirm the claimed asset ID equals the observed asset ID.
2. Preserve `DEMO` if the observation came from committed fixtures.
3. Return `INCOMPLETE` if the observation did not cover the full claim.
4. Compare claimed total supply with observed total supply.
5. Normalize holder categories by summing duplicate category labels.
6. Compare every claimed and observed category.
7. Return `VERIFIED` only if the complete observation has no differences.

## Verdict State Machine

```text
fixture observation
        -> DEMO

live/replay observation + incomplete coverage
        -> INCOMPLETE

complete observation + any supply/category difference
        -> MISMATCH

complete observation + no differences
        -> VERIFIED
```

Exit codes are stable:

```text
0 VERIFIED
1 MISMATCH
2 INCOMPLETE
3 DEMO
```

## CAB Bridge

The v0.1 output is CAB-compatible JSON:

```json
{
  "cab_version": "0.1-compatible",
  "subject": {
    "asset_id": "...",
    "network": "testnet"
  },
  "claim": {
    "total_supply": 1000,
    "claim_sha256": "..."
  },
  "observed": {},
  "verdict": "DEMO",
  "reasons": [],
  "evidence": {
    "mode": "DEMO",
    "source": "liquid-witness"
  },
  "generated_at": "2026-08-06T00:00:00Z"
}
```

`claim_sha256` is computed over the canonical JSON serialization used by `witness-core`.
