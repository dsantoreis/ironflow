---
title: Deployment
description: Run Ironflow in Docker and Kubernetes.
---

## Docker

Build image:

```bash
docker build -t ironflow:latest .
```

Run with mounted fixture data:

```bash
docker run --rm -v $(pwd)/tests/fixtures:/data ironflow:latest \
  --input /data/input.csv --output /data/out.json --min-amount 10 --uppercase-name
```

## Docker Compose

```bash
docker compose up --build
```

## Kubernetes

Apply the job manifest:

```bash
kubectl apply -f k8s/pipeline-job.yaml
```

## CI and docs deploy

- CI workflow validates fmt, clippy, tests, build, and coverage >= 80%
- Docs workflow builds Starlight site and publishes to GitHub Pages
