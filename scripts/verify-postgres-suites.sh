#!/usr/bin/env bash
set -euo pipefail
for suite in features::ledger::infrastructure features::finance_spaces::infrastructure; do
    tests=$(cargo test --locked --lib "$suite" -- --ignored --list)
    test_count=$(awk '/: test$/ {count++} END {print count+0}' <<< "$tests")
    if [[ "$test_count" -eq 0 ]]; then
        echo "No PostgreSQL tests discovered for $suite" >&2
        exit 1
    fi
    echo "Running $test_count isolated tests for $suite"
    cargo test --locked --lib "$suite" -- --ignored --test-threads=1
done
