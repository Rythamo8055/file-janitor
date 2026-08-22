#!/usr/bin/env bash
# log-progress.sh - Log every dev progress from convo with time/day stamp → README + DEVLOG → content
# Skills: spec-driven-development 26.4K (gate), research 355K (facts)
# Usage: bash scripts/log-progress.sh "What changed" "Why decision" "Content hook"
# Example: bash scripts/log-progress.sh "Added BLAKE3 scanner" "Why: benchmark BLAKE3 vs SHA256, Fedora NVMe <2s" "Video: hashing demo"
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
DEVLOG="$REPO_DIR/docs/DEVLOG.md"
README="$REPO_DIR/README.md"

TITLE="${1:-Update}"
WHY="${2:-via spec-driven skill}"
HOOK="${3:-Devlog}"
STAMP="$(date +'%Y-%m-%d %H:%M IST')"
DATE_ONLY="$(date +'%Y-%m-%d')"

echo "Logging $STAMP — $TITLE"

# Append to DEVLOG.md (full journal)
cat >> "$DEVLOG" << LOG

## $STAMP — $TITLE
- **What changed:** $TITLE
- **Why decision:** $WHY
- **Convo source:** logged via scripts/log-progress.sh from agent convo
- **Content angle:** $HOOK

LOG

# Update README Live Dev Log table (insert after header row)
if grep -q "Live Dev Log" "$README"; then
  # Insert new row after header "| When (IST) | What Changed |" line
  # Keep only last 10 rows to avoid bloat: we prepend
  ROW="| $STAMP | $TITLE | $WHY | $HOOK |"
  # Use python for safe insert
  python3 << PY
import pathlib
p = pathlib.Path("$README")
t = p.read_text()
marker = "| ... | ... | ... | ... |"
row = """$ROW"""
if marker in t:
    t = t.replace(marker, row + "\n" + marker)
    p.write_text(t)
    print("README updated")
else:
    print("Marker not found, DEVLOG only")
PY
fi

echo "✓ Logged to $DEVLOG and README"
echo "Next: bash scripts/generate-content-from-convo.sh"
