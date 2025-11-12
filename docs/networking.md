# Networking Architecture

## Overview
The NeuroSwarm node uses a peer-to-peer networking model over QUIC for secure, low-latency communication.

## Components
- **DNS Seeds**: Bootstrap peers resolved from domain names.
- **Static Peers**: Manually configured peer addresses.
- **QUIC Transport**: Encrypted channels using Ed25519 keys.
- **Handshake**: Version and capability exchange.
- **Banlist**: Reputation-based peer management.

## Packet Formats
- Handshake: JSON { "node_id": "string", "version": "string" }

## CLI Usage
- `nsd peer add <addr>`: Add static peer
- `nsd peer list`: List connected peers
- `nsd peer remove <addr>`: Remove peer

## Architecture
Networking is modular, allowing extension to validator/gateway modes.