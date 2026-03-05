# Agent Data Pipeline (Rust)

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
