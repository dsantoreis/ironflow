---
title: API Reference
description: CLI contract for Ironflow pipeline runs.
---

## Binary

```bash
cargo run -- [options]
```

## Options

| Flag | Type | Required | Description |
|---|---|---:|---|
| `--input` | path | yes | Path to source CSV file |
| `--output` | path | yes | Path for generated JSON file |
| `--min-amount` | number | no | Keep rows with amount >= value |
| `--category` | string | no | Keep rows matching category |
| `--uppercase-name` | boolean | no | Uppercase the `name` field before export |
| `--format` | string | no | Output format (default: `json`) |

## Example

```bash
cargo run -- \
  --input tests/fixtures/input.csv \
  --output out.json \
  --min-amount 10 \
  --category retail \
  --uppercase-name
```
