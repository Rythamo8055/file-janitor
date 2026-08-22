<script>
  // @ts-nocheck
  // skill: svelte-code-writer 8.1K, tauri-v2 6.9K - Svelte 5 runes + invoke
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let folders = $state([]);
  let groups = $state([]);
  let scanning = $state(false);
  let error = $state("");
  let stats = $state({ totalGroups: 0, totalWasted: 0, totalFiles: 0 });
  let renamePattern = $state("(.*) \\(1\\)");
  let renameReplace = $state("$1");
  let selectedPaths = $state(new Set());

  async function pickFolders() {
    try {
      const selected = await open({ directory: true, multiple: true });
      if (selected) {
        const arr = Array.isArray(selected) ? selected : [selected];
        folders = [...new Set([...folders, ...arr])];
      }
    } catch (e) {
      error = String(e);
    }
  }

  function removeFolder(idx) {
    folders = folders.filter((_, i) => i !== idx);
  }

  async function scan() {
    if (folders.length === 0) {
      error = "Pick at least one folder first";
      return;
    }
    scanning = true;
    error = "";
    groups = [];
    try {
      const res = await invoke("scan_folders", { paths: folders });
      groups = res;
      let wasted = 0, files = 0;
      for (const g of groups) {
        wasted += g.wasted;
        files += g.count;
      }
      stats = { totalGroups: groups.length, totalWasted: wasted, totalFiles: files };
    } catch (e) {
      error = String(e);
    } finally {
      scanning = false;
    }
  }

  async function trashGroup(group) {
    // keep first file, trash rest
    const toTrash = group.files.slice(1).map(f => f.path);
    if (toTrash.length === 0) return;
    try {
      await invoke("trash_files", { paths: toTrash });
      // remove trashed from groups
      groups = groups.filter(g => g.hash !== group.hash);
      stats.totalGroups = groups.length;
    } catch (e) {
      error = String(e);
    }
  }

  async function trashSingle(path) {
    try {
      await invoke("trash_files", { paths: [path] });
      // remove from groups
      groups = groups.map(g => ({
        ...g,
        files: g.files.filter(f => f.path !== path),
        count: g.files.filter(f => f.path !== path).length
      })).filter(g => g.count > 1);
    } catch (e) {
      error = String(e);
    }
  }

  async function doRename() {
    let allPaths = groups.flatMap(g => g.files.map(f => f.path));
    if (allPaths.length === 0) return;
    try {
      await invoke("regex_rename", { paths: allPaths, pattern: renamePattern, replacement: renameReplace });
      await scan();
    } catch (e) {
      error = String(e);
    }
  }

  function exportCSV() {
    let rows = ["hash,size,count,path,wasted"];
    for (const g of groups) {
      for (const f of g.files) {
        rows.push(`${g.hash},${g.size},${g.count},"${f.path}",${g.wasted}`);
      }
    }
    const csv = rows.join("\n");
    const blob = new Blob([csv], { type: "text/csv" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "dupes.csv";
    a.click();
    URL.revokeObjectURL(url);
  }

  function formatBytes(b) {
    if (b < 1024) return b + " B";
    if (b < 1024*1024) return (b/1024).toFixed(1) + " KB";
    if (b < 1024*1024*1024) return (b/1024/1024).toFixed(1) + " MB";
    return (b/1024/1024/1024).toFixed(1) + " GB";
  }

  function isImage(name) {
    return /\.(jpg|jpeg|png|gif|webp|bmp)$/i.test(name);
  }
</script>

<main class="container">
  <header>
    <h1>File Janitor <span class="badge">offline • 5-15MB</span></h1>
    <p class="subtitle">Lightest duplicate finder. Rust BLAKE3 hashing, no cloud. Fedora-first, Win/Mac next.</p>
  </header>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <section class="card">
    <h2>1. Pick folders</h2>
    <div class="row">
      <button onclick={pickFolders}>+ Add folder</button>
      <button onclick={scan} disabled={scanning || folders.length===0} class="primary">
        {#if scanning}Scanning...{:else}Scan {folders.length} folder(s){/if}
      </button>
      {#if groups.length>0}
        <button onclick={exportCSV}>Export CSV</button>
      {/if}
    </div>
    {#if folders.length>0}
      <ul class="folders">
        {#each folders as f, i}
          <li>{f} <button class="mini" onclick={()=>removeFolder(i)}>✕</button></li>
        {/each}
      </ul>
    {:else}
      <p class="hint">No folders yet - pick ~/Downloads or ~/Photos</p>
    {/if}
  </section>

  {#if groups.length>0}
    <section class="card">
      <h2>2. Results — {stats.totalGroups} groups • {stats.totalFiles} files • {formatBytes(stats.totalWasted)} wasted</h2>
      <div class="row">
        <input placeholder="regex pattern e.g. (.*) \(1\)" bind:value={renamePattern} class="grow" />
        <input placeholder="replacement e.g. $1" bind:value={renameReplace} />
        <button onclick={doRename}>Bulk rename preview</button>
      </div>
      <p class="hint">Rename uses Rust regex crate on file names only. Keep-one = first file per group.</p>

      {#each groups as g (g.hash)}
        <details class="group" open>
          <summary>
            <strong>{g.count} files</strong> • {formatBytes(g.size)} each • wasted {formatBytes(g.wasted)} • {g.hash.slice(0,12)}...
            <button class="mini primary" onclick={(e)=>{e.stopPropagation(); trashGroup(g)}}>Trash duplicates (keep 1)</button>
          </summary>
          <ul class="files">
            {#each g.files as f, idx}
              <li class:keeper={idx===0}>
                <span class="keep">{idx===0 ? "KEEP" : "DUP"}</span>
                <span class="path">{f.path}</span>
                <span class="size">{formatBytes(f.size)}</span>
                {#if isImage(f.name)}
                  <span class="thumb">🖼️ {f.name}</span>
                {/if}
                {#if idx!==0}
                  <button class="mini" onclick={()=>trashSingle(f.path)}>Trash</button>
                {/if}
              </li>
            {/each}
          </ul>
        </details>
      {/each}
    </section>
  {:else if !scanning && folders.length>0}
    <p class="hint">No duplicates found yet. Try adding more folders or check scan.</p>
  {/if}

  <footer>
    <p>Skills: <code>tauri-v2</code> 6.9K <code>svelte-code-writer</code> 8.1K | Spec: <code>docs/file-janitor-spec.md</code> | <a href="https://github.com/Rythamo8055/file-janitor" target="_blank">GitHub</a></p>
    <p class="hint">Offline • trash not delete • BLAKE3 hashing • SQLite index coming (Task 4)</p>
  </footer>
</main>

<style>
  :root { font-family: Inter, system-ui, sans-serif; background: #f6f6f6; color: #0f0f0f; }
  .container { max-width: 900px; margin: 0 auto; padding: 2rem 1rem; }
  header h1 { margin: 0; font-size: 1.8rem; }
  .badge { font-size: 0.7rem; background: #24c8db; color: white; padding: 0.2em 0.6em; border-radius: 1em; vertical-align: middle; }
  .subtitle { color: #666; margin: 0.2rem 0 1rem; }
  .card { background: white; border-radius: 12px; padding: 1rem; margin: 1rem 0; box-shadow: 0 2px 8px rgba(0,0,0,0.06); }
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.5rem 0; }
  .grow { flex: 1; min-width: 200px; }
  button { border: 1px solid #ddd; background: white; padding: 0.5em 1em; border-radius: 8px; cursor: pointer; }
  button.primary { background: #24c8db; color: white; border-color: #24c8db; }
  button:disabled { opacity: 0.5; }
  button.mini { padding: 0.2em 0.5em; font-size: 0.8rem; }
  .folders, .files { list-style: none; padding: 0; margin: 0.5rem 0; }
  .folders li { background: #f0f0f0; padding: 0.3em 0.6em; border-radius: 6px; margin: 0.2em 0; display: flex; justify-content: space-between; font-size: 0.9rem; }
  .files li { display: flex; gap: 0.5rem; align-items: center; padding: 0.4em; border-bottom: 1px solid #eee; font-size: 0.85rem; }
  .files li.keeper { background: #e6f7f9; }
  .keep { font-weight: 700; font-size: 0.7rem; padding: 0.1em 0.4em; border-radius: 4px; background: #ddd; }
  .keeper .keep { background: #24c8db; color: white; }
  .path { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .size { color: #666; }
  .group { border: 1px solid #eee; border-radius: 8px; margin: 0.5rem 0; padding: 0.5rem; }
  .group summary { cursor: pointer; display: flex; gap: 0.5rem; align-items: center; flex-wrap: wrap; }
  .error { background: #ffeaea; border: 1px solid #ffb3b3; padding: 0.6em; border-radius: 8px; margin: 0.5rem 0; }
  .hint { color: #888; font-size: 0.85rem; }
  footer { text-align: center; margin-top: 2rem; color: #888; font-size: 0.8rem; }
  input { border: 1px solid #ddd; padding: 0.5em; border-radius: 8px; }
</style>
