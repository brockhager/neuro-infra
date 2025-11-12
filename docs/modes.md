# Operating Modes

NeuroSwarm nodes can operate in specialized roles, allowing the network to scale horizontally while maintaining efficiency.

## Modes

### Validator Mode
**Purpose**: Anchors manifests to Solana blockchain, verifies attestations, participates in consensus.

**Components**:
- Network (peer connectivity)
- Storage (catalog persistence)
- Anchor (Solana verification)
- Sync (data synchronization)

**Use Case**: Dedicated consensus nodes for trust and finality.

**Config**:
```yaml
node:
  mode: validator
```

### Gateway Mode
**Purpose**: Exposes API surface for external services, handles queries and metrics.

**Components**:
- Network (peer connectivity)
- Storage (catalog access)
- API Server (HTTP/GraphQL endpoints)

**Use Case**: Public API nodes for applications and services.

**Config**:
```yaml
node:
  mode: gateway
```

### Indexer Mode
**Purpose**: Builds and serves advanced search, lineage tracing, and confidence scoring.

**Components**:
- Network (peer connectivity)
- Storage (catalog access)
- Index (search engine)
- Search API (query endpoints)

**Use Case**: Specialized search nodes for data discovery.

**Config**:
```yaml
node:
  mode: indexer
```

### Full Mode (Default)
**Purpose**: Runs all components for development or small deployments.

**Components**: All subsystems active.

**Use Case**: Development, testing, or small-scale deployments.

**Config**:
```yaml
node:
  mode: full
```

## Configuration

### Config File
Set mode in `~/.neuroswarm/ns.conf`:

```yaml
node:
  mode: validator  # validator | gateway | indexer | full
```

### CLI Override
Override config with CLI flag:

```bash
nsd --mode validator start
```

## Observability

- **Mode-specific metrics**: Each mode exposes relevant performance indicators
- **Structured logging**: Logs indicate active role and subsystem lifecycle
- **Health checks**: Mode-appropriate health endpoints

## Deployment Examples

### Validator Cluster
```bash
# Multiple validators for consensus
nsd --mode validator start
```

### API Gateway Pool
```bash
# Load-balanced API nodes
nsd --mode gateway start
```

### Distributed Indexers
```bash
# Specialized search nodes
nsd --mode indexer start
```

### Development Node
```bash
# Full functionality locally
nsd --mode full start
```

## Benefits

- **Horizontal Scaling**: Different roles can scale independently
- **Resource Efficiency**: Nodes only run necessary components
- **Specialization**: Optimized performance for specific functions
- **Operational Flexibility**: Easy deployment of role-specific infrastructure