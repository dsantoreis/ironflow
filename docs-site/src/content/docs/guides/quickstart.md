---
title: Quickstart
---

```bash
cargo run -- \
  --input tests/fixtures/input.csv \
  --output out.json \
  --min-amount 10 \
  --category retail \
  --uppercase-name
```

```bash
cat out.json
```

Run quality gate:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```
