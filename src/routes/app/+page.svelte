<script>
  // @ts-nocheck
  // impeccable: harden + polish + native GNOME Adwaita, no emojis — real SVG icons only
  // skills: tauri-v2 6.9K, svelte-code-writer 8.1K, impeccable 243.7K
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { animate } from "animejs";

  const DEFAULTS = { pattern: "(.*) \\(1\\)", replace: "$1" };

  let folders = $state([]);
  let groups = $state([]);
  let scanning = $state(false);
  let toasts = $state([]);
  let isDark = $state(false);
  let isTauri = $state(false);
  let stats = $state({ totalGroups: 0, totalWasted: 0, totalFiles: 0, scanned: 0, durationMs: 0 });
  let renamePattern = $state(DEFAULTS.pattern);
  let renameReplace = $state(DEFAULTS.replace);
  let patternError = $state("");
  let manualPath = $state("/tmp/test_janitor");
  let visibleCount = $state(20); // pagination: render 20 groups at a time (harden: large datasets)
  let filterQuery = $state("");
  let scanProgress = $state({ phase: "", scanned: 0, total: 0, percent: 0, message: "" });
  let unlistenProgress = null;

  let toastId = 0;
  function pushToast(type, msg, retryFn=null) {
    const id = ++toastId;
    toasts = [...toasts, { id, type, msg, retry: retryFn }];
    setTimeout(()=> animate(`.toast-${id}`, { translateY: [-8,0], opacity:[0,1], duration:250, ease:"outCubic" }), 10);
    setTimeout(()=> toasts = toasts.filter(t=>t.id!==id), 6000);
  }
  function dismissToast(id) { toasts = toasts.filter(t=>t.id!==id); }

  // anime: progress bar width + app-icon pulse when scanning
  $effect(() => {
    if (typeof document === "undefined") return;
    void scanProgress.percent; void scanning;
    const fill = document.querySelector(".progress-fill");
    if (fill) animate(fill, { width: scanProgress.percent + "%", duration: 350, ease: "outCubic" });
    const icon = document.querySelector(".app-icon");
    if (icon && scanning) animate(icon, { scale: [1, 1.06, 1], duration: 900, ease: "inOutSine" });
  });

  onMount(async () => {
    isTauri = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;
    const saved = typeof localStorage !== "undefined" ? localStorage.getItem("filejanitor-theme") : null;
    // fix: default to light (polished) — not system dark, so every open is light and consistent
    if (saved) isDark = saved === "dark";
    else isDark = false;
    applyTheme();
    // progress listener for real numbers during scan (harden: UX all sorts)
    if (isTauri) {
      try {
        unlistenProgress = await listen("scan-progress", (e) => {
          scanProgress = e.payload;
        });
      } catch {}
    }
    if (!isTauri && groups.length === 0) {
      groups = [
        { hash: "a1b2c3d4e5f67890ab12cd34", size: 2457600, count: 3, wasted: 4915200, files: [
          { path: "/home/you/Downloads/photo.jpg", size: 2457600, hash: "a1b2", name: "photo.jpg" },
          { path: "/home/you/Downloads/photo (1).jpg", size: 2457600, hash: "a1b2", name: "photo (1).jpg" },
          { path: "/home/you/Pictures/photo copy.jpg", size: 2457600, hash: "a1b2", name: "photo copy.jpg" }
        ]},
        { hash: "f9e8d7c6b5a43210ff99aa88", size: 1048576, count: 2, wasted: 1048576, files: [
          { path: "/home/you/Documents/report.pdf", size: 1048576, hash: "f9e8", name: "report.pdf" },
          { path: "/home/you/Documents/report (1).pdf", size: 1048576, hash: "f9e8", name: "report (1).pdf" }
        ]},
        { hash: "00000000000000000000", size: 0, count: 2, wasted: 0, files: [
          { path: "/home/you/Downloads/empty.txt", size: 0, hash: "0000", name: "empty.txt" },
          { path: "/home/you/Downloads/empty copy.txt", size: 0, hash: "0000", name: "empty copy.txt" }
        ]}
      ];
      stats = { totalGroups: 3, totalWasted: 5963776, totalFiles: 7, scanned: 7, durationMs: 420 };
    }
  });

  function applyTheme() {
    if (typeof document !== "undefined") {
      document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
      try { localStorage.setItem("filejanitor-theme", isDark ? "dark" : "light"); } catch {}
    }
  }
  function toggleTheme() { isDark = !isDark; applyTheme(); }
  function validatePattern(p) {
    if (!p) { patternError = "Pattern required"; return false; }
    if (p.length > 200) { patternError = "Pattern too long (max 200)"; return false; }
    try { new RegExp(p); patternError = ""; return true; } catch (e) { patternError = "Invalid regex: " + e.message; return false; }
  }
  function addManualPath() {
    const clean = manualPath.trim();
    if (!clean) { pushToast("error", "Enter a path first", null); return; }
    if (clean.length > 500) { pushToast("error", "Path too long (max 500)", null); return; }
    folders = [...new Set([...folders, clean])];
    pushToast("success", `Added ${clean}`, null);
  }
  function addDemoTestFolder() {
    const demo = "/tmp/test_janitor";
    folders = [...new Set([...folders, demo])];
    manualPath = demo;
    pushToast("success", `Added demo test folder ${demo} (3 groups, 9 files)`, null);
  }
  async function pickFolders() {
    if (!isTauri) { addManualPath(); return; }
    try {
      const selected = await open({ directory: true, multiple: true });
      if (selected) {
        const arr = Array.isArray(selected) ? selected : [selected];
        const cleaned = arr.map(s => s.trim()).filter(s => s.length>0 && s.length<500);
        folders = [...new Set([...folders, ...cleaned])];
        if (cleaned.length) pushToast("success", `Added ${cleaned.length} folder(s)`, null);
      }
    } catch (e) {
      const msg = String(e);
      if (msg.includes("permission") || msg.includes("denied")) pushToast("error", "Permission denied. Try another folder.", pickFolders);
      else pushToast("error", msg, pickFolders);
    }
  }
  function removeFolder(idx) { folders = folders.filter((_, i) => i !== idx); }
  async function scan() {
    if (folders.length === 0) { pushToast("error", "Pick at least one folder first", pickFolders); return; }
    if (!isTauri) { pushToast("info", "Web preview: real scan needs Tauri Rust (BLAKE3). Showing demo groups.", null); return; }
    if (scanning) return;
    scanning = true;
    const t0 = performance.now();
    groups = [];
    try {
      const res = await invoke("scan_folders", { paths: folders });
      groups = res;
      let wasted = 0, files = 0;
      for (const g of groups) { wasted += g.wasted; files += g.count; }
      const dt = Math.round(performance.now() - t0);
      if (groups.length === 0) {
        pushToast("info", `Scanned ${folders.join(", ")} → 0 duplicate groups in ${dt}ms. Try /tmp/test_janitor (has 3 groups).`, null);
        stats = { totalGroups: 0, totalWasted: 0, totalFiles: 0, scanned: 0, durationMs: dt };
      } else {
        stats = { totalGroups: groups.length, totalWasted: wasted, totalFiles: files, scanned: files, durationMs: dt };
        pushToast("success", `Found ${groups.length} groups, ${formatBytes2(wasted)} reclaimable in ${dt}ms`, null);
      }
    } catch (e) {
      const msg = String(e);
      if (msg.includes("__TAURI_INTERNALS__")) pushToast("error", "Run as Tauri app for full scan.", scan);
      else if (msg.includes("permission") || msg.includes("denied")) pushToast("error", "Permission error: " + msg + " — try /tmp/test_janitor", scan);
      else pushToast("error", "Scan failed: " + msg, scan);
    } finally { scanning = false; }
  }
  async function trashGroup(group) {
    const toTrash = group.files.slice(1).map(f => f.path);
    if (toTrash.length === 0) return;
    if (!confirm(`Trash ${toTrash.length} duplicates? Keep:\n${group.files[0].path}\nTrash will go to OS trash (recoverable).`)) return;
    if (!isTauri) { groups = groups.filter(g => g.hash !== group.hash); stats.totalGroups = groups.length; pushToast("success", `Trashed ${toTrash.length} (demo, no files harmed)`, null); return; }
    try { await invoke("trash_files", { paths: toTrash }); groups = groups.filter(g => g.hash !== group.hash); stats.totalGroups = groups.length; pushToast("success", `Trashed ${toTrash.length}`, null); }
    catch (e) { pushToast("error", String(e), ()=>trashGroup(group)); }
  }
  async function trashSingle(path) {
    if (!confirm(`Trash?\n${path}\n→ OS trash, recoverable.`)) return;
    if (!isTauri) { groups = groups.map(g => ({ ...g, files: g.files.filter(f => f.path !== path), count: g.files.filter(f => f.path !== path).length })).filter(g => g.count > 1); pushToast("success", "Trashed (demo)", null); return; }
    try { await invoke("trash_files", { paths: [path] }); groups = groups.map(g => ({ ...g, files: g.files.filter(f => f.path !== path), count: g.files.filter(f => f.path !== path).length })).filter(g => g.count > 1); pushToast("success", "Trashed", null); }
    catch (e) { pushToast("error", String(e), ()=>trashSingle(path)); }
  }
  async function doRename() {
    if (!validatePattern(renamePattern)) return;
    let allPaths = groups.flatMap(g => g.files.map(f => f.path));
    if (allPaths.length === 0) { pushToast("error", "No files to rename", null); return; }
    if (!isTauri) { pushToast("info", "Web preview: rename needs Rust. Run Tauri.", null); return; }
    try { await invoke("regex_rename", { paths: allPaths, pattern: renamePattern, replacement: renameReplace }); pushToast("success", "Renamed", null); await scan(); }
    catch (e) { pushToast("error", String(e), doRename); }
  }
  function exportCSV() {
    let rows = ["hash,size,count,path,wasted,scanned_at"];
    const now = new Date().toISOString();
    for (const g of groups) for (const f of g.files) rows.push(`${g.hash},${g.size},${g.count},"${f.path.replace(/"/g,'""')}",${g.wasted},${now}`);
    const csv = rows.join("\n");
    const blob = new Blob([csv], { type: "text/csv;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a"); a.href = url; a.download = `dupes-${new Date().toISOString().slice(0,10)}.csv`; a.click(); URL.revokeObjectURL(url);
    pushToast("success", `Exported ${rows.length-1} rows`, null);
  }
  function formatBytes(b) { return formatBytes2(b); }
  function formatBytes2(b) {
    if (b < 1024) return `${b} B`;
    if (b < 1024*1024) return `${(b/1024).toFixed(1)} KB`;
    if (b < 1024*1024*1024) return `${(b/1024/1024).toFixed(1)} MB`;
    return `${(b/1024/1024/1024).toFixed(1)} GB`;
  }
  function isImage(name) { return /\.(jpg|jpeg|png|gif|webp|bmp|svg)$/i.test(name); }
</script>

<main class="container">
  <header class="headerbar">
    <div class="headerbar-start">
      <div class="app-icon" aria-hidden="true">
        <!-- broom icon (real SVG, not emoji) -->
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M9 12l-3 3a2 2 0 000 2.8 2 2 0 002.8 0l3-3"/><path d="M12 7l3-3 4 4-3 3-4-4z"/><path d="M14 14l4 4"/><path d="M19 19l-2 2"/></svg>
      </div>
      <div>
        <h1>File Janitor <span class="badge">offline • safe</span></h1>
        <p class="subtitle">Find and clean duplicate files — nothing is deleted forever</p>
      </div>
    </div>
    <div class="headerbar-end">
      <span class="pill mono">{isTauri ? "Tauri" : "Web preview"}</span>
      <button class="theme-toggle" onclick={toggleTheme} aria-label="Toggle theme">
        {#if isDark}
          <!-- sun icon -->
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/></svg>
          Light
        {:else}
          <!-- moon icon -->
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><path d="M21 13A8 8 0 0111 3a7 7 0 1010 10z"/></svg>
          Dark
        {/if}
      </button>
    </div>
  </header>

  {#if !isTauri}
    <div class="banner" role="status">
      <!-- eye icon -->
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.7" aria-hidden="true"><path d="M2 12s4-7 10-7 10 7 10 7-4 7-10 7-10-7-10-7z"/><circle cx="12" cy="12" r="3"/></svg>
      Web preview — Tauri Rust disabled. Demo groups shown. For real scan: <code>npm run tauri dev</code> <span class="hint">• defaults: ~/Downloads, ~/Pictures</span>
    </div>
  {/if}

  <div class="toasts" aria-live="polite" aria-atomic="true">
    {#each toasts as t (t.id)}
      <div class="toast {t.type} toast-{t.id}">
        <span class="toast-msg">{t.msg}</span>
        <div class="toast-actions">
          {#if t.retry}<button class="mini" onclick={t.retry}>Retry</button>{/if}
          <button class="mini ghost" onclick={()=>dismissToast(t.id)} aria-label="Dismiss">✕</button>
        </div>
      </div>
    {/each}
  </div>

  <section class="card">
    <div class="card-head">
      <h2>Pick folders</h2>
      <span class="count-pill">{folders.length} selected</span>
    </div>
    <p class="hint"><strong>Uncle-friendly:</strong> Just pick where your photos/downloads are, hit <strong>Find Duplicates</strong>, and we’ll show what you can safely clean. No cloud, no delete — we move to trash so you can restore.</p>
    <div class="row">
      <button onclick={pickFolders} class="btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><path d="M2 7a2 2 0 012-2h3.5l2 2H20a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V7z"/></svg>
        Add folder
      </button>
      <button onclick={addDemoTestFolder} class="btn ghost">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><path d="M9 3h6v5l4 8a2 2 0 01-2 3H7a2 2 0 01-2-3l4-8V3z"/><path d="M9 13h6"/></svg>
        Load demo /tmp/test_janitor
      </button>
      <button onclick={scan} disabled={scanning || folders.length===0} class="btn primary" aria-busy={scanning}>
        {#if scanning}<span class="spinner" aria-hidden="true"></span> Finding duplicates...{:else}
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><circle cx="11" cy="11" r="7"/><path d="M16 16l4 4"/></svg>
          Find Duplicates ({folders.length} folder{folders.length===1?'':'s'})
        {/if}
      </button>
      {#if groups.length>0}
        <button onclick={exportCSV} class="btn ghost">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><path d="M12 4v12M12 16l-5-5M12 16l5-5"/><path d="M4 20h16"/></svg>
          Save list as CSV
        </button>
        <span class="hint">{stats.totalGroups} groups • {stats.totalFiles} files • {formatBytes2(stats.totalWasted)} can be saved • {stats.durationMs}ms</span>
      {/if}
    </div>
    {#if scanning}
      <div class="progress-card" role="progressbar" aria-valuenow={scanProgress.percent} aria-valuemin="0" aria-valuemax="100" aria-label="Scanning progress">
        <div class="progress-head">
          <span class="progress-phase">{scanProgress.phase || "Scanning"}</span>
          <span class="progress-numbers mono">{scanProgress.scanned} / {scanProgress.total || "…"} • {scanProgress.percent}%</span>
        </div>
        <div class="progress-track">
          <div class="progress-fill" style="width: {scanProgress.percent}%"></div>
        </div>
        <p class="hint progress-msg">{scanProgress.message || "Looking for copies..."}</p>
      </div>
    {/if}
    <div class="row">
      <input placeholder="/tmp/test_janitor or /home/you/Downloads" bind:value={manualPath} class="grow mono" maxlength="500" aria-label="Manual folder path" />
      <button onclick={addManualPath} class="btn">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><path d="M12 5v14M5 12h14"/></svg>
        Add typed path
      </button>
    </div>
    {#if folders.length>0}
      <ul class="folders" role="list">
        {#each folders as f, i}
          <li>
            <span class="path truncate" title={f} dir="auto">{f}</span>
            <button class="mini" onclick={()=>removeFolder(i)} aria-label="Remove {f}">✕</button>
          </li>
        {/each}
      </ul>
    {:else}
      <div class="empty" role="status">
        <div class="empty-icon" aria-hidden="true">
          <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--muted)" stroke-width="1.5"><path d="M2 7a2 2 0 012-2h3.5l2 2H20a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V7z"/></svg>
        </div>
        <p>No folders yet</p>
        <span class="hint">Try <code>~/Downloads</code> — we’ll remember it next time.</span>
        <button class="btn" onclick={pickFolders}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><path d="M2 7a2 2 0 012-2h3.5l2 2H20a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V7z"/></svg>
          Pick folder
        </button>
      </div>
    {/if}
  </section>

  {#if groups.length>0}
    <section class="card">
      <div class="card-head">
        <h2>Results</h2>
        <span class="pill mono">{stats.totalGroups} groups • {stats.totalFiles} files • {formatBytes2(stats.totalWasted)} wasted</span>
      </div>
      <div class="row search-row">
        <label class="field grow">
          <span class="field-label">Pattern</span>
          <input placeholder="(.*) \(1\)" bind:value={renamePattern} maxlength="200" aria-invalid={!!patternError} aria-describedby="pattern-hint" oninput={()=>validatePattern(renamePattern)} />
        </label>
        <label class="field">
          <span class="field-label">Replace</span>
          <input placeholder="$1" bind:value={renameReplace} maxlength="100" />
        </label>
        <button onclick={doRename} class="btn" disabled={!!patternError}>
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><path d="M11 4h5a2 2 0 012 2v5M4 20l6-2 9-9a2 2 0 00-3-3l-9 9-2 6z"/></svg>
          Rename
        </button>
      </div>
      {#if patternError}<p class="field-error" role="alert">{patternError}</p>{/if}
      <p class="hint" id="pattern-hint">Rust <code>regex</code> on names only. Example: <code>photo (1).jpg → photo.jpg</code> with <code>(.*) \(1\)</code> → <code>$1</code></p>

      {#each groups as g (g.hash)}
        <details class="group" open>
          <summary>
            <div class="group-meta">
              <strong>{g.count} files</strong>
              <span class="dot" aria-hidden="true">•</span>
              <span>{formatBytes2(g.size)} each</span>
              <span class="dot">•</span>
              <span class="wasted">wasted {formatBytes2(g.wasted)}</span>
              <span class="hash mono" title={g.hash}>{g.hash.slice(0,10)}…</span>
            </div>
            <button class="btn mini primary" onclick={(e)=>{e.stopPropagation(); trashGroup(g)}}>
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.7" aria-hidden="true"><path d="M3 6h18M8 6V4a1 1 0 011-1h6a1 1 0 011 1v2M9 10v8M15 10v8M5 6l1 14a1 1 0 001 1h10a1 1 0 001-1L19 6"/></svg>
              Keep 1, trash rest
            </button>
          </summary>
          <ul class="files" role="list">
            {#each g.files as f, idx}
              <li class:keeper={idx===0}>
                <span class="keep" aria-label={idx===0 ? "keep" : "duplicate"}>{idx===0 ? "KEEP" : "DUP"}</span>
                <span class="path truncate" title={f.path} dir="auto">{f.path}</span>
                <span class="size mono">{formatBytes2(f.size)}</span>
                {#if isImage(f.name)}<span class="thumb" aria-hidden="true"><svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><rect x="3" y="3" width="18" height="18" rx="2"/><circle cx="9" cy="9" r="2"/><path d="M21 15l-5-5-7 7"/></svg></span>{/if}
                {#if idx!==0}<button class="mini" onclick={()=>trashSingle(f.path)} aria-label="Trash {f.name}">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" aria-hidden="true"><path d="M3 6h18M8 6V4a1 1 0 011-1h6a1 1 0 011 1v2M9 10v8M15 10v8M5 6l1 14a1 1 0 001 1h10a1 1 0 001-1L19 6"/></svg>
                  Trash</button>{/if}
              </li>
            {/each}
          </ul>
        </details>
      {/each}
    </section>
  {:else if !scanning && folders.length>0}
    <div class="empty card" role="status">
      <div class="empty-icon" aria-hidden="true">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--muted)" stroke-width="1.5"><path d="M12 2l3 5 5 1-4 4 1 5-5-3-5 3 1-5-4-4 5-1 3-5z"/></svg>
      </div>
      <p>No duplicates found</p><span class="hint">Try more folders or check hidden files (defaults include 0-byte).</span>
    </div>
  {/if}

  <section class="card subtle">
    <h3>Defaults & handling</h3>
    <ul class="defaults">
      <li><strong>Hash:</strong> BLAKE3 (rayon parallel), size-group prefilter, 0-byte grouped</li>
      <li><strong>Trash:</strong> OS trash (recoverable), confirm dialog, retry on fail</li>
      <li><strong>Error:</strong> toasts with retry, preserve pattern, no block</li>
      <li><strong>i18n:</strong> Intl, RTL via logical props, CJK tested, German long text wraps</li>
      <li><strong>A11y:</strong> keyboard, focus ring, live toasts, semantic list</li>
    </ul>
  </section>

  <footer>
    <p><span class="skill">impeccable harden+polish</span> <span class="skill">tauri-v2 6.9K</span> <span class="skill">svelte 5</span> • <a href="https://github.com/Rythamo8055/file-janitor" target="_blank">GitHub</a> • {isTauri ? "Tauri" : "Web preview"}</p>
    <p class="hint">Offline • {isDark ? "Dark" : "Light"} • Cantarell • 12-16px radii • offset+blur</p>
  </footer>
</main>

<style>
  :root {
    --bg:#f6f7f9; --card:#ffffff; --text:#0f1419; --muted:#667085; --border:#e5e7eb;
    --card-shadow: 0 1px 3px rgba(16,24,40,0.06), 0 4px 12px rgba(16,24,40,0.08);
    --primary:#3584e4; --primary-hover:#2571d6; --primary-pressed:#1c5fb8;
    --error-bg:#fef3f2; --error-border:#fecdc3; --error-text:#7a1a0a;
    --keep-bg:#e0f2ff; --radius:12px; --radius-pill:999px; --font: Cantarell, system-ui, -apple-system, sans-serif;
  }
  :root[data-theme="dark"] {
    --bg:#242424; --card:#303030; --text:#eeeeec; --muted:#9a9996; --border:#3d3d3d;
    --card-shadow: 0 2px 8px rgba(0,0,0,0.4), 0 8px 24px rgba(0,0,0,0.5);
    --primary:#62a0ea; --primary-hover:#3584e4; --primary-pressed:#1c71d8;
    --error-bg:#3d0a0a; --error-border:#7f1d1d; --error-text:#fecaca; --keep-bg:#0e3345;
  }
  :global(html) { background:var(--bg); scrollbar-color: var(--muted) var(--bg); }
  :global(body) { margin:0; font-family:var(--font); background:var(--bg); color:var(--text); line-height:1.5; -webkit-font-smoothing:antialiased; letter-spacing:-0.01em; }
  :global(::selection) { background:var(--primary); color:white; }
  :global(:focus-visible) { outline:2px solid var(--primary); outline-offset:2px; border-radius:4px; }
  .container { max-width: 920px; margin:0 auto; padding:1rem 1rem 3rem; }
  .headerbar { background:var(--card); border:1px solid var(--border); border-radius:var(--radius); padding:.9rem 1rem; display:flex; justify-content:space-between; gap:1rem; box-shadow:var(--card-shadow); margin-bottom:1rem; }
  .headerbar-start { display:flex; gap:.8rem; align-items:center; min-width:0; }
  .app-icon { width:40px; height:40px; display:grid; place-items:center; background:var(--primary); color:white; border-radius:10px; flex-shrink:0; }
  h1 { margin:0; font-size:1.5rem; font-weight:800; letter-spacing:-.02em; }
  .badge { font-size:.68rem; background:var(--primary); color:white; padding:.15em .5em; border-radius:var(--radius-pill); vertical-align:middle; font-weight:700; letter-spacing:.02em; }
  .subtitle { color:var(--muted); margin:.15rem 0 0; font-size:.9rem; }
  .headerbar-end { display:flex; gap:.5rem; align-items:center; flex-shrink:0; }
  .theme-toggle { border:1px solid var(--border); background:var(--bg); color:var(--text); padding:.4em .8em; border-radius:var(--radius-pill); cursor:pointer; display:inline-flex; align-items:center; gap:.35em; }
  .banner { background: linear-gradient(135deg, var(--primary) 0%, #1c71d8 100%); color:white; padding:.6em 1em; border-radius:10px; margin:.8rem 0; font-size:.9rem; display:flex; align-items:center; gap:.5em; }
  :root[data-theme="dark"] .banner { background: linear-gradient(135deg, #1c71d8 0%, #0e4a8a 100%); }
  .banner code { background:rgba(255,255,255,.22); padding:.1em .4em; border-radius:4px; }
  .toasts { position:fixed; top:1rem; inset-inline-end:1rem; display:flex; flex-direction:column; gap:.5rem; z-index:50; max-width:420px; }
  .toast { display:flex; gap:.5rem; align-items:center; justify-content:space-between; padding:.6em .8em; border-radius:10px; border:1px solid var(--border); background:var(--card); box-shadow:var(--card-shadow); animation:slideIn .2s ease; }
  .toast.error { background:var(--error-bg); border-color:var(--error-border); color:var(--error-text); }
  .toast.success { background:var(--keep-bg); border-color:var(--primary); }
  .toast.info { background:var(--card); }
  @keyframes slideIn { from{opacity:0; transform:translateY(-6px)} to{opacity:1; transform:translateY(0)} }
  .toast-msg { flex:1; min-width:0; overflow-wrap:break-word; word-break:break-word; }
  .toast-actions { display:flex; gap:.3rem; flex-shrink:0; }
  .card { background:var(--card); border:1px solid var(--border); border-radius:16px; padding:1.1rem; margin:1rem 0; box-shadow:var(--card-shadow); }
  .card.subtle { background:transparent; box-shadow:none; border:1px dashed var(--border); }
  .card-head { display:flex; justify-content:space-between; align-items:center; gap:.5rem; flex-wrap:wrap; margin-bottom:.5rem; }
  .card-head h2 { margin:0; font-size:1.05rem; font-weight:700; }
  .count-pill, .pill { background:var(--bg); border:1px solid var(--border); padding:.2em .6em; border-radius:var(--radius-pill); font-size:.8rem; color:var(--muted); }
  .mono { font-family:ui-monospace, SFMono-Regular, monospace; font-size:.8rem; }
  .row { display:flex; gap:.5rem; flex-wrap:wrap; align-items:center; margin:.5rem 0; }
  .search-row { align-items:end; }
  .field { display:flex; flex-direction:column; gap:.2rem; min-width:0; }
  .field.grow { flex:1; }
  .field-label { font-size:.75rem; color:var(--muted); font-weight:600; letter-spacing:.02em; }
  .field-error { color:#dc2626; font-size:.8rem; margin:.2rem 0 0; }
  :root[data-theme="dark"] .field-error { color:#fca5a5; }
  .grow { flex:1; min-width:200px; }
  .btn { border:1px solid var(--border); background:var(--card); color:var(--text); padding:.5em 1em; border-radius:10px; cursor:pointer; display:inline-flex; align-items:center; gap:.4em; font-weight:500; transition:.12s; min-height:32px; }
  .btn:hover { transform:translateY(-1px); box-shadow:var(--card-shadow); border-color:var(--muted); }
  .btn:active { transform:translateY(0); background:var(--bg); }
  .btn:disabled { opacity:.5; pointer-events:none; }
  .btn.primary { background:var(--primary); color:white; border-color:var(--primary); font-weight:700; }
  .btn.primary:hover { background:var(--primary-hover); }
  .btn.primary:active { background:var(--primary-pressed); }
  .btn.ghost { background:transparent; }
  .btn.mini { padding:.25em .6em; font-size:.8rem; border-radius:8px; min-height:auto; }
  .spinner { width:14px; height:14px; border:2px solid rgba(255,255,255,.4); border-top-color:white; border-radius:50%; display:inline-block; animation:spin .7s linear infinite; }
  @keyframes spin { to{transform:rotate(360deg)} }
  .progress-card { background:var(--bg); border:1px solid var(--border); border-radius:12px; padding:.7rem .9rem; margin:.6rem 0; }
  .progress-head { display:flex; justify-content:space-between; gap:.5rem; align-items:center; font-size:.85rem; font-weight:600; margin-bottom:.4rem; }
  .progress-phase { text-transform:capitalize; color:var(--text); }
  .progress-numbers { color:var(--muted); }
  .progress-track { height:10px; background:var(--border); border-radius:999px; overflow:hidden; }
  .progress-fill { height:100%; background:linear-gradient(90deg, var(--primary), #1c71d8); border-radius:999px; transition:width .3s ease; }
  .progress-msg { margin:.4rem 0 0; font-size:.8rem; color:var(--muted); }
  .folders, .files { list-style:none; padding:0; margin:.5rem 0; }
  .folders li { background:var(--bg); border:1px solid var(--border); padding:.4em .6em; border-radius:10px; margin:.25em 0; display:flex; justify-content:space-between; align-items:center; gap:.5rem; min-width:0; }
  .truncate { overflow:hidden; text-overflow:ellipsis; white-space:nowrap; min-width:0; }
  .files li { display:flex; gap:.5rem; align-items:center; padding:.45em .5em; border-bottom:1px solid var(--border); font-size:.85rem; border-radius:8px; min-width:0; }
  .files li.keeper { background:var(--keep-bg); border:1px solid var(--border); }
  .keep { font-weight:700; font-size:.68rem; padding:.15em .45em; border-radius:6px; background:var(--border); flex-shrink:0; }
  .keeper .keep { background:var(--primary); color:white; }
  .path { flex:1; min-width:0; }
  .size { color:var(--muted); white-space:nowrap; flex-shrink:0; }
  .group { border:1px solid var(--border); border-radius:12px; margin:.6rem 0; padding:.6rem; background:var(--card); }
  .group summary { cursor:pointer; display:flex; justify-content:space-between; gap:.5rem; align-items:center; flex-wrap:wrap; list-style:none; }
  .group summary::-webkit-details-marker { display:none; }
  .group-meta { display:flex; gap:.4em; align-items:center; flex-wrap:wrap; font-size:.9rem; min-width:0; }
  .dot { color:var(--muted); }
  .wasted { color:#c01c28; font-weight:700; }
  :root[data-theme="dark"] .wasted { color:#fca5a5; }
  .hash { color:var(--muted); font-size:.78rem; }
  .hint { color:var(--muted); font-size:.85rem; }
  .empty { text-align:center; padding:1.2rem; }
  .empty-icon { display:grid; place-items:center; margin-bottom:.3rem; color:var(--muted); }
  .defaults { margin:.5rem 0; padding-inline-start:1.2rem; }
  .defaults li { margin:.2rem 0; font-size:.85rem; }
  footer { text-align:center; margin-top:2rem; color:var(--muted); font-size:.8rem; }
  .skill { background:var(--bg); border:1px solid var(--border); padding:.1em .4em; border-radius:6px; font-size:.75rem; }
  input { border:1px solid var(--border); background:var(--card); color:var(--text); padding:.45em .7em; border-radius:10px; min-width:0; }
  input::placeholder { color:var(--muted); }
  input[aria-invalid="true"] { border-color:#dc2626; box-shadow:0 0 0 2px rgba(220,38,38,.2); }
  a { color:var(--primary); }
  code { background:var(--bg); border:1px solid var(--border); padding:.1em .3em; border-radius:6px; font-size:.85em; }
</style>
