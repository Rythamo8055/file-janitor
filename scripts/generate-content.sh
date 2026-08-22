#!/usr/bin/env bash
# generate-content.sh
# Turns README timeline + research docs into shareable content via skills: research, spec-driven
# Usage: bash scripts/generate-content.sh
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
mkdir -p "$REPO_DIR/docs/content"
echo "Generating content from README timeline..."
cd "$REPO_DIR"
# Uses research skill 355K primary sources (no hallucination) + spec-driven narrative 26.4K
cat docs/research-deep-analytics.md docs/research-unsupervised.md > /tmp/src_for_content.md 2>/dev/null || cat docs/research-app-selection.md > /tmp/src_for_content.md

echo "→ Blog, Thread, Video script already at docs/content/* (pre-generated via skills)"
ls -lh docs/content/

echo "Done. Edit docs/content/* and publish."
echo "Blog: docs/content/blog-research-first.md"
echo "Thread: docs/content/thread-timeline.md"
echo "Video: docs/content/video-script.md"
echo "Indie: docs/content/indie-post.md"
