# We Built a File Janitor by Letting Reddit Decide (Real Data, No Gut)

**TL;DR:** 9,363 Reddit wish posts + 1,416 threads + unsupervised KMeans told us to build a 5-15MB offline duplicate finder, not another finance app. Here's the research-first timeline with real maths.

---

## Why Research-First? (Skill: `research` 355K, `spec-driven-development` 26.4K)

Most indie devs pick an idea they like, then validate. We inverted it: **collect unsolicited complaints first**. Reddit rants are 10x more honest than surveys. 7% of 9,363 explicitly screamed "offline-first" (655 posts, 1 in 14) - in a world obsessed with cloud, that's a rebellion (Markethunt 2026-01-21). If you ignore that, you build subscription fatigue.

## The Datasets (Primary Sources Only)

| N | Finding | Source |
| :--- | :--- | :--- |
| 9,363 | 13.1% productivity, 200+ word furious Cooking | `markethunt.io` |
| 3,000 dev | Docs 24% (720) top gap | `trend-seeker.app` |
| 80 ideas / 1,416 threads / 1.6M searches / 941 Stripe $2,310 median | App 43.8K vs SaaS 7.4K (5.9x) | `businessideasdb.com` |
| AltTo 151/213 likes | No Tauri GUI janitor | `alternativeto.net` |

## Weighted Maths (Not Mental)

`Score = Fr*0.25 + Off*0.25 + Pay*0.20 + CompInv*0.15 + Gro*0.15` via `python3 -c`:

- Ledger 8*0.25+10*0.25+10*0.20+6*0.15+7*0.15 = **8.45**
- Janitor 9*0.25+10*0.25+6*0.20+9*0.15+6*0.15 = **8.20**

Ledger wins supervised due to Pay 10, but...

## Unsupervised Twist (Skill: `sklearn 1.9.0` KMeans + PCA 63.39%)

10×7 matrix → StandardScaler → KMeans k=3 (silhouette 0.269 best). PCA: PC1 `CompInv +0.58 Off +0.51 Pay -0.50` = Pay vs Offline trade-off.

Winner **Cluster 2 (mean 8.22): Janitor + Recipe** (Fr 9.5 Off 9.5 CompInv 8.0) - unsupervised found blue-ocean **without ever weighting Pay**. Hierarchical confirmed. Translation: low competition + high frustration beats high-pay crowded finance.

## Why Janitor Now? (Skill: `tauri-v2` 6.9K `svelte-code-writer` 8.1K)

- **Lightest:** Tauri 5-15MB vs Electron 150MB (10x). Fedora `webkit2gtk4.1` native.
- **Fastest:** Rust BLAKE3 hashing, `trash` not `rm`, SQLite, 7-day MVP, no auth/compliance.
- **Real gap:** `r/linux 1368s73` "What apps missing?" + `r/selfhosted` source-of-truth need + dupeGuru PPA broken.

## Reproduce From README

```bash
bash scripts/reproduce-from-readme.sh # reinstalls 7 skills, re-runs 9,363 analytics, KMeans, PRD 94/100 Sarah
bash scripts/generate-content.sh       # → this blog + thread + video
```

All via `npx skills add` with install counts, not hallucination.

## Content Loop

This blog was generated FROM the README timeline via `research` skill primary sources. Next devlog will demo `npm run tauri dev` scanning 10K files <60s.

**Repo:** https://github.com/Rythamo8055/file-janitor `docs/research-*.md` has all citations.

*Built 2026-08-23 on Fedora 44, Node 24, Rust 1.98.*


---

## Update 2026-08-23 03:26 IST — From Convo Log

*Logged change:* ## 2026-08-23 03:26 IST — Setup timestamped DEVLOG engine
*Total devlog entries:* 12
*Source:* docs/DEVLOG.md + README Live Dev Log (time/day stamped)

This section was auto-generated from our timestamped convo via `scripts/log-progress.sh` → `generate-content-from-convo.sh` (skills: research + spec-driven). No content gap.


---

## Update 2026-08-23 03:38 IST — From Convo Log

*Logged change:* ## 2026-08-23 03:38 IST — Implemented File Janitor MVP Tasks 1-3
*Total devlog entries:* 13
*Source:* docs/DEVLOG.md + README Live Dev Log (time/day stamped)

This section was auto-generated from our timestamped convo via `scripts/log-progress.sh` → `generate-content-from-convo.sh` (skills: research + spec-driven). No content gap.


---

## Update 2026-08-23 03:48 IST — From Convo Log

*Logged change:* ## 2026-08-23 03:48 IST — Fixed Tauri browser error + dark theme + UX
*Total devlog entries:* 14
*Source:* docs/DEVLOG.md + README Live Dev Log (time/day stamped)

This section was auto-generated from our timestamped convo via `scripts/log-progress.sh` → `generate-content-from-convo.sh` (skills: research + spec-driven). No content gap.


---

## Update 2026-08-23 03:55 IST — From Convo Log

*Logged change:* ## 2026-08-23 03:55 IST — Impeccable native polish + harden + error handling + defaults
*Total devlog entries:* 15
*Source:* docs/DEVLOG.md + README Live Dev Log (time/day stamped)

This section was auto-generated from our timestamped convo via `scripts/log-progress.sh` → `generate-content-from-convo.sh` (skills: research + spec-driven). No content gap.

