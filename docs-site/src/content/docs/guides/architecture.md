---
title: Architecture
---

Pipeline stages:

1. **Ingest**: CSV records loaded with schema validation
2. **Transform**: Filter + normalize + uppercase transform (optional)
3. **Export**: Stable sorted JSON output

The core transformation logic lives in `src/lib.rs`; CLI argument parsing and IO orchestration lives in `src/main.rs`.
