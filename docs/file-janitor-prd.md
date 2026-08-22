# Product Requirements Document: File Janitor

**Version:** 1.0  
**Date:** 2026-08-23  
**Author:** Sarah (Product Owner) - offline-vault  
**Quality Score:** 94/100  
**Stack:** Tauri v2 + Svelte 5 + Vite + SQLite + Rust (BLAKE3)  
**Identifier:** com.offlinevault.filejanitor  

---

## Executive Summary

**File Janitor** is the lightest cross-platform (Win/Mac/Linux, 5-15MB) offline duplicate-finder + bulk organizer + PDF toolkit. Solves the **#1 validated Linux missing app** (`r/linux 1368s73` 2026-02) where users still rely on 2013-era CLI tools (`fdupes`, `rmlint`, `FSlint` SourceForge 2-188 DL/week). Unlike `Czkawka (151 likes)` / `dupeGuru (213 likes <1MB, PPA broken on Jammy)` / `dskDitto 354 stars CLI`, Janitor is modern Tauri-native with GUI preview, source-of-truth picker, bulk rename, PDF merge/split, image convert - all 100% offline via Rust hashing + SQLite index. No account, no cloud, pay-once $9-12.

Selected via unsupervised KMeans (k=3, silhouette 0.269, 63.39% PCA) on 10×7 real data (9,363 wish posts + 1,416 threads + 1.6M searches) → winner Cluster 2 (Janitor+Recipe, mean 8.22, PC1 right: high Fr 9.5 + Off 9.5 + CompInv 8.0) beat high-pay finance cluster.

---

## Problem Statement

**Current Situation:**
- Linux users download same mp3/pdf/epub habitually; photo/music libs balloon; `r/selfhosted 17yvcyt` asks for source-of-truth aware dup finder - none exists as modern GUI.
- Existing: `tecmint.com` 6 CLI tools, `sourceforge.net` abandoned 2013, `dupeGuru` install hack `ppa bionic`, `Czkawka` heavy, all no preview/bulk/PDF.
- 7% of 9,363 wish posts (655) explicitly demand offline-first (Markethunt 2026-01-21). BID `13 threads 21,080 upvotes` self-hosted loudest.

**Proposed Solution:** Single Tauri binary: scan folders → BLAKE3/SHA256 content hash → groups by duplicate → preview → choose keeper/source-of-truth → safe move/delete (trash) + bulk rename (regex) + PDF tools.

**Business Impact:** Solo dev can build MVP in 7 days, $0 infra, 1-time $12 = 0.18mo of avg AI stack $66/mo (Bango 2025). TAM: App avg 43.8K search vs SaaS 7.4K (5.9x) BID; file-manager evergreen.

---

## Success Metrics

**Primary KPIs:**
- Scan 10K files (50GB) in <60s on 4GB RAM Fedora 44 (Rust fclones-like) - measured via `time npm run tauri dev` bench
- Preview + bulk action succeeds on 95% groups without crash
- Binary <15MB (Tauri vs Electron 120MB)
- 1K GitHub stars + 500 installs first 90 days

**Validation:** Bench on Fedora 44, r/linux dogfooding thread.

---

## User Personas

### Primary: Ravi, Linux Power User (Fedora/Ubuntu)
- **Role:** Developer / photographer
- **Goals:** Reclaim disk, organize `~/Downloads` `~/Photos` without CLI
- **Pain Points:** `fdupes -r` no preview, `dupeGuru` broken, `Czkawka` no PDF
- **Technical Level:** Intermediate

### Secondary: Anya, Cross-platform Freelancer
- **Role:** Manages Win laptop + Mac + Linux server
- **Goals:** One tool for all OS, works on plane (offline)
- **Pain Points:** Duplicate APKs, PDFs across devices

---

## User Stories & Acceptance Criteria

### Story 1: Scan by Content
**As** Ravi **I want** to select 2 folders and scan by content hash **So that** identical files with different names are grouped.
- [ ] Select folders via OS dialog (Tauri fs)
- [ ] Hash via BLAKE3, groups show size + count + hash
- [ ] Progress bar + cancel

### Story 2: Preview & Source-of-Truth
**As** Ravi **I want** to preview images/PDFs and mark source-of-truth folder **So that** safe delete keeps one.
- [ ] Image thumb, PDF first page preview
- [ ] Radio: keep per group + global source-of-truth folder
- [ ] Delete moves to trash (not rm), CSV export `dupes.csv`

### Story 3: Bulk Rename + PDF
**As** Anya **I want** regex bulk rename + PDF merge/split offline **So that** organize without extra apps.
- [ ] Rename: find/replace regex, preview list
- [ ] PDF: merge selected PDFs, split by pages
- [ ] All offline, no network

### Out of Scope
- Cloud sync, account, AI, network drives scan v1

---

## Functional Requirements

**Feature 1: Scanner Engine (Rust)**
- Description: Recursive scan, file size → hash (BLAKE3), SQLite index `~/.local/share/filejanitor.db`
- Flow: Pick folders → Scan → Groups sorted by wasted space
- Edge: 0-byte files grouped separately, permission errors skipped logged
- Error: Show `Permission denied: /root` toast, continue

**Feature 2: Viewer & Actions**
- Bulk select, keep-one enforcement, trash via `trash-rs`

**Feature 3: Rename/PDF**
- Rename uses `regex` crate, PDF via `lopdf` Rust

---

## Technical Constraints

**Performance:** Hash 1GB in <2s on NVMe, scan 10K <60s
**Security:** No network, CSP null disabled only for local, trash not delete
**Integration:** Tauri fs/dialog, SQLite `tauri-plugin-sql`, no Plaid
**Stack:** Tauri v2, Svelte 5, Vite 6, Rust 1.98, BLAKE3, lopdf, trash
**Commands:** `npm run dev` / `npm run build` / `npm run tauri dev` / `cargo test`

---

## MVP Scope & Phasing

**Phase 1 MVP (ship):** Scan + groups + preview + keep + trash + CSV + rename
**Phase 2:** PDF merge/split, image convert, symlink option `dskDitto --link`
**Future:** Duplicate image fuzzy (perceptual hash), scheduled scan

---

## Risk Assessment

| Risk | Prob | Impact | Mitigation |
| :--- | :--- | :--- | :--- |
| BLAKE3 macOS slower than SHA256 | Med | Med | Benchmark both, feature flag `dskDitto` note |
| Trash not available on some Linux | Low | High | Fallback to rename to `~/.trash` |
| Large 100K scan OOM | Med | High | Stream hashing, batched DB writes |

---

## Dependencies & Blockers

**Dependencies:** `webkit2gtk4.1-devel librsvg2` Fedora prereq, `blake3` crate
**Blockers:** None

---

*Source: docs/research-app-selection.md + research-deep-analytics.md + research-unsupervised.md*
