# Distribution & Installation

NeuroSwarm provides multiple deployment options for different environments and use cases. This guide covers deployment strategies for each node role (Gateway, Indexer, Validator) and includes production-ready cloud examples.

## Quick Start

- **[Getting Started](../../getting-started.md)** - Basic setup and prerequisites
- **[Development Guide](../../development.md)** - Development workflow and deployment
- **[Observability Setup](../neuro-services/docs/observability.md)** - Monitoring and metrics configuration

## Node Roles & Deployment

### Gateway Node

**Purpose:** API gateway for external applications, handles queries and authentication.

**Resource Requirements:**
- CPU: 1-2 cores
- Memory: 2-4 GB
- Storage: 50-100 GB (for local cache)

**Docker Deployment:**
```bash
docker run -d \
  --name neuroswarm-gateway \
  -p 3000:3000 \
  -e NODE_MODE=gateway \
  -e JWT_SECRET=your-secret \
  -v gateway-data:/app/data \
  ghcr.io/brockhager/neuro-services:latest
```

**Kubernetes (Gateway):**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: neuroswarm-gateway
spec:
  replicas: 2
  selector:
    matchLabels:
      app: neuroswarm-gateway
  template:
    metadata:
      labels:
        app: neuroswarm-gateway
    spec:
      containers:
      - name: gateway
        image: ghcr.io/brockhager/neuro-services:latest
        ports:
        - containerPort: 3000
        env:
        - name: NODE_MODE
          value: "gateway"
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: neuroswarm-secrets
              key: jwt-secret
        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 1000m
            memory: 2Gi
```

### Indexer Node

**Purpose:** Search indexing, lineage queries, and confidence scoring.

**Resource Requirements:**
- CPU: 2-4 cores
- Memory: 4-8 GB
- Storage: 200-500 GB (for full index)

**Docker Deployment:**
```bash
docker run -d \
  --name neuroswarm-indexer \
  -p 3001:3000 \
  -e NODE_MODE=indexer \
  -v indexer-data:/app/data \
  -v indexer-index:/app/index \
  ghcr.io/brockhager/neuro-services:latest
```

### Validator Node

**Purpose:** Consensus validation, attestation signing, and network security.

**Resource Requirements:**
- CPU: 4-8 cores
- Memory: 8-16 GB
- Storage: 500 GB+ (for full chain state)

**Docker Deployment:**
```bash
docker run -d \
  --name neuroswarm-validator \
  -p 8080:8080 \
  -e NODE_MODE=validator \
  -e SOLANA_RPC_URL=https://api.mainnet.solana.com \
  --mount type=bind,source=/host/keys,target=/app/keys \
  ghcr.io/brockhager/neuro-infra:latest
```

**Kubernetes (Validator):**
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: neuroswarm-validator
spec:
  serviceName: neuroswarm-validator
  replicas: 1
  selector:
    matchLabels:
      app: neuroswarm-validator
  template:
    metadata:
      labels:
        app: neuroswarm-validator
    spec:
      containers:
      - name: validator
        image: ghcr.io/brockhager/neuro-infra:latest
        ports:
        - containerPort: 8080
        env:
        - name: NODE_MODE
          value: "validator"
        - name: SOLANA_RPC_URL
          value: "https://api.mainnet.solana.com"
        volumeMounts:
        - name: validator-keys
          mountPath: /app/keys
        - name: validator-data
          mountPath: /app/data
        resources:
          requests:
            cpu: 2000m
            memory: 4Gi
          limits:
            cpu: 4000m
            memory: 8Gi
  volumeClaimTemplates:
  - metadata:
      name: validator-data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 500Gi
  - metadata:
      name: validator-keys
    spec:
      accessModes: ["ReadWriteOnce"]
      storageClassName: fast-ssd
      resources:
        requests:
          storage: 10Gi
```

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

### AWS EKS (Production)

