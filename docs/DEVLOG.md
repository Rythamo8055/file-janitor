# DEVLOG — File Janitor (Timestamped Digital Journal)

> **Every change + decision from our convo is logged here with time/day stamp.** This file + README timeline is the single source for **all platform content** (blog, thread, video, IndieHackers, LinkedIn). Skills: `research` + `spec-driven` keep it honest.

**Format per entry (enforced by `scripts/log-progress.sh`):**
```
## YYYY-MM-DD HH:MM IST — Title
- **What changed:** files/code
- **Why decision:** real data / skill / convo quote
- **Convo source:** user msg + agent action
- **Content angle:** blog/thread/video hook
```

---

## 2026-08-23 02:22 IST — Need lightweight offline app
- **What changed:** Initial problem statement, compared Tauri 5-15MB vs Electron 150MB
- **Why:** Fedora 44 lightweight requirement, user explicitly said offline helpful app
- **Convo source:** user "we need to create a app that need to be light weight and all we are currently in the linux"
- **Content angle:** Blog intro "Why Tauri killed Electron for us"

## 2026-08-23 02:30 IST — Installed find-skills + searched stacks
- **What changed:** `npx skills add find-skills`, searched offline/desktop/tauri/pwa/niche with install counts
- **Why:** Use verified skills (>1K installs) not hallucination; alinaqi 2.9K, tauri-v2 6.9K, niche 23.3K
- **Convo source:** user `npx skills add https://github.com/vercel-labs/skills --skill find-skills`
- **Content angle:** Thread tweet 2 "We don't trust libs without install counts"

## 2026-08-23 02:35 IST — Scaffolded offline-vault + Rust
- **What changed:** `npx create-tauri-app` svelte, `rustup 1.98`, `Tauri CLI 2.6.2`, noted webkit2gtk4.1 missing
- **Why:** Lightest cross-platform, 1 codebase for Win/Mac/Linux per `tauri-v2` skill
- **Convo source:** user "we go for the lightest app possible so we go for a"
- **Content angle:** Video 1:00 stack demo

## 2026-08-23 02:50 IST — Market research 9,363 + 3,000 + 1,416
- **What changed:** `docs/research-app-selection.md` (154 lines) with 3 datasets, ranked 10 ideas
- **Why:** Markethunt 13.1% productivity, 7% anti-cloud 655, 200+ word furious Cooking - real Reddit over guess
- **Convo source:** user "use the real feed from the reddits and see what really any person needed"
- **Content angle:** Blog section "7% rebellion"

## 2026-08-23 02:58 IST — Deep analytics weighted maths
- **What changed:** `docs/research-deep-analytics.md` (236 lines) computed `1231/9363*100=13.1%` via python3, weighted `Fr*0.25+Off*0.25+Pay*0.20+CompInv*0.15+Gro*0.15` → Ledger 8.45 vs Janitor 8.20
- **Why:** Turn opinions into numbers, bias table (Reddit vocal, survivorship)
- **Convo source:** user "find more and also using the skills first find research skills"
- **Content angle:** Thread tweet 6 weighted table

## 2026-08-23 03:08 IST — Unsupervised KMeans + PCA
- **What changed:** `docs/research-unsupervised.md` (236 lines) 10×7 → StandardScaler → KMeans k=3 silhouette 0.269 PCA 63.39%, winner Cluster 2 Janitor+Recipe 8.22
- **Why:** Let data geometry veto supervised Pay bias, found blue-ocean low-comp
- **Convo source:** user "can you please use some algorith of unsupervised learning"
- **Content angle:** Video 4:30 live `run_unsupervised.py`

## 2026-08-23 03:10 IST — Selected Janitor for FAST
- **What changed:** Decision File Janitor (CompInv 9, r/linux 1368s73, dupeGuru broken, Czkawka 151 likes, 12.1K +52%)
- **Why:** Lightest + fastest 7-day MVP <15MB, no finance trust barrier, unsupervised winner
- **Convo source:** user "we gofor the janator"
- **Content angle:** Blog "Why we ignored the 8.45 winner"

## 2026-08-23 03:11 IST — Vibe files ready + GitHub
- **What changed:** `docs/file-janitor-prd.md` 94/100 Sarah, `docs/file-janitor-spec.md` 62 lines, `AGENTS.md`, `README` timeline, `gh repo create file-janitor` → https://github.com/Rythamo8055/file-janitor 48e2e07
- **Why:** spec-driven-development 26.4K gate: spec is truth before code
- **Convo source:** user "are our files for this vibe coded app ready you had githhub gh acess create a new repo"
- **Content angle:** Thread 9 PRD screenshot

## 2026-08-23 03:18 IST — WHY + scripts + content pack
- **What changed:** README ⑨ WHY table, `scripts/reproduce-from-readme.sh` (7 skills), `scripts/generate-content.sh`, `docs/content/blog/thread/video/indie` (4 files) → commit 92a1033
- **Why:** README = executable spec, skills required when required, never out of content
- **Convo source:** user "do it and commit with updates in the read me why we need to do this things means from read me we generate a script using the skills"
- **Content angle:** Indie post "README as script"

---

## How to log next progress (DO THIS EVERY CHANGE)

```bash
# From any convo, run:
bash scripts/log-progress.sh "Added BLAKE3 scanner in Rust" "Why: benchmark BLAKE3 1.5 vs SHA256 per dskDitto note, Fedora NVMe 1GB <2s" "Content: Devlog video 02:00 hashing demo"

# Then auto-generate all platform content:
bash scripts/generate-content-from-convo.sh
# → updates docs/content/blog-*.md + thread-*.md + video-*.md + linkedin-*.md with new entry
```

Next expected entries: `2026-08-23 03:xx - PLAN tasks/todo.md`, `... - Implement scan_folders Rust`, `... - Svelte preview UI`, etc. Each will append here with timestamp.

## 2026-08-23 03:26 IST — Setup timestamped DEVLOG engine
- **What changed:** Setup timestamped DEVLOG engine
- **Why decision:** Why: user asked to never be out of content; spec-driven says log decision + time from convo so README becomes journal
- **Convo source:** logged via scripts/log-progress.sh from agent convo
- **Content angle:** Content: This thread itself

