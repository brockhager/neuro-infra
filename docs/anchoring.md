# Solana Anchoring

The anchoring system verifies NeuroSwarm manifests and attestations against the Solana blockchain, providing decentralized trust and provenance.

## Architecture

- **On-Chain Verification**: Queries Solana RPC for manifest existence and attestation data
- **Provenance Caching**: Stores verification results locally to reduce RPC calls
- **Sync Integration**: Verifies manifests during peer sync before storage
- **Event Monitoring**: Future: Listen to Solana events for real-time updates

## Components

- `Anchor`: Solana client wrapper for program queries
- Provenance cache in SQLite: finalized status, attestation count, tx signature, slot
- Integration with sync engine for verification gates

## Security

- PDA-based account derivation for deterministic manifest addresses
- Confidence thresholds for attestation validation
- Finalization requires majority validator consensus

## Observability

- Logs verification successes/failures
- Metrics for RPC call latency and success rates
- Cache hit ratios for provenance queries