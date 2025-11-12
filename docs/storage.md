# Storage Architecture

## Overview
The NeuroSwarm node uses SQLite for the catalog database and IPFS for artifact storage.

## Database Schema
- **manifests**: cid (TEXT PK), data (BLOB), timestamp (INTEGER)
- **attestations**: id (INTEGER PK), manifest_cid (TEXT FK), validator (TEXT), confidence (REAL), timestamp (INTEGER)

## IPFS Integration
- Pin/unpin artifacts for caching.
- Add/get data via IPFS API.

## Pruning Strategy
- Remove manifests older than X days.
- Unpin unused artifacts.

## CLI Usage
- `nsd catalog list`: List manifests
- `nsd catalog prune <days>`: Prune old entries
- `nsd catalog stats`: Show counts

## Indexing
- In-memory index for fast lookups by CID, node ID.
- Supports lineage queries.