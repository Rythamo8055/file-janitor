# [Indie Hackers] Research-First: How Reddit + KMeans Picked File Janitor Over Finance

**One-liner:** We weighted 9,363 wish posts, then let unsupervised learning veto the weighted winner. Result: offline duplicate finder 5-15MB.

**Revenue reality (BID 941 Stripe):** median $2,310 vs mean $14K, 69% <$5K, SaaS 4.1x App. Lesson: App must be pay-once, not $5/mo subscription.

**Demand:** App 43.8K avg search vs SaaS 7.4K (5.9x). Problem severity 8.4/10. 7% demand offline explicitly.

**Supervised:** Fr 0.25 Off 0.25 Pay 0.20 Comp 0.15 Gro 0.15 → Ledger 8.45 > Janitor 8.20 (Pay 10 weight).

**Unsupervised:** 10×7 → KMeans k=3 (silhouette 0.269) → Cluster 2 Janitor+Recipe 8.22 wins (Fr 9.5 CompInv 8.0 low). Pay negative on PC1 vs Off/CompInv positive → proof pay vs blue-ocean trade-off. Hierarchical confirmed.

**Build:** Tauri v2 + Svelte 5 + Rust BLAKE3 + trash + SQLite, <15MB, Fedora-first. Spec: `docs/file-janitor-prd.md` 94/100 Sarah.

**Repro:** `bash scripts/reproduce-from-readme.sh` (7 skills) + `bash scripts/generate-content.sh`.

**Proof:** `github.com/Rythamo8055/file-janitor` docs/research-*.md citations. Ask: would you ship Pay winner or Cluster winner?
