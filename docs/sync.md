# Sync Engine

The sync engine handles peer-to-peer data synchronization for the NeuroSwarm node, ensuring all nodes maintain a consistent catalog of manifests and attestations.

## Architecture

- **Initial Sync**: On startup, requests full catalog from connected peers
- **Resumable Sync**: Tracks last sync timestamp per peer to avoid re-downloading
- **Incremental Sync**: Periodically requests updates since last sync
- **Message Protocol**: Uses QUIC streams for sync messages (RequestCatalog, CatalogChunk, etc.)

## Components

- `SyncEngine`: Core sync logic, manages sync state and peer communication
- `SyncMessage`: Enum for sync protocol messages
- Integration with `Network` for message sending, `Storage` for persistence

## Observability

- Logs sync progress and errors
- Tracks sync timestamps in memory
- Future: Metrics for sync throughput, peer reliability

## Security

- Sync messages validated before storage
- Rate limiting on sync requests
- Banlist integration for malicious peers