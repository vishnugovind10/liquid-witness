## Summary

-

## Evidence Boundary

- [ ] Does not commit CT descriptors, xpub/xprv material, mnemonics, seeds, or unredacted GAIDs
- [ ] Does not promote fixture output to `VERIFIED`
- [ ] Keeps live network calls out of default CI
- [ ] Updates docs when verdict semantics or live-scan behavior changes

## Verification

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `cargo check -p witness-cli --features live-lwk`
