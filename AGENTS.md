# AGENTS.md - File Janitor (offline-vault)

## Project: File Janitor
Lightest offline duplicate-finder + organizer, Tauri v2 + Svelte 5, 5-15MB, Win/Mac/Linux.
Spec: `docs/file-janitor-spec.md` | PRD: `docs/file-janitor-prd.md` | Research: `docs/research-*.md`

## How to vibe code
1. Read `docs/file-janitor-spec.md` before any code - spec is truth.
2. Use Svelte 5 runes `$state`, `invoke("scan_folders")` for Rust.
3. Rust: BLAKE3 hash, `trash` crate not delete, `lopdf` for PDF.
4. Always run `cargo test` + `npm run check` before commit.
5. Keep `docs/research-*.md` citations, don't hallucinate metrics.

## Commands
- `npm run tauri dev` - dev
- `cargo test` - rust tests
- `npm run check` - svelte type

## Boundaries
Always do: trash, SQLite plugin, validate paths, **log every change via scripts/log-progress.sh with time/day stamp from convo**
Never do: network, delete, commit secrets, push without DEVLOG entry

## Timeline
2026-08-23 selected via unsupervised KMeans (see README timeline) → now spec → plan → tasks → implement

## Digital Content Engine (From Convo → All Platforms)
- **Every progress** (code, spec, research) MUST be logged: `bash scripts/log-progress.sh "What changed" "Why decision (data/skill)" "Content hook"` → appends to `docs/DEVLOG.md` + `README` Live Dev Log with `date +'%Y-%m-%d %H:%M IST'`
- Text is **taken from our convo** (user msg + agent action) so content is never invented - skill `research` ensures primary source, skill `spec-driven` ensures decision trace.
- Then `bash scripts/generate-content-from-convo.sh` auto-regenerates `docs/content/blog-*.md`, `thread-*.md`, `video-*.md`, `linkedin-*.md`, `indie-*.md` from DEVLOG + README timeline. **Never out of content.**
- Skills used when required: `research` 355K for facts, `tauri-v2`/`svelte-code-writer` for code, `spec-driven` 26.4K for decision log.
