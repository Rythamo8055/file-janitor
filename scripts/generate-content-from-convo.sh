#!/usr/bin/env bash
# generate-content-from-convo.sh - DEVLOG + convo → all platform content (never out of content)
# Skills: research 355K (primary sources only), svelte-code-writer/tauri-v2 for tech posts, spec-driven for narrative
# Usage: bash scripts/generate-content-from-convo.sh
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
cd "$REPO_DIR"

mkdir -p docs/content
STAMP="$(date +'%Y-%m-%d %H:%M IST')"

echo "Generating content from convo/journal... $STAMP"

# Source is our convo journal + README timeline (never hallucinated)
SRC="docs/DEVLOG.md"
COUNT=$(grep -c "^## " "$SRC" || echo 0)
LAST=$(grep "^## " "$SRC" | tail -n 1 || echo "No entries")

# 1. Blog update (append latest)
cat >> docs/content/blog-research-first.md << BLOG

---

## Update $STAMP — From Convo Log

*Logged change:* $LAST
*Total devlog entries:* $COUNT
*Source:* $SRC + README Live Dev Log (time/day stamped)

This section was auto-generated from our timestamped convo via \`scripts/log-progress.sh\` → \`generate-content-from-convo.sh\` (skills: research + spec-driven). No content gap.

BLOG

# 2. LinkedIn post (new file per run)
cat > docs/content/linkedin-$(date +%Y%m%d-%H%M).md << LI
# LinkedIn — File Janitor Devlog $STAMP

We log every change with why + time from our actual convo. Latest:

$LAST

Why this matters: spec-driven 26.4K says spec is truth before code, research 355K says primary sources only. So our LinkedIn never runs dry - it's the DEVLOG.

Full log: docs/DEVLOG.md ($COUNT entries) | Repo: https://github.com/Rythamo8055/file-janitor
LI

# 3. IndieHackers update
cat > docs/content/indie-update-$(date +%Y%m%d-%H%M).md << INDIE
# Indie Update $STAMP — $COUNT entries logged

$LAST

Stack: Tauri v2 + Svelte 5, logged via scripts/log-progress.sh, content via research skill.

INDIE

# 4. Tweet from last log
LAST_TITLE=$(grep "^## " "$SRC" | tail -n 1 | sed 's/^## //')
cat > docs/content/tweet-latest.md << TWEET
🧵 Devlog $STAMP — $LAST_TITLE

Logged with why + time/day stamp from our convo. Never out of content because README is the journal.

Full: docs/DEVLOG.md ($COUNT entries) #buildinpublic
TWEET

# 5. Video chapter
cat > docs/content/video-chapter-$(date +%Y%m%d-%H%M).md << VID
# Video Chapter $STAMP
- Show DEVLOG entry: $LAST
- Explain why decision (skill: spec-driven)
- Demo code change
VID

ls -lh docs/content/*.md | tail -n 20
echo "✓ Content generated from DEVLOG ($COUNT entries) → docs/content/"
echo "Publish: blog, thread-timeline.md, video-script.md, linkedin-*.md, indie-*.md"
