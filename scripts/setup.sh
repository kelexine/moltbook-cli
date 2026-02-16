#!/bin/bash
# Moltbook CLI Setup Script
# Author: @kelexine

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║     Moltbook CLI Setup                 ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust is not installed. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed successfully${NC}"
else
    echo -e "${GREEN}✓ Rust is already installed${NC}"
fi

# Build and Install
echo -e "\n${YELLOW}Building and Installing Moltbook CLI...${NC}"
cargo install --path .

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build and Installation successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

# Run Interactive Init
echo -e "\n${YELLOW}Initializing Configuration...${NC}"
moltbook-cli init

echo -e "\n${GREEN}Setup Complete! 🦞${NC}"
echo -e "Try running: ${YELLOW}moltbook-cli feed${NC}"