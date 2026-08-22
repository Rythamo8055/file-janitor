# X Thread: Research-First File Janitor (12 tweets)

**1/12** We let Reddit decide our next app. 9,363 wish posts + unsupervised ML told us to build an offline duplicate finder, not another AI wrapper. Here's real data, real maths, no gut. 🧵

**2/12** Stack: Need lightest offline cross-platform. Tauri v2 (5-15MB, OS WebView) vs Electron 150MB vs Flutter 20-50MB. Fedora 44. Chose Tauri + Svelte 5 via verified skills: `tauri-v2` 6.9K `svelte-code-writer` 8.1K

**3/12** Data 1: 9,363 "I wish this existed" posts (Markethunt, 6 months). 13.1% productivity (1,231) but **7% = 655 explicitly demand offline-first** (1 in 14). Anti-cloud rebellion is real.

**4/12** Data 2: 3,000 dev requests (Trend Seeker). Docs 24% (720) top gap, Testing 21% (630). Devs want simpler offline tools, not enterprise.

**5/12** Data 3: BID 80 ideas from 1,416 threads + 1.6M searches + 941 Stripe (median $2,310 vs mean $14K, 69% <$5K). App avg 43.8K vs SaaS 7.4K (5.9x search). Problem severity 8.4/10.

**6/12** Weighted score `Fr*0.25+Off*0.25+Pay*0.20+CompInv*0.15+Gro*0.15` via python3: Ledger 8.45 > ADHD 8.40 > Recipe 8.25 > Janitor 8.20. Ledger wins on Pay 10, but...

**7/12** Unsupervised KMeans (sklearn 1.9.0, StandardScaler, k=3 silhouette 0.269, PCA 63%) found **Cluster 2 WINNER: Janitor + Recipe mean 8.22** (Fr 9.5 Off 9.5 CompInv 8.0) — low competition blue ocean WITHOUT weighting Pay. Data geometry > opinion.

**8/12** Why Janitor for FAST? r/linux missing app + dupeGuru PPA broken + Czkawka 151 likes no Tauri GUI + 12.1K inventory +52% YoY. Lightest + 7-day MVP. Rust BLAKE3 hashing, trash not delete, <15MB.

**9/12** Vibe files via spec-driven: PRD 94/100 Sarah + Spec 6 areas + AGENTS.md. Spec is truth before code. Skills: `research` 355K `spec-driven` 26.4K.

**10/12** Reproduce entire process from README:
`bash scripts/reproduce-from-readme.sh` → 7 skills, 9,363 analytics, KMeans, PRD

**11/12** Repo: github.com/Rythamo8055/file-janitor has `docs/research-*.md` with all citations + `research-unsupervised.md` PCA loadings. No hallucination.

**12/12** Next: `npm run tauri dev` demo scanning 10K <60s. Follow for devlog. What would YOU build: Janitor or Ledger? (Weighted vs unsupervised disagree!)
