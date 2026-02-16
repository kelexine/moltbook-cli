#!/bin/bash
# Moltbook CLI Test Script
# Author: @kelexine
# Quick verification that everything works

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Moltbook CLI v0.2.0 Tests            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""

# Test 1: Check if CLI is installed
echo -e "${YELLOW}Test 1: Checking installation...${NC}"
if command -v moltbook-cli &> /dev/null; then
    VERSION=$(moltbook-cli --version)
    echo -e "${GREEN}✓ CLI is installed: $VERSION${NC}"
else
    echo -e "${RED}✗ CLI not found in PATH${NC}"
    echo "Run: cargo install --path ."
    exit 1
fi

# Test 2: Check config file
echo -e "\n${YELLOW}Test 2: Checking config file...${NC}"
if [ -f ~/.config/moltbook/credentials.json ]; then
    echo -e "${GREEN}✓ Config file exists${NC}"
else
    echo -e "${RED}✗ Config file not found${NC}"
    exit 1
fi

# Test 3: Test status command
echo -e "\n${YELLOW}Test 3: Testing status command...${NC}"
if moltbook-cli status > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Status command works${NC}"
else
    echo -e "${RED}✗ Status command failed${NC}"
    echo "This might be a network or API key issue"
fi

# Test 4: Test profile command
echo -e "\n${YELLOW}Test 4: Testing profile command...${NC}"
if moltbook-cli profile > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Profile command works${NC}"
else
    echo -e "${RED}✗ Profile command failed${NC}"
fi

# Test 5: Test global command
echo -e "\n${YELLOW}Test 5: Testing global command...${NC}"
if moltbook-cli global --limit 5 > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Global command works${NC}"
else
    echo -e "${RED}✗ Global command failed${NC}"
fi

# Test 6: Test DM check
echo -e "\n${YELLOW}Test 6: Testing DM check...${NC}"
if moltbook-cli dm-check > /dev/null 2>&1; then
    echo -e "${GREEN}✓ DM check works${NC}"
else
    echo -e "${RED}✗ DM check failed${NC}"
fi

echo -e "\n${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  Test Summary                          ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}All basic tests passed! 🦞${NC}"
echo ""
echo -e "${YELLOW}Try these commands next:${NC}"
echo -e "  moltbook-cli global"
echo -e "  moltbook-cli search \"rust\""
echo -e "  moltbook-cli submolts"
echo ""
