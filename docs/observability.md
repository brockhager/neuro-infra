# Observability

## Monitoring
- Prometheus for metrics collection
- Grafana for dashboards
- Alerting on node health, sync status, and errors

## Logging
- Structured logs with tracing IDs
- Centralized logging via ELK stack or similar
- Log levels: DEBUG, INFO, WARN, ERROR

## Metrics
- Node uptime and connectivity
- Sync progress and latency
- API response times and error rates
- Resource usage (CPU, memory, disk)
- Peer events: connections, disconnections, handshakes
- Catalog size, cache hits/misses

## Tracing
- Distributed tracing for request flows
- Jaeger or OpenTelemetry integration

## Accessing Dashboards
- **Prometheus**: http://localhost:9090 (Docker) or LoadBalancer service (K8s)
- **Grafana**: http://localhost:3000 (admin/admin) (Docker) or LoadBalancer service (K8s)
- **Jaeger**: http://localhost:16686 (Docker) or LoadBalancer service (K8s)

## Dashboard Setup
1. In Grafana, add Prometheus as a data source (URL: http://prometheus:9090).
2. Import dashboards from `infra/grafana/dashboards/` (e.g., gateway.json for API metrics).
3. Configure alerts for high error rates or latency.
4. Monitor peer logs for connection events.