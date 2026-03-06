# Agent Data Pipeline (Rust)

[![CI](../../actions/workflows/ci.yml/badge.svg)](#) [![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](./LICENSE)


High-performance **ingest → transform → export** CLI for operational data flows.

Built for teams that need a fast, predictable pipeline executable in CI, containers, or Kubernetes.

## Why this project

- **Fast by default**: parallel transformations with Rayon.
- **Production-friendly**: Docker multi-stage build + Kubernetes job manifest.
- **Automation-ready**: GitHub Actions CI (fmt, clippy, test, release build).
- **Composable CLI**: scriptable flags for filtering and transformation.

## Quick start

```bash
cargo run -- \
  --input tests/fixtures/input.csv \
  --output out.json \
  --min-amount 10 \
  --category retail \
  --uppercase-name
```

Output:
```bash
pipeline complete: 2 records
```

## CLI

```bash
agent-data-pipeline-rust \
  --input <file.csv> \
  --output <file.json> \
  [--min-amount <f64>] \
  [--category <string>] \
  [--uppercase-name] \
  [--format json]
```

## Development

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo bench --bench pipeline_bench
```

## Container

```bash
docker build -t agent-data-pipeline-rust .
docker run --rm -v $(pwd)/tests/fixtures:/data agent-data-pipeline-rust \
  --input /data/input.csv --output /data/out.json --min-amount 10 --uppercase-name
```

Or with compose:

```bash
docker compose up --build
```

## Kubernetes

Manifest: `k8s/pipeline-job.yaml`

```bash
kubectl apply -f k8s/pipeline-job.yaml
```

## Quality

- Unit tests (`src/lib.rs`)
- Integration CLI test (`tests/integration_cli.rs`)
- Criterion benchmark (`benches/pipeline_bench.rs`)

## Roadmap

- Streaming mode (stdin/stdout)
- Pluggable transforms (WASM)
- Parquet export
- Observability metrics (OpenTelemetry)


## Conversion Standard

### Hero
Production-ready solution for a concrete business problem with measurable outcome.

### Problem
Describe the pain with one sentence and a real operator context.

### Demo
Add a GIF at `docs/assets/demo.gif` and reference it here.

### Quickstart (3 commands)
```bash
make setup || pnpm install || npm install
make test || pnpm test || npm test
make run || pnpm dev || npm run dev
```

### Architecture
Document API, workers, and storage in `docs/architecture.md`.

### Results
Add benchmark, latency, throughput, or conversion impact.

### Roadmap
Include 30-day and 90-day milestones.

### CTA
If this helps, star the repo and open an issue with your use case.


## Docs

- Local docs site config: `mkdocs.yml`
- Entry point: `docs/index.md`
