# Neuro-Infra

This repository contains infrastructure-related code and configurations for the NeuroSwarm project.

## Scope

- CI/CD workflows (GitHub Actions, pipelines)
- Deployment scripts (docker-compose, k8s manifests, Helm charts)
- Bootstrap tooling (local dev scripts, environment setup)
- Monitoring/observability configs (Prometheus, Grafana, logging)
- Security docs and threat models
- Node daemon skeleton, CLI, and config loader
- Networking and sync implementations
- Distribution and install setups
- Snapshots, bootstrap, and E2E tests

## Contributor Guidelines

- Follow the main NeuroSwarm contributing guidelines.
- Use Rust for daemon and CLI components.
- Use YAML/TOML for configurations.
- Ensure all code is well-documented and tested.

## Connections to Other Repos

- `neuro-shared`: Uses shared schemas and types.
- `neuro-program`: Integrates with Solana anchoring.
- `neuro-services`: Provides Gateway and Indexer APIs.
- `neuro-web`: Supports web UI deployments.
- Main `neuroswarm` repo: References docs and specs.