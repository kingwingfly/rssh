#! /bin/bash
set -e

export TERM=xterm-256color

# Statements waiting to be executed
statements=(
    "cargo fetch --locked"
    "cargo clippy --all-features --all-targets -- -D warnings"
    # rssh
    "cargo test"
)

# loop echo and executing statements
for statement in "${statements[@]}"; do
    echo "$(tput setaf 3)$statement$(tput sgr0)"
    eval $statement
    echo
done
