# Tasks: File Janitor MVP

**Spec:** `docs/file-janitor-spec.md` | **Plan:** `tasks/plan.md` | **Branch:** `main`  
**Skills:** `tauri-v2`, `svelte-code-writer`, `spec-driven-implementation`  
**Gated:** Do not start next until current verify passes.

---

- [x] **Task 1: Rust scanner engine (core)**
  - **Acceptance:** `cargo test` passes: identical files same BLAKE3, different different, 0-byte grouped, `scan_folders` returns `Vec<FileGroup>` JSON
  - **Verify:** `cargo test --manifest-path src-tauri/Cargo.toml` + `cargo check`
  - **Files:** `src-tauri/Cargo.toml`, `src-tauri/src/lib.rs`, `src-tauri/src/scanner.rs`
  - **Skill:** `tauri-v2` calling-rust-from-frontend

- [x] **Task 2: Tauri fs/dialog capabilities**
  - **Acceptance:** `default.json` allows `fs:readDir,readFile` + `dialog:open` + `path:default` + `sql:default`; `tauri dev` picks 2 folders
  - **Verify:** `npm run tauri dev` → OS folder dialog opens, no permission error
  - **Files:** `src-tauri/capabilities/default.json`, `src-tauri/tauri.conf.json`
  - **Skill:** `tauri-v2` configuring-permissions

- [x] **Task 3: Svelte UI shell (scan + groups + preview)**
  - **Acceptance:** `+page.svelte` shows Scan button, folder picker, groups list with size/count, image thumb + PDF name, keep-1 radio, trash + CSV buttons
  - **Verify:** `npm run check` green + manual scan `/tmp/test_janitor` with 3 dup PDFs 2 images
  - **Files:** `src/routes/+page.svelte`, `src/lib/scanner.js`, `src/routes/+layout.js`
  - **Skill:** `svelte-code-writer` svelte5 runes `$state` + `svelte-core-bestpractices`

- [ ] **Task 4: SQLite index (tauri-plugin-sql)**
  - **Acceptance:** `filejanitor.db` at `~/.local/share/com.offlinevault.filejanitor/` created on first scan, re-scan skips unchanged via mtime+size
  - **Verify:** `sqlite3` shows `files(path,hash,size,mtime)` rows after scan
  - **Files:** `src-tauri/src/db.rs`, `package.json` (plugin-sql), `src-tauri/src/lib.rs` (plugin init)
  - **Skill:** `martinholovsky/sqlite-database-expert` 2.7K

- [ ] **Task 5: Bulk rename + PDF merge (MVP)**
  - **Acceptance:** Rename regex `photo_(1)` → `photo-1` preview list + apply via Rust `regex`; Merge 2 PDFs → 1 via `lopdf`
  - **Verify:** Rename 3 files + merge `/tmp/a.pdf + /tmp/b.pdf` → `/tmp/merged.pdf` pages = sum
  - **Files:** `src/routes/+page.svelte` (rename pane), `src-tauri/src/rename.rs`, `src-tauri/src/pdf.rs`
  - **Skill:** `tauri-v2` integrating-js-frontends

- [ ] **Task 6: Trash safety + polish**
  - **Acceptance:** Delete moves to OS trash (not rm), CSV export `dupes.csv`, empty folders shown, permission errors toasted
  - **Verify:** `trash` crate test + manual delete → appears in `~/.local/share/Trash`
  - **Files:** `src-tauri/src/lib.rs` (trash), `src/lib/csv.js`
  - **Skill:** `tauri-v2` debugging

- [ ] **Task 7: Build + bundle <15MB**
  - **Acceptance:** `npm run tauri build` → `src-tauri/target/release/bundle` Flatpak/AppImage/Deb <15MB, `cargo fmt --check` pass
  - **Verify:** `du -h src-tauri/target/release/bundle/**/*` + `npm run check`
  - **Files:** `src-tauri/tauri.conf.json` (bundle), `.gitignore`
  - **Skill:** `tauri-v2` distributing-for-linux

---

**Next:** Implement Task 1 now.
