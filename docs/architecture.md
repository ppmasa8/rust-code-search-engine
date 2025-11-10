# Architecture Overview

This workspace is intentionally split into small crates so that each component can evolve independently.

- **Ingestion path**: `crawler` collects documents, `indexer` normalizes them, and `storage` keeps canonical metadata.
- **Serving path**: `apps/api` exposes HTTP endpoints, calling into `search`, which orchestrates `query`, `indexer`, and `embedding` crates.
- **Offline compute**: `apps/worker` and `apps/cli` call into shared crates for maintenance or batch jobs.

The repository favors explicit specs (`specs/`) and fixtures (`tests/fixtures`) so LLM-based agents can reason over requirements.
