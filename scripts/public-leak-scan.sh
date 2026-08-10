#!/bin/bash
# public-leak-scan.sh
# Server-side backstop for public-source leak gate.
# This script scans public-bound source files for any internal identifiers or infrastructure paths.

set -euo pipefail

echo "==> Starting public leak scan..."

# Define internal identifier patterns to reject if found
FORBIDDEN_PATTERNS=(
    "/Users/Admin/Documents/Calyx"
    "C:\\Users\\Admin"
)

FOUND_LEAKS=0

# Scan modified or all relevant source files
# For now, we scan files in crates/ to ensure no local home directory leaks exist
for pattern in "${FORBIDDEN_PATTERNS[@]}"; do
    if grep -rF "$pattern" crates/ .github/ 2>/dev/null; then
        echo "ERROR: Internal identifier pattern '$pattern' leaked in the repository!"
        FOUND_LEAKS=1
    fi
done

if [ "$FOUND_LEAKS" -ne 0 ]; then
    echo "Leak scan FAILED!"
    exit 1
fi

echo "==> Public leak scan completed successfully. No internal identifiers found."
exit 0
