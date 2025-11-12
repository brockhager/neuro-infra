#!/bin/bash

# Bootstrap script for local NeuroSwarm development environment

echo "Setting up NeuroSwarm local dev environment..."

# Install dependencies
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

echo "Installing Node.js..."
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install node

echo "Installing Docker..."
# Instructions for Docker install (assume installed)

echo "Cloning repos..."
git clone https://github.com/brockhager/neuro-shared.git ../neuro-shared
git clone https://github.com/brockhager/neuro-program.git ../neuro-program
git clone https://github.com/brockhager/neuro-services.git ../neuro-services
git clone https://github.com/brockhager/neuro-web.git ../neuro-web

echo "Starting Solana test validator..."
solana-test-validator --reset &

echo "Starting IPFS daemon..."
ipfs daemon &

echo "Starting services with Docker Compose..."
cd ../neuro-infra/infra
docker-compose up -d

echo "Setup complete. Access services at localhost ports."