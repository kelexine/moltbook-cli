#!/bin/bash
# Moltbook CLI Setup Script
# Author: @kelexine
# Version: 0.6.2

set -e

# Colors & Formatting
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
BOLD='\033[1m'
DIM='\033[2m'
NC='\033[0m'

clear
echo -e "${CYAN}${BOLD}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}${BOLD}║             Moltbook CLI Guided Onboarding               ║${NC}"
echo -e "${CYAN}${BOLD}╚══════════════════════════════════════════════════════════╝${NC}"
echo -e "${DIM}A production-grade client for the social network for AI agents${NC}"
echo ""

# 1. Dependency Check
echo -e "${BOLD}[1/4] Checking Dependencies...${NC}"

check_cmd() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}✗ $1 is not installed.${NC}"
        return 1
    else
        echo -e "${GREEN}✓ $1 is ready${NC}"
        return 0
    fi
}

# Check/Install Rust
if ! check_cmd "cargo"; then
    echo -e "${YELLOW}! Rust is required. Attempting to install...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust installed via rustup${NC}"
fi

check_cmd "git" || { echo -e "${RED}Please install git and try again.${NC}"; exit 1; }
check_cmd "curl" || { echo -e "${RED}Please install curl and try again.${NC}"; exit 1; }

# 2. Build & Install
echo -e "\n${BOLD}[2/4] Building Moltbook CLI...${NC}"
echo -e "${DIM}This may take a minute depending on your system...${NC}"

if cargo install --path . &> /dev/null; then
    echo -e "${GREEN}✓ Moltbook CLI installed to ~/.cargo/bin/moltbook-cli${NC}"
else
    echo -e "${RED}✗ Build failed. Check your Rust installation.${NC}"
    exit 1
fi

# 3. Path Integration
echo -e "\n${BOLD}[3/4] Shell Integration...${NC}"
if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
    echo -e "${YELLOW}! ~/.cargo/bin is not in your PATH.${NC}"
    echo -e "To run 'moltbook-cli' from anywhere, add this to your .bashrc or .zshrc:"
    echo -e "${CYAN}export PATH=\"\$HOME/.cargo/bin:\$PATH\"${NC}"
else
    echo -e "${GREEN}✓ CLI is correctly available in your PATH${NC}"
fi

# 4. Configuration Choice
echo -e "\n${BOLD}[4/4] Final Configuration${NC}"
echo -e "How would you like to start?"
echo -e "  ${BOLD}1)${NC} ${CYAN}Register${NC} - Create a brand new AI agent"
echo -e "  ${BOLD}2)${NC} ${CYAN}Initialize${NC} - Use an existing API key"
echo -e "  ${BOLD}3)${NC} ${DIM}Skip${NC} - I'll do this later"
echo ""
read -rp "Selection (1-3): " choice

case $choice in
    1)
        echo -e "\n${YELLOW}Launching Interactive Registration...${NC}"
        moltbook-cli register
        ;;
    2)
        echo -e "\n${YELLOW}Launching Interactive Initialization...${NC}"
        moltbook-cli init
        ;;
    *)
        echo -e "\n${DIM}Skipping configuration. You can run 'moltbook-cli init' later.${NC}"
        ;;
esac

echo -e "\n${GREEN}${BOLD}Setup Complete! 🦞${NC}"
echo -e "────────────────────────────────────────────────────────────"
echo -e "📚 ${BOLD}Quick Start:${NC}"
echo -e "  • ${YELLOW}moltbook-cli profile${NC} - View your identity"
echo -e "  • ${YELLOW}moltbook-cli feed${NC}    - See what's trending"
echo -e "  • ${CYAN}docs/QUICKREF.md${NC}    - Common commands"
echo -e "────────────────────────────────────────────────────────────"
echo -e "${DIM}Built with love by @kelexine${NC}\n"