**Gateway Node Cluster:**
```hcl
# Terraform for Gateway nodes
resource "aws_eks_cluster" "neuroswarm" {
  name     = "neuroswarm"
  version  = "1.28"
  vpc_config {
    subnet_ids = aws_subnet.private[*].id
  }
}

resource "aws_eks_node_group" "gateway" {
  cluster_name    = aws_eks_cluster.neuroswarm.name
  node_group_name = "gateway"
  instance_types  = ["t3.medium"]
  scaling_config {
    desired_size = 3
    max_size     = 10
    min_size     = 2
  }
}

# ALB Ingress for API access
resource "aws_lb" "gateway" {
  name               = "neuroswarm-gateway"
  internal           = false
  load_balancer_type = "application"
  subnets            = aws_subnet.public[*].id
}

resource "aws_lb_listener" "gateway" {
  load_balancer_arn = aws_lb.gateway.arn
  port              = "443"
  protocol          = "HTTPS"
  ssl_policy        = "ELBSecurityPolicy-2016-08"
  certificate_arn   = aws_acm_certificate.gateway.arn

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.gateway.arn
  }
}
```

**Validator Node (EC2 Auto Scaling):**
```hcl
resource "aws_launch_template" "validator" {
  name_prefix   = "neuroswarm-validator"
  image_id      = data.aws_ami.ubuntu.id
  instance_type = "c5.2xlarge"

  user_data = base64encode(templatefile("validator-init.sh", {
    solana_version = "1.16.0"
    node_mode      = "validator"
  }))
}

resource "aws_autoscaling_group" "validator" {
  desired_capacity    = 3
  max_size           = 5
  min_size           = 1
  launch_template {
    id      = aws_launch_template.validator.id
    version = "$Latest"
  }

  tag {
    key                 = "Name"
    value               = "neuroswarm-validator"
    propagate_at_launch = true
  }
}
```

### GCP GKE (Production)

**Multi-zone Gateway Deployment:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: neuroswarm-gateway
  namespace: neuroswarm
spec:
  replicas: 3
  selector:
    matchLabels:
      app: neuroswarm-gateway
  template:
    metadata:
      labels:
        app: neuroswarm-gateway
    spec:
      nodeSelector:
        cloud.google.com/gke-nodepool: gateway-pool
      containers:
      - name: gateway
        image: ghcr.io/brockhager/neuro-services:latest
        ports:
        - containerPort: 3000
        env:
        - name: NODE_MODE
          value: "gateway"
        resources:
          requests:
            cpu: 500m
            memory: 1Gi
          limits:
            cpu: 1000m
            memory: 2Gi
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: neuroswarm-gateway
  namespace: neuroswarm
  annotations:
    kubernetes.io/ingress.class: "gce"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - api.neuroswarm.io
    secretName: neuroswarm-tls
  rules:
  - host: api.neuroswarm.io
    http:
      paths:
      - path: /
        pathType: Required
        backend:
          service:
            name: neuroswarm-gateway
            port:
              number: 3000
```

### Azure AKS (Production)

**Validator StatefulSet with Managed Identity:**
```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: neuroswarm-validator
  namespace: neuroswarm
spec:
  serviceName: neuroswarm-validator
  replicas: 2
  selector:
    matchLabels:
      app: neuroswarm-validator
  template:
    metadata:
      labels:
        app: neuroswarm-validator
    spec:
      serviceAccountName: neuroswarm-validator
      nodeSelector:
        agentpool: validatorpool
      containers:
      - name: validator
        image: ghcr.io/brockhager/neuro-infra:latest
        ports:
        - containerPort: 8080
        env:
        - name: NODE_MODE
          value: "validator"
        - name: AZURE_CLIENT_ID
          valueFrom:
            secretKeyRef:
              name: azure-credentials
              key: client-id
        volumeMounts:
        - name: validator-keys
          mountPath: /app/keys
        - name: validator-data
          mountPath: /app/data
        resources:
          requests:
            cpu: 2000m
            memory: 4Gi
          limits:
            cpu: 4000m
            memory: 8Gi
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: neuroswarm-validator
  namespace: neuroswarm
  annotations:
    azure.workload.identity/client-id: "12345678-1234-1234-1234-123456789012"
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: neuroswarm-validator
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: neuroswarm-validator
subjects:
- kind: ServiceAccount
  name: neuroswarm-validator
  namespace: neuroswarm
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

### Monitoring Setup
- **[Observability Guide](../neuro-services/docs/observability.md)** - Complete monitoring stack setup with Prometheus/Grafana
- **Metrics Endpoints:** `/metrics` for Prometheus scraping
- **Health Checks:** `/health` for load balancer monitoring

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