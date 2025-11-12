# Distribution & Installation

NeuroSwarm provides multiple deployment options for different environments and use cases.

## Docker

### Local Development
```bash
# Clone repositories
git clone https://github.com/brockhager/neuro-infra.git
cd neuro-infra

# Build and run
docker build -t neuroswarm/node .
docker run -p 8080:8080 neuroswarm/node
```

### Docker Compose (Full Stack)
```bash
cd neuro-services
docker-compose up
```

### Pre-built Images
```bash
# Pull from GitHub Container Registry
docker pull ghcr.io/brockhager/neuro-infra:latest
docker pull ghcr.io/brockhager/neuro-services:latest
```

## Kubernetes (Helm)

### Install Helm Chart
```bash
# Add repository (when published)
helm repo add neuroswarm https://charts.neuroswarm.io
helm repo update

# Install with default values
helm install neuroswarm neuroswarm/neuroswarm

# Install with custom mode
helm install neuroswarm neuroswarm/neuroswarm --set mode=validator
```

### Custom Values
```yaml
# values.yaml
replicaCount: 3
mode: validator

resources:
  limits:
    cpu: 2000m
    memory: 2Gi
  requests:
    cpu: 1000m
    memory: 1Gi
```

## Bare Metal

### Systemd Service
```bash
# Download binary
wget https://github.com/brockhager/neuro-infra/releases/download/v0.1.0/nsd-linux-amd64
chmod +x nsd-linux-amd64

# Create service file
sudo tee /etc/systemd/system/neuroswarm.service > /dev/null <<EOF
[Unit]
Description=NeuroSwarm Node
After=network.target

[Service]
Type=simple
User=neuroswarm
ExecStart=/usr/local/bin/nsd start
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Enable and start
sudo systemctl enable neuroswarm
sudo systemctl start neuroswarm
```

### Configuration
Create `/etc/neuroswarm/ns.conf`:

```yaml
network:
  listen_addr: "0.0.0.0:8080"
  max_peers: 100

node:
  mode: full

solana:
  rpc_url: "https://api.mainnet.solana.com"
```

## Cloud Deployment

### AWS
```hcl
# Terraform example
resource "aws_ecs_cluster" "neuroswarm" {
  name = "neuroswarm"
}

resource "aws_ecs_service" "validator" {
  name            = "validator"
  cluster         = aws_ecs_cluster.neuroswarm.id
  task_definition = aws_ecs_task_definition.validator.arn
  desired_count   = 3
}
```

### GCP
```yaml
# Cloud Run example
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: neuroswarm-gateway
spec:
  template:
    spec:
      containers:
      - image: ghcr.io/brockhager/neuro-services:latest
        ports:
        - containerPort: 3000
```

## Security

### Image Signing
All Docker images are signed with Cosign:

```bash
# Verify signature
cosign verify ghcr.io/brockhager/neuro-infra:latest
```

### RBAC
For Kubernetes deployments, use service accounts with minimal permissions:

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: neuroswarm
  namespace: neuroswarm
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: neuroswarm-role
  namespace: neuroswarm
rules:
- apiGroups: [""]
  resources: ["pods", "services"]
  verbs: ["get", "list", "watch"]
```

### Secrets Management
Use Kubernetes secrets or external secret managers:

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: neuroswarm-secrets
type: Opaque
data:
  solana-key: <base64-encoded-key>
```

## Observability

### Monitoring
```bash
# Prometheus metrics
curl http://localhost:8080/metrics

# Health check
curl http://localhost:8080/health
```

### Logging
Structured logs are output to stdout/stderr for containerized deployments.

### Tracing
Jaeger integration for distributed tracing:

```yaml
# Enable tracing
env:
  - name: JAEGER_AGENT_HOST
    value: "jaeger-agent"
  - name: JAEGER_AGENT_PORT
    value: "6831"
```

## Bootstrap

### Initial Peers
For new nodes, configure bootstrap peers:

```yaml
network:
  static_peers:
    - "node1.neuroswarm.io:8080"
    - "node2.neuroswarm.io:8080"
```

### Snapshot Restore
```bash
# Download and restore snapshot
wget https://snapshots.neuroswarm.io/latest.tar.gz
tar -xzf latest.tar.gz -C /var/lib/neuroswarm
nsd start
```

## Multi-Architecture Support

Images are built for:
- `linux/amd64` (Intel/AMD)
- `linux/arm64` (ARM)

Use `docker buildx` for local multi-arch builds:

```bash
docker buildx build --platform linux/amd64,linux/arm64 -t neuroswarm/node .
```