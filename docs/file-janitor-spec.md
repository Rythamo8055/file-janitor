# Spec: File Janitor

## Objective
Build the lightest duplicate-finder + organizer for Fedora-first, cross-platform later. User: Ravi (Linux power user). Success: scan 10K files <60s, <15MB binary, preview+trash works 95%.

## Tech Stack
- Tauri v2.6.2, Svelte 5, Vite 6, SvelteKit static adapter, Rust 1.98, BLAKE3 1.5, trash 3.15, lopdf 0.32, serde, tauri-plugin-sql 2, tauri-plugin-dialog/fs
- Node 24, identifier `com.offlinevault.filejanitor`, productName `File Janitor`

## Commands
```
Dev:   npm run tauri dev  (vite on 1420 → Tauri)
Build: npm run build && npm run tauri build  (→ Flatpak/AppImage/MSI/DMG)
Test:  cargo test --manifest-path src-tauri/Cargo.toml && npm run check
Lint:  cargo fmt --check && npm run check
```

## Project Structure
```
offline-vault/
  docs/                           → research-*.md, file-janitor-prd.md, this spec
  src/routes/+page.svelte         → Svelte UI (scan button, groups, preview, rename, pdf)
  src/lib/scanner.js              → invoke Rust commands
  src-tauri/src/lib.rs            → Tauri commands: scan, hash, trash, rename, pdf
  src-tauri/Cargo.toml            → blake3, trash, lopdf
  src-tauri/capabilities/default.json → fs/dialog/sql perms
  static/                         → icons
  build/                          → frontendDist
```

## Code Style
```svelte
<script>
  import { invoke } from "@tauri-apps/api/core";
  let folders = $state([]); // Svelte 5 runes
  async function scan() {
    const groups = await invoke("scan_folders", { paths: folders }); // Rust BLAKE3
  }
</script>
```
Naming: kebab `file-janitor`, snake `scan_folders`, Pascal `FileGroup`. No secrets, validate paths.

## Testing Strategy
- Rust `cargo test`: hash identical files → same BLAKE3, different → different, 0-byte group, trash moves not delete (check `trash` crate)
- Svelte `svelte-check --tsconfig jsconfig.json` type check
- Manual: scan `/tmp/test_janitor` with 3 dup PDFs + 2 images, preview, keep-1, trash, rename `photo_(1)` → `photo-1`, merge 2 PDFs
- Coverage: scanner 80% logic

## Boundaries
- Always: run `cargo test` + `svelte-check` before commit, use trash not `std::fs::remove_file`, SQLite via plugin
- Ask first: schema change, new crate, CSP change, bundle target change
- Never: commit `node_modules`/`target`/`build`, hardcode `/home`, network fetch, delete without trash

## Success Criteria
- [ ] `npm run tauri dev` opens 800x600 window, scan 2 folders groups dupes
- [ ] Preview image thumb + PDF page1, keep-1, CSV export
- [ ] Bulk rename regex preview, PDF merge 2→1
- [ ] `cargo test` green, `npm run check` green
- [ ] `npm run tauri build` → <15MB binary (check `src-tauri/target/release/bundle`)

## Open Questions
- Default hash: BLAKE3 or SHA256 on macOS? → bench both, default BLAKE3 Linux/Win, SHA256 mac fallback (`dskDitto` note).
