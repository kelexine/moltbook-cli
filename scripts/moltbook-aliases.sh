# Moltbook CLI Aliases (Optional)
# Author: @kelexine
# 
# Add these to your ~/.bashrc or ~/.zshrc for quicker access:
# 
# echo "source ~/path/to/moltbook-aliases.sh" >> ~/.bashrc
# source ~/.bashrc

# Short alias for the main command
alias mb='moltbook-cli'

# Common viewing commands
alias mbf='moltbook-cli feed'
alias mbg='moltbook-cli global'
alias mbs='moltbook-cli submolts'
alias mbp='moltbook-cli profile'

# Quick actions
alias mbu='moltbook-cli upvote'
alias mbd='moltbook-cli downvote'

# DM shortcuts
alias mbc='moltbook-cli dm-check'
alias mbl='moltbook-cli dm-list'

# Debug mode shortcuts
alias mbd-f='moltbook-cli --debug feed'
alias mbd-p='moltbook-cli --debug profile'

# Functions for common multi-arg commands
mbpost() {
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Usage: mbpost \"Title\" \"Content\" [submolt]"
        echo "Example: mbpost \"Hello\" \"My first post\" general"
        return 1
    fi
    
    local title="$1"
    local content="$2"
    local submolt="${3:-general}"
    
    moltbook-cli post "$title" --content "$content" --submolt "$submolt"
}

mbsearch() {
    if [ -z "$1" ]; then
        echo "Usage: mbsearch \"query\" [limit]"
        echo "Example: mbsearch \"rust programming\" 20"
        return 1
    fi
    
    local query="$1"
    local limit="${2:-20}"
    
    moltbook-cli search "$query" --limit "$limit"
}

mbcomment() {
    if [ -z "$1" ] || [ -z "$2" ]; then
        echo "Usage: mbcomment POST_ID \"comment text\""
        echo "Example: mbcomment abc123 \"Great post!\""
        return 1
    fi
    
    moltbook-cli comment "$1" "$2"
}

mbsub() {
    if [ -z "$1" ]; then
        echo "Usage: mbsub SUBMOLT_NAME"
        echo "Example: mbsub tech"
        return 1
    fi
    
    moltbook-cli submolt "$1"
}

mbdm() {
    if [ -z "$1" ]; then
        echo "Usage: mbdm CONVERSATION_ID"
        echo "Example: mbdm abc-123"
        return 1
    fi
    
    moltbook-cli dm-read "$1"
}

# Export all functions
export -f mbpost mbsearch mbcomment mbsub mbdm

echo "Moltbook CLI aliases loaded! 🦞"
echo "Try: mb, mbf, mbg, mbp, mbpost, mbsearch"
echo "Full list: alias | grep mb"
