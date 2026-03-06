---
title: Architecture
description: How Ironflow turns CSV ingest into deterministic JSON output.
---

## Pipeline stages

1. **Ingest**: read CSV rows and validate base schema
2. **Transform**: apply filters (`--min-amount`, `--category`) and optional name normalization (`--uppercase-name`)
3. **Export**: write stable, sorted JSON for deterministic downstream consumption

## Runtime components

- `src/main.rs`: CLI parsing, input/output handling, orchestration
- `src/lib.rs`: transformation and filtering logic
- `tests/`: unit + integration coverage for CSV to JSON behavior

## Deploy surfaces

- Local binary (`cargo run`)
- Docker image (`Dockerfile`)
- Kubernetes Job (`k8s/pipeline-job.yaml`)
