#!/bin/bash

# Bootstrap script for local NeuroSwarm development environment

echo "Setting up NeuroSwarm local dev environment..."

# Install dependencies
echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

echo "Installing Node.js..."
# Assume nvm or similar
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install node

echo "Installing Docker..."
# Instructions for Docker install

echo "Cloning repos..."
git clone https://github.com/brockhager/neuro-shared.git ../neuro-shared
git clone https://github.com/brockhager/neuro-program.git ../neuro-program
# etc.

echo "Setup complete. Run 'docker-compose up' to start services."