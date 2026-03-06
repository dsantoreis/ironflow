---
title: Getting Started
description: Build, test, and run Ironflow locally in under 2 minutes.
---

## Prerequisites

- Rust toolchain (stable)
- `cargo` available in your shell

## Run in 3 commands

```bash
cargo build
cargo test --all
cargo run -- --input tests/fixtures/input.csv --output out.json --min-amount 10 --category retail --uppercase-name
```

Inspect the output:

```bash
cat out.json
```

## Quality gate

Before opening a PR:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```
