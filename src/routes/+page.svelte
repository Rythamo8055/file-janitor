<script>
  // @ts-nocheck
  // skill: svelte-code-writer 8.1K, tauri-v2 6.9K - Svelte 5 runes + invoke + dark mode
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  let folders = $state([]);
  let groups = $state([]);
  let scanning = $state(false);
  let error = $state("");
  let isDark = $state(false);
  let isTauri = $state(false);
  let stats = $state({ totalGroups: 0, totalWasted: 0, totalFiles: 0 });
  let renamePattern = $state("(.*) \\(1\\)");
  let renameReplace = $state("$1");

  onMount(() => {
    // detect Tauri vs web preview - fixes window.__TAURI_INTERNALS__ is undefined in browser
    isTauri = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;
    // dark mode: respect system + remember choice
    const saved = typeof localStorage !== "undefined" ? localStorage.getItem("filejanitor-theme") : null;
    if (saved) isDark = saved === "dark";
    else if (typeof window !== "undefined" && window.matchMedia) isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    applyTheme();
    // demo data for web preview so UI not empty
    if (!isTauri && groups.length === 0) {
      groups = [
        { hash: "a1b2c3d4e5f67890ab12", size: 2457600, count: 3, wasted: 4915200, files: [
          { path: "/home/you/Downloads/photo.jpg", size: 2457600, hash: "a1b2", name: "photo.jpg" },
          { path: "/home/you/Downloads/photo (1).jpg", size: 2457600, hash: "a1b2", name: "photo (1).jpg" },
          { path: "/home/you/Pictures/photo copy.jpg", size: 2457600, hash: "a1b2", name: "photo copy.jpg" }
        ]},
        { hash: "f9e8d7c6b5a43210ff99", size: 1048576, count: 2, wasted: 1048576, files: [
          { path: "/home/you/Documents/report.pdf", size: 1048576, hash: "f9e8", name: "report.pdf" },
          { path: "/home/you/Documents/report (1).pdf", size: 1048576, hash: "f9e8", name: "report (1).pdf" }
        ]}
      ];
      stats = { totalGroups: 2, totalWasted: 5963776, totalFiles: 5 };
    }
  });

  function applyTheme() {
    if (typeof document !== "undefined") {
      document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
      try { localStorage.setItem("filejanitor-theme", isDark ? "dark" : "light"); } catch {}
    }
  }
  function toggleTheme() { isDark = !isDark; applyTheme(); }

  async function pickFolders() {
    if (!isTauri) {
      error = "";
      // web preview fallback: use demo folders + allow manual entry
      const manual = prompt("Web preview: enter folder path (Tauri dialog needs desktop app). Try: /home/you/Downloads");
      if (manual) folders = [...new Set([...folders, manual])];
      else error = "In browser: folder picker needs Tauri desktop. Use Tauri build for real scan. Demo data shown below.";
      return;
    }
    try {
      const selected = await open({ directory: true, multiple: true });
      if (selected) {
        const arr = Array.isArray(selected) ? selected : [selected];
        folders = [...new Set([...folders, ...arr])];
        error = "";
      }
    } catch (e) {
      error = String(e);
    }
  }

  function removeFolder(idx) { folders = folders.filter((_, i) => i !== idx); }

  async function scan() {
    if (folders.length === 0) { error = "Pick at least one folder first"; return; }
    if (!isTauri) { error = "Web preview: scanning needs Tauri Rust backend (BLAKE3). Run `npm run tauri dev` for real scan. Demo groups shown."; return; }
    scanning = true; error = ""; groups = [];
    try {
      const res = await invoke("scan_folders", { paths: folders });
      groups = res;
      let wasted = 0, files = 0;
      for (const g of groups) { wasted += g.wasted; files += g.count; }
      stats = { totalGroups: groups.length, totalWasted: wasted, totalFiles: files };
      if (groups.length === 0) error = "";
    } catch (e) {
      const msg = String(e);
      if (msg.includes("__TAURI_INTERNALS__")) error = "Run as Tauri app for full scan (window.__TAURI_INTERNALS__ not in browser). Demo data shown in preview.";
      else error = msg;
    } finally { scanning = false; }
  }

  async function trashGroup(group) {
    if (!isTauri) { groups = groups.filter(g => g.hash !== group.hash); return; }
    const toTrash = group.files.slice(1).map(f => f.path);
    if (toTrash.length === 0) return;
    try { await invoke("trash_files", { paths: toTrash }); groups = groups.filter(g => g.hash !== group.hash); stats.totalGroups = groups.length; }
    catch (e) { error = String(e); }
  }
  async function trashSingle(path) {
    if (!isTauri) { groups = groups.map(g => ({ ...g, files: g.files.filter(f => f.path !== path), count: g.files.filter(f => f.path !== path).length })).filter(g => g.count > 1); return; }
    try { await invoke("trash_files", { paths: [path] }); groups = groups.map(g => ({ ...g, files: g.files.filter(f => f.path !== path), count: g.files.filter(f => f.path !== path).length })).filter(g => g.count > 1); }
    catch (e) { error = String(e); }
  }
  async function doRename() {
    if (!isTauri) { error = "Web preview: rename needs Rust. Run Tauri for real."; return; }
    let allPaths = groups.flatMap(g => g.files.map(f => f.path));
    if (allPaths.length === 0) return;
    try { await invoke("regex_rename", { paths: allPaths, pattern: renamePattern, replacement: renameReplace }); await scan(); }
    catch (e) { error = String(e); }
  }
  function exportCSV() {
    let rows = ["hash,size,count,path,wasted"];
    for (const g of groups) for (const f of g.files) rows.push(`${g.hash},${g.size},${g.count},"${f.path}",${g.wasted}`);
    const csv = rows.join("\n");
    const blob = new Blob([csv], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a"); a.href = url; a.download = "dupes.csv"; a.click(); URL.revokeObjectURL(url);
  }
  function formatBytes(b) { if (b < 1024) return b + " B"; if (b < 1024*1024) return (b/1024).toFixed(1) + " KB"; if (b < 1024*1024*1024) return (b/1024/1024).toFixed(1) + " MB"; return (b/1024/1024/1024).toFixed(1) + " GB"; }
  function isImage(name) { return /\.(jpg|jpeg|png|gif|webp|bmp)$/i.test(name); }
</script>

<main class="container">
  <header>
    <div class="header-top">
      <div>
        <h1>File Janitor <span class="badge">offline • 5-15MB</span></h1>
        <p class="subtitle">Lightest duplicate finder. Rust BLAKE3 hashing, no cloud. Fedora-first, Win/Mac next.</p>
      </div>
      <button class="theme-toggle" onclick={toggleTheme} aria-label="Toggle dark mode">
        {#if isDark}☀️ Light{:else}🌙 Dark{/if}
      </button>
    </div>
    {#if !isTauri}
      <div class="banner">👁️ Web preview — Tauri backend disabled (demo groups shown). Run <code>npm run tauri dev</code> for real BLAKE3 scan.</div>
    {/if}
  </header>

  {#if error}
    <div class="error">⚠️ {error}</div>
  {/if}

  <section class="card">
    <div class="card-head">
      <h2>1. Pick folders</h2>
      <span class="hint">{folders.length} selected</span>
    </div>
    <div class="row">
      <button onclick={pickFolders} class="btn">
        <span class="icon">📁</span> + Add folder
      </button>
      <button onclick={scan} disabled={scanning || folders.length===0} class="btn primary">
        {#if scanning}<span class="spinner"></span> Scanning...{:else}🔍 Scan {folders.length} folder(s){/if}
      </button>
      {#if groups.length>0}
        <button onclick={exportCSV} class="btn ghost">⬇ Export CSV</button>
      {/if}
    </div>
    {#if folders.length>0}
      <ul class="folders">
        {#each folders as f, i}
          <li><span class="path">{f}</span> <button class="mini" onclick={()=>removeFolder(i)} aria-label="Remove">✕</button></li>
        {/each}
      </ul>
    {:else}
      <div class="empty">
        <div class="empty-icon">📂</div>
        <p>No folders yet</p>
        <span class="hint">Pick ~/Downloads or ~/Photos to find duplicates</span>
      </div>
    {/if}
  </section>

  {#if groups.length>0}
    <section class="card">
      <div class="card-head">
        <h2>2. Results</h2>
        <span class="pill">{stats.totalGroups} groups • {stats.totalFiles} files • {formatBytes(stats.totalWasted)} wasted</span>
      </div>
      <div class="row search-row">
        <input placeholder="regex pattern e.g. (.*) \(1\)" bind:value={renamePattern} class="grow" />
        <input placeholder="replacement e.g. $1" bind:value={renameReplace} />
        <button onclick={doRename} class="btn">✏️ Bulk rename</button>
      </div>
      <p class="hint">Rename uses Rust <code>regex</code> on names only. Keep-first = safest.</p>

      {#each groups as g (g.hash)}
        <details class="group" open>
          <summary>
            <div class="group-meta">
              <strong>{g.count} files</strong>
              <span class="dot">•</span> {formatBytes(g.size)} each
              <span class="dot">•</span> <span class="wasted">wasted {formatBytes(g.wasted)}</span>
              <span class="hash">{g.hash.slice(0,10)}…</span>
            </div>
            <button class="btn mini primary" onclick={(e)=>{e.stopPropagation(); trashGroup(g)}}>🗑️ Keep 1, trash rest</button>
          </summary>
          <ul class="files">
            {#each g.files as f, idx}
              <li class:keeper={idx===0}>
                <span class="keep">{idx===0 ? "KEEP" : "DUP"}</span>
                <span class="path">{f.path}</span>
                <span class="size">{formatBytes(f.size)}</span>
                {#if isImage(f.name)}<span class="thumb">🖼️</span>{/if}
                {#if idx!==0}<button class="mini" onclick={()=>trashSingle(f.path)}>Trash</button>{/if}
              </li>
            {/each}
          </ul>
        </details>
      {/each}
    </section>
  {:else if !scanning && folders.length>0}
    <div class="empty card"><div class="empty-icon">✨</div><p>No duplicates found</p><span class="hint">Try more folders</span></div>
  {/if}

  <footer>
    <p><span class="skill">tauri-v2 6.9K</span> <span class="skill">svelte-code-writer 8.1K</span> • <a href="https://github.com/Rythamo8055/file-janitor" target="_blank">GitHub</a> • <span class="dot">•</span> {isTauri ? "Tauri mode" : "Web preview"}</p>
    <p class="hint">Offline • trash not delete • BLAKE3 • {isDark ? "Dark" : "Light"} theme</p>
  </footer>
</main>

<style>
  /* Light + Dark via data-theme — UX friendly, no flash */
  :root { --bg:#f6f7f9; --card:#ffffff; --text:#0f0f0f; --muted:#667085; --border:#e5e7eb; --card-shadow:0 8px 24px rgba(16,24,40,0.06); --primary:#24c8db; --primary-hover:#1fb0c1; --error-bg:#fef3f2; --error-border:#fecdca; --keep-bg:#e0f2fe; }
  :root[data-theme="dark"] { --bg:#0b1220; --card:#121a2b; --text:#e5e7eb; --muted:#94a3b8; --border:#1f2a44; --card-shadow:0 8px 24px rgba(0,0,0,0.35); --primary:#22d3ee; --primary-hover:#06b6d4; --error-bg:#2b0f0f; --error-border:#7f1d1d; --keep-bg:#0e2a33; }
  :global(html) { background: var(--bg); }
  :global(body) { margin:0; font-family: Inter, system-ui, sans-serif; background: var(--bg); color: var(--text); transition: background .2s, color .2s; }
  .container { max-width: 920px; margin: 0 auto; padding: 1.5rem 1rem 3rem; }
  header { margin-bottom: .5rem; }
  .header-top { display:flex; justify-content:space-between; align-items:flex-start; gap:1rem; }
  h1 { margin:0; font-size:1.9rem; letter-spacing:-.02em; }
  .badge { font-size:.7rem; background:var(--primary); color:#001018; padding:.2em .6em; border-radius:1em; vertical-align:middle; font-weight:700; }
  .subtitle { color:var(--muted); margin:.25rem 0 0; }
  .theme-toggle { border:1px solid var(--border); background:var(--card); color:var(--text); padding:.45em .9em; border-radius:999px; cursor:pointer; box-shadow:var(--card-shadow); }
  .banner { background: linear-gradient(135deg, #0ea5e9 0%, #22d3ee 100%); color:white; padding:.6em 1em; border-radius:10px; margin:.8rem 0; font-size:.9rem; }
  :root[data-theme="dark"] .banner { background: linear-gradient(135deg, #0e7490 0%, #0891b2 100%); }
  .banner code { background:rgba(255,255,255,.2); padding:.1em .4em; border-radius:4px; }
  .card { background:var(--card); border:1px solid var(--border); border-radius:16px; padding:1.1rem; margin:1rem 0; box-shadow:var(--card-shadow); }
  .card-head { display:flex; justify-content:space-between; align-items:center; gap:.5rem; flex-wrap:wrap; margin-bottom:.5rem; }
  .card-head h2 { margin:0; font-size:1.1rem; }
  .pill { background:var(--bg); border:1px solid var(--border); padding:.25em .6em; border-radius:999px; font-size:.8rem; color:var(--muted); }
  .row { display:flex; gap:.5rem; flex-wrap:wrap; align-items:center; margin:.5rem 0; }
  .search-row input { flex:1; min-width:160px; }
  .grow { flex:1; min-width:200px; }
  .btn { border:1px solid var(--border); background:var(--card); color:var(--text); padding:.55em 1em; border-radius:10px; cursor:pointer; display:inline-flex; align-items:center; gap:.4em; transition:.15s; }
  .btn:hover { transform:translateY(-1px); box-shadow:var(--card-shadow); }
  .btn.primary { background:var(--primary); color:#001018; border-color:var(--primary); font-weight:600; }
  .btn.primary:hover { background:var(--primary-hover); }
  .btn.ghost { background:transparent; }
  .btn:disabled { opacity:.5; pointer-events:none; }
  .btn.mini { padding:.25em .6em; font-size:.8rem; border-radius:8px; }
  .icon { font-size:1.1em; }
  .spinner { width:14px; height:14px; border:2px solid rgba(0,0,0,.2); border-top-color:currentColor; border-radius:50%; display:inline-block; animation:spin .7s linear infinite; }
  @keyframes spin { to{transform:rotate(360deg)} }
  .folders, .files { list-style:none; padding:0; margin:.5rem 0; }
  .folders li { background:var(--bg); border:1px solid var(--border); padding:.4em .6em; border-radius:10px; margin:.25em 0; display:flex; justify-content:space-between; align-items:center; font-size:.9rem; }
  .files li { display:flex; gap:.5rem; align-items:center; padding:.45em .5em; border-bottom:1px solid var(--border); font-size:.85rem; border-radius:8px; }
  .files li.keeper { background:var(--keep-bg); border:1px solid var(--border); }
  .keep { font-weight:700; font-size:.7rem; padding:.15em .45em; border-radius:6px; background:var(--border); }
  .keeper .keep { background:var(--primary); color:#001018; }
  .path { flex:1; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }
  .size { color:var(--muted); white-space:nowrap; }
  .group { border:1px solid var(--border); border-radius:12px; margin:.6rem 0; padding:.6rem; background:var(--card); }
  .group summary { cursor:pointer; display:flex; justify-content:space-between; gap:.5rem; align-items:center; flex-wrap:wrap; list-style:none; }
  .group summary::-webkit-details-marker { display:none; }
  .group-meta { display:flex; gap:.4em; align-items:center; flex-wrap:wrap; font-size:.9rem; }
  .dot { color:var(--muted); }
  .wasted { color:#ef4444; font-weight:600; }
  :root[data-theme="dark"] .wasted { color:#fca5a5; }
  .hash { font-family:monospace; font-size:.8rem; color:var(--muted); }
  .error { background:var(--error-bg); border:1px solid var(--error-border); padding:.7em 1em; border-radius:10px; margin:.5rem 0; }
  .hint { color:var(--muted); font-size:.85rem; }
  .empty { text-align:center; padding:1.2rem; }
  .empty-icon { font-size:2rem; margin-bottom:.3rem; }
  footer { text-align:center; margin-top:2rem; color:var(--muted); font-size:.8rem; }
  .skill { background:var(--bg); border:1px solid var(--border); padding:.1em .4em; border-radius:6px; font-size:.75rem; }
  input { border:1px solid var(--border); background:var(--card); color:var(--text); padding:.5em .7em; border-radius:10px; }
  input::placeholder { color:var(--muted); }
  a { color:var(--primary); }
</style>
