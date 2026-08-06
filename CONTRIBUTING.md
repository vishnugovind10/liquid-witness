# CONTRIBUTING

Contributions should preserve the evidence boundary:

- Do not promote fixture or demo output to `VERIFIED`.
- Keep all LWK-specific code inside `crates/witness-lwk`.
- Add tests for verdict, CAB, and descriptor behavior when changing core logic.
- Keep CI free of live network requirements unless the workflow is explicitly marked as live and optional.

Before opening a PR:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
