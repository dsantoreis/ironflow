---
title: Operations
---

## Docker

```bash
docker build -t ironflow:latest .
```

## Kubernetes

```bash
kubectl apply -f k8s/pipeline-job.yaml
```

## CI

GitHub Actions runs format, lint, tests, release build, and coverage threshold checks on every push/PR.
