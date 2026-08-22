# Plan: File Janitor MVP (Tauri v2 + Svelte 5)

**Date:** 2026-08-23 03:29 IST  
**Spec:** `docs/file-janitor-spec.md` | **PRD:** `docs/file-janitor-prd.md`  
**Skills used:** `tauri-v2` 6.9K, `svelte-code-writer` 8.1K, `spec-driven-implementation` 22.4K  
**Goal:** Ship lightest offline janitor (5-15MB) MVP in 7 tasks, each <5 files, verifiable.

---

## 1. Components & Dependencies

```
core (Rust scanner + Tauri cmds)  ←  no dep, build first
  │
  ├─→ fs-perms (dialog/fs capabilities)
  ├─→ ui-shell (Svelte routes, invoke)
  └─→ sqlite-index (plugin-sql, ~/share/filejanitor.db)

rename/pdf (lopdf)  ←  depends on core groups
```

**Build order:** core → fs-perms → ui-shell → sqlite-index → rename/pdf

**Parallel:** fs-perms + ui-shell can be parallel after core.

---

## 2. Risks & Mitigation

| Risk | Prob | Mitigation |
| :--- | :--- | :--- |
| BLAKE3 slower on macOS (dskDitto note) | Med | Benchmark BLAKE3 vs SHA256, feature-flag fallback via `blake3` crate `rayon` |
| `trash` fails on headless Fedora | Low | Fallback `std::fs::rename` to `~/.local/share/Trash` + toast |
| 100K files OOM | Med | Streaming `WalkDir` + batched `INSERT`, skip >100MB unless forced |
| webkit2gtk4.1 missing | High on Fedora | Docs prereq + fallback `cargo check` not `tauri dev` |

---

## 3. Verification Checkpoints (gated per spec-driven)

1. After core: `cargo test` hash identical/different + `cargo check`
2. After fs-perms: `npm run tauri dev` opens dialog, picks folders
3. After ui-shell: scan 2 folders → groups rendered, preview thumb
4. After sqlite: `filejanitor.db` exists, re-scan uses index
5. After rename/pdf: rename preview + merge 2 PDFs → 1

---

## 4. File Map (max 5 files per task)

- **Task 1:** `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`, `src-tauri/build.rs`
- **Task 2:** `src-tauri/capabilities/default.json`, `src-tauri/tauri.conf.json`
- **Task 3:** `src/routes/+page.svelte`, `src/lib/scanner.js`, `static/*`
- **Task 4:** `src-tauri/src/db.rs` (via plugin), `package.json`
- **Task 5:** `src/routes/+page.svelte` (rename), `src-tauri/src/pdf.rs`

---

## 5. Tech Decisions (from spec)

- Hash: BLAKE3 (rayon parallel) default, SHA256 fallback via `sha2` if `blake3` feature disabled on macOS.
- Trash: `trash = "3.15"` (not `remove_file`)
- PDF: `lopdf = "0.32"` for merge/split 2→1 MVP
- SQLite: `tauri-plugin-sql` 2 with `sqlite:filejanitor.db`

---

*Plan saved to `tasks/plan.md` per `spec-driven-development` Phase 2. Next: `tasks/todo.md`.*
