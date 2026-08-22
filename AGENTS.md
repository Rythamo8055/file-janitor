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
Always do: trash, SQLite plugin, validate paths
Never do: network, delete, commit secrets

## Timeline
2026-08-23 selected via unsupervised KMeans (see README timeline) → now spec → plan → tasks → implement
