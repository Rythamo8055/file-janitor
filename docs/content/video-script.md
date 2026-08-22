# Video Script: "We Let AI Pick Our App Using 9,363 Reddit Posts" (8 min devlog)

**[0:00 HOOK - 20s]**
"Everyone builds what THEY want. We built what Reddit SCREAMS for. 9,363 posts, KMeans, and a 5MB Tauri app. Here's real data."

**[0:20 PROBLEM - 40s]**
Show Fedora 44, r/linux missing apps, Electron bloat 150MB, subscription fatigue $273/mo. Need lightest offline.

**[1:00 STACK - 60s]**
Demo `npx skills find tauri` 6.9K vs electron, `create-tauri-app` svelte. Skill `tauri-v2` required, `svelte-code-writer` 8.1K. Show 5-15MB.

**[2:00 DATA - 90s]**
Screen: Markethunt 9,363, Trend Seeker 3,000, BID 80/1416. Highlight 7% anti-cloud 655, 720 docs. `python3 -c 1231/9363*100` live.

**[3:30 MATHS - 60s]**
Show weighted table 8.45 vs 8.20, bias table. Ledger wins supervised.

**[4:30 UNSUPERVISED - 90s]**
Run `python3 /tmp/run_unsupervised.py` live. Show KMeans silhouette 0.269, PCA 63%, Cluster 2 Janitor+Recipe winner 8.22. Explain Pay vs CompInv trade-off.

**[6:00 WHY JANITOR - 40s]**
r/linux thread, dupeGuru broken PPA, SourceForge 2/week, show Czkawka. Blue ocean.

**[6:40 REPRODUCE - 30s]**
`bash scripts/reproduce-from-readme.sh` - skills reinstall, research rerun, PRD 94/100.

**[7:10 CTA - 20s]**
Repo github.com/Rythamo8055/file-janitor, star, next dev `tauri dev` scan demo.

**B-roll:** CLI skills installs, research md, unsupervised csv, Tauri window.

**Title/Thumbnail:** "9,363 Posts Picked Our App (Not Us)" + KMeans plot
