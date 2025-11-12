# CLI Commands

The NeuroSwarm daemon (`nsd`) provides a comprehensive command-line interface for node operators to manage and query the node without API calls.

## Commands

### nsd status
Shows overall node health and statistics.

```bash
nsd status
```

Output:
```
Node Status:
- Version: 0.1.0
- Uptime: 2h 30m
- Peers: 5 connected
- Sync Progress: 85%
- Anchoring Latency: 120ms
```

### nsd catalog
Manage the local manifest catalog.

#### nsd catalog list
List all manifests in the catalog.

```bash
nsd catalog list
```

#### nsd catalog prune <days>
Remove manifests older than specified days.

```bash
nsd catalog prune 30
```

#### nsd catalog stats
Show catalog statistics.

```bash
nsd catalog stats
```

Output:
```
Manifests: 150, Attestations: 300
```

### nsd peer
Manage peer connections.

#### nsd peer add <addr>
Add a static peer.

```bash
nsd peer add 127.0.0.1:8080
```

#### nsd peer list
List connected peers.

```bash
nsd peer list
```

#### nsd peer remove <addr>
Remove a peer.

```bash
nsd peer remove 127.0.0.1:8080
```

### nsd index
Query the local index directly.

#### nsd index search [options]
Search manifests.

```bash
nsd index search --query "neural" --tag ai
```

#### nsd index lineage <cid>
Show provenance lineage.

```bash
nsd index lineage QmTest123
```

#### nsd index confidence <cid>
Show confidence score.

```bash
nsd index confidence QmTest123
```

### nsd anchor verify <cid> <creator>
Verify manifest against Solana blockchain.

```bash
nsd anchor verify QmTest123 CreatorPubkey
```

## Design Principles

- **Consistency**: Commands mirror Gateway API functionality
- **Structured Output**: JSON or table format for scripting
- **Modularity**: Easy to add new commands
- **Observability**: All commands logged for auditability

## Integration

- Direct access to local storage, network, index, and anchor components
- No network calls required for local operations
- Future: Remote CLI for managing distributed nodes

## Examples

```bash
# Check node health
nsd status

# Clean old data
nsd catalog prune 7

# Find AI models
nsd index search --tag ai

# Verify provenance
nsd anchor verify QmAbc123 CreatorKey
```