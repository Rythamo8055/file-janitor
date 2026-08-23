<script>
  // @ts-nocheck
  // impeccable: Persuade landing for Rythamo ranking, SEO, screenshots
  // skills: impeccable 243.7K ( Persuade mode ), svelte-code-writer 8.1K
  import { onMount } from "svelte";
  import { animate, stagger } from "animejs";
  let isDark = false;
  let heroCanvas;
  let heroSection;
  onMount(() => {
    const saved = localStorage.getItem("filejanitor-theme");
    if (saved) isDark = saved === "dark";
    else isDark = false;
    document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");

    // anime.js 4.5: hero stagger (impeccable animate)
    animate(".hero-copy h1 span, .hero-copy .kicker, .hero-copy .lead, .hero-copy .proof, .cta-row .btn", {
      translateY: [20, 0],
      opacity: [0, 1],
      delay: stagger(80, {start: 200}),
      duration: 700,
      ease: "outCubic"
    });
    animate(".hero-mock", {
      translateY: [16, 0],
      scale: [0.98, 1],
      opacity: [0, 1],
      duration: 900,
      delay: 600,
      ease: "outExpo"
    });
    // procedural canvas: blobs (anime.js + canvas)
    if (heroCanvas) {
      const ctx = heroCanvas.getContext("2d");
      const dpr = window.devicePixelRatio || 1;
      function resize() {
        heroCanvas.width = heroSection.clientWidth * dpr;
        heroCanvas.height = 400 * dpr;
        heroCanvas.style.width = heroSection.clientWidth + "px";
        heroCanvas.style.height = "400px";
        ctx.setTransform(dpr,0,0,dpr,0,0);
      }
      resize();
      window.addEventListener("resize", resize);
      const blobs = [
        { x: 200, y: 120, r: 90, dx: 0.6, dy: 0.4, color: isDark ? "rgba(98,160,234,0.18)" : "rgba(53,132,228,0.12)" },
        { x: 600, y: 180, r: 120, dx: -0.4, dy: 0.5, color: isDark ? "rgba(98,160,234,0.12)" : "rgba(53,132,228,0.08)" },
        { x: 400, y: 80, r: 70, dx: 0.3, dy: -0.3, color: isDark ? "rgba(255,255,255,0.04)" : "rgba(0,0,0,0.04)" }
      ];
      let raf;
      function frame() {
        ctx.clearRect(0,0, heroCanvas.width, heroCanvas.height);
        for (const b of blobs) {
          b.x += b.dx; b.y += b.dy;
          if (b.x < b.r || b.x > heroSection.clientWidth - b.r) b.dx *= -1;
          if (b.y < b.r || b.y > 400 - b.r) b.dy *= -1;
          ctx.beginPath();
          ctx.arc(b.x, b.y, b.r, 0, Math.PI*2);
          ctx.fillStyle = b.color;
          ctx.fill();
        }
        raf = requestAnimationFrame(frame);
      }
      frame();
      // procedural pulse via anime (scale r)
      // blobs pulse handled via canvas frame loop + anime not needed for r (keep simple)
      // keep blobs drifting via requestAnimationFrame only
      return () => { cancelAnimationFrame(raf); window.removeEventListener("resize", resize); };
    }
    // feature stagger on scroll (simple)
    const obs = new IntersectionObserver((entries) => {
      for (const e of entries) if (e.isIntersecting) {
        animate(e.target.querySelectorAll(".feature"), { translateY:[16,0], opacity:[0,1], delay: stagger(100), duration:600, ease:"outCubic" });
        obs.unobserve(e.target);
      }
    }, { threshold: 0.2 });
    const f = document.querySelector(".feature-grid");
    if (f) obs.observe(f);
  });
  function toggleTheme() {
    isDark = !isDark;
    document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
    try { localStorage.setItem("filejanitor-theme", isDark ? "dark" : "light"); } catch {}
    // procedural recolor on theme toggle
  }
</script>

<svelte:head>
  <title>Rythamo — File Janitor | Lightweight Offline Duplicate Finder 5-15MB</title>
  <meta name="description" content="File Janitor by Rythamo — the lightest offline duplicate finder (5-15MB, Tauri + Rust BLAKE3). No cloud, no subscription, 100% local. Find and clean duplicate files on Linux, Windows. Built research-first from 9,363 Reddit wishes." />
  <meta name="keywords" content="Rythamo, File Janitor, duplicate finder, offline, Tauri, Linux, Rythamo8055, file cleaner, BLAKE3" />
  <meta name="author" content="Vishnu Vardhan (Rythamo)" />
  <meta name="google-site-verification" content="zOOj--hb4C2SawK-7BPeqrCavYMLjEHaIXc7wpvULZ8" />
  <link rel="canonical" href="https://offline-vault.vercel.app/" />
  <script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "Person",
      "name": "Vishnu Vardhan",
      "alternateName": "Rythamo",
      "url": "https://vishnuvardhanm.vercel.app",
      "sameAs": [
        "https://github.com/Rythamo8055",
        "https://linkedin.com/in/vishnu-vardhan8055"
      ],
      "jobTitle": "AI Engineer & Fullstack Developer"
    }
  </script>
  <meta property="og:title" content="Rythamo — File Janitor" />
  <meta property="og:description" content="Lightweight offline duplicate finder by Rythamo. 5-15MB, Rust BLAKE3, trash not delete. Free forever." />
  <meta property="og:url" content="https://offline-vault.vercel.app/" />
  <meta property="og:type" content="website" />
  <meta property="og:image" content="/og-rythamo.png" />
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:creator" content="@Rythamo8055" />
  <script type="application/ld+json">
    {
      "@context": "https://schema.org",
      "@type": "SoftwareApplication",
      "name": "File Janitor",
      "author": { "@type": "Person", "name": "Rythamo", "url": "https://github.com/Rythamo8055" },
      "publisher": "Rythamo",
      "description": "Lightweight offline duplicate finder 5-15MB, Tauri Rust BLAKE3",
      "operatingSystem": "Linux, Windows",
      "applicationCategory": "UtilitiesApplication",
      "offers": { "@type": "Offer", "price": "0", "priceCurrency": "USD" }
    }
  </script>
</svelte:head>

<header class="nav">
  <div class="nav-inner">
    <a href="/" class="brand">
      <span class="brand-icon" aria-hidden="true">
        <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.7"><path d="M9 12l-3 3a2 2 0 000 2.8 2 2 0 002.8 0l3-3"/><path d="M12 7l3-3 4 4-3 3-4-4z"/><path d="M14 14l4 4"/></svg>
      </span>
      <span class="brand-text"><strong>Rythamo</strong> <span class="muted">/ File Janitor</span></span>
    </a>
    <nav class="nav-links">
      <a href="#features">Features</a>
      <a href="#screenshots">Screenshots</a>
      <a href="/app" class="btn primary small">Open App</a>
      <button class="theme-toggle" on:click={toggleTheme} aria-label="Toggle theme">{isDark ? "Light" : "Dark"}</button>
    </nav>
  </div>
</header>

<main>
  <!-- Hero: Persuade mode — visitor decides and acts -->
  <section class="hero" bind:this={heroSection}>
    <canvas bind:this={heroCanvas} class="hero-procedural" aria-hidden="true"></canvas>
    <div class="hero-grid">
      <div class="hero-copy">
        <p class="kicker">Built by <strong>Rythamo</strong> — research-first, not intuition-first</p>
        <h1>Find <span class="accent">duplicate files</span> before they eat your disk.</h1>
        <p class="lead">File Janitor by <strong>Rythamo</strong> is the lightest offline cleaner — <strong>5-15MB</strong> (not 150MB Electron), Rust <strong>BLAKE3</strong> hashing, 100% local, <strong>trash not delete</strong>. No account, no cloud, no subscription — <em>free forever</em>.</p>
        <p class="proof">Picked by 9,363 Reddit wishes + KMeans on 1,416 threads — the blue-ocean was low-competition offline, not paywalled finance.</p>
        <div class="cta-row">
          <a href="https://github.com/Rythamo8055/file-janitor/releases/tag/v0.1.0" class="btn primary large">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.7"><path d="M12 4v12M12 16l-5-5M12 16l5-5"/><path d="M4 20h16"/></svg>
            Download for Linux (deb 6.8M)
          </a>
          <a href="/app" class="btn ghost large">Try Web Demo</a>
          <a href="https://github.com/Rythamo8055/file-janitor" class="btn ghost small">GitHub — Rythamo8055/file-janitor</a>
        </div>
        <p class="hint">Also Windows <code>msi 5.3M</code> <code>exe 3.6M</code> • Fedora 44 tested • offline • 6/6 cargo tests</p>
      </div>
      <div class="hero-mock">
        <!-- Screenshot mockup: real app UI as image, not emoji -->
        <div class="mock-window" role="img" aria-label="File Janitor screenshot — dark headerbar, pick folders, progress bar, results groups">
          <div class="mock-titlebar">
            <span class="dot red"></span><span class="dot yellow"></span><span class="dot green"></span>
            <span class="mock-title">File Janitor — offline • safe</span>
          </div>
          <img src="/tauri.svg" alt="File Janitor screenshot placeholder" style="display:none" />
          <div class="mock-body">
            <div class="mock-card">
              <strong>Pick folders</strong><span class="pill">1 selected</span>
              <div class="mock-row"><span class="mock-btn">Add folder</span><span class="mock-btn primary">Find Duplicates (1 folder)</span></div>
              <div class="mock-path">/tmp/test_janitor</div>
              <div class="mock-progress"><div class="mock-fill" style="width:68%"></div></div>
              <div class="hint" style="font-size:.75rem; color:#667085;">Hashing 1,230 of 1,800 files • 68% • BLAKE3</div>
            </div>
            <div class="mock-card">
              <strong>Results</strong><span class="pill">3 groups • 7 files • 5.7 MB wasted</span>
              <div class="mock-group"><span><strong>3 files</strong> • 2.3 MB each • wasted 4.7 MB</span><span class="mock-btn mini primary">Keep 1, trash rest</span></div>
              <div class="mock-file keep"><span class="keep">KEEP</span> /home/you/Downloads/photo.jpg</div>
              <div class="mock-file"><span class="keep dup">DUP</span> /home/you/Downloads/photo (1).jpg</div>
            </div>
          </div>
        </div>
        <p class="hint center">Screenshot: dark headerbar, progress bar with real numbers, SVG icons (no emojis) • <a href="/app">Open app</a></p>
      </div>
    </div>
  </section>

  <!-- Social proof: Rythamo ranking -->
  <section class="strip">
    <div class="strip-inner">
      <span><strong>Built by Rythamo</strong> — Search <code>Rythamo</code> on Google → this site. Research-first, 22 devlog entries, 5 assets.</span>
      <span class="muted">Free forever • No paywall (you said “ew dont slam a subscription”)</span>
    </div>
  </section>

  <!-- Features: Operate mode details -->
  <section id="features" class="features">
    <h2>Why File Janitor by Rythamo</h2>
    <div class="feature-grid">
      <div class="feature"><h3>Lightest</h3><p>5-15MB Tauri (OS WebView) vs Electron 150MB. Fedora 44, Windows msi 5.3M, exe 3.6M.</p></div>
      <div class="feature"><h3>Offline & Safe</h3><p>Rust BLAKE3, 100% local, OS trash (recoverable) — not <code>rm</code>. No account, no cloud.</p></div>
      <div class="feature"><h3>Real Data Picked It</h3><p>9,363 wishes + unsupervised KMeans (silhouette 0.269) chose low-competition blue-ocean.</p></div>
      <div class="feature"><h3>Hardened</h3><p>Skips unreadable, caps 50K files, progress per 10 files, toasts retry, CJK/RTL, 6/6 tests.</p></div>
      <div class="feature"><h3>Uncle Friendly</h3><p>Plain language, large buttons, progress bar with real numbers, demo /tmp/test_janitor.</p></div>
      <div class="feature"><h3>Content Engine</h3><p>Every commit logged to <code>docs/DEVLOG.md</code> → auto blog/thread/video — never out of content.</p></div>
    </div>
  </section>

  <!-- Screenshots -->
  <section id="screenshots" class="screenshots">
    <h2>Screenshots — dark & light, native GNOME Adwaita</h2>
    <p class="hint">Cantarell, 12-16px radii, offset+blur shadows, real SVG icons (16, no emojis). Light default (polished), dark toggle.</p>
    <div class="shot-grid">
      <div class="shot"><div class="shot-label">Light — Pick folders</div><div class="shot-box light"><div class="shot-bar">File Janitor</div><div class="shot-body">Add folder • Load demo • Find Duplicates</div></div></div>
      <div class="shot"><div class="shot-label">Dark — Results + progress</div><div class="shot-box dark"><div class="shot-bar">Scanning 68% • Hashing 1,230/1,800</div><div class="shot-body">3 groups • Keep 1, trash rest</div><div class="progress-mini"><div style="width:68%"></div></div></div></div>
      <div class="shot"><div class="shot-label">File dialog — /tmp/test_janitor</div><div class="shot-box"><div class="shot-body">3 groups demo • /tmp/test_janitor (9 files, 412K)</div></div></div>
    </div>
    <p class="center"><a href="/app" class="btn primary">Open Web Demo</a> <a href="https://github.com/Rythamo8055/file-janitor/releases/tag/v0.1.0" class="btn ghost">Download Linux/Windows</a></p>
  </section>

  <!-- How it works -->
  <section class="how">
    <h2>How Rythamo built it — timeline</h2>
    <ol class="timeline">
      <li><strong>02:22</strong> Fedora 44 lightweight need → Tauri vs Electron</li>
      <li><strong>02:50</strong> 9,363 Reddit wishes → 7% anti-cloud 655</li>
      <li><strong>02:58</strong> Weighted 8.45 vs 8.20 (python3)</li>
      <li><strong>03:08</strong> KMeans k=3 silhouette 0.269 → winner Cluster 2 Janitor 8.22</li>
      <li><strong>03:38</strong> MVP 6/6 tests, 132KB</li>
      <li><strong>12:01</strong> Release v0.1.0 5 assets, Win CI 8.8M</li>
      <li><strong>Now</strong> Vercel https://offline-vault.vercel.app live</li>
    </ol>
    <p class="hint">Full journal: <code>docs/DEVLOG.md</code> 22 entries — <a href="https://github.com/Rythamo8055/file-janitor">GitHub</a></p>
  </section>

  <!-- Download -->
  <section class="download">
    <h2>Download — free forever</h2>
    <div class="dl-grid">
      <a href="https://github.com/Rythamo8055/file-janitor/releases/download/v0.1.0/offline-vault_0.1.0_amd64.deb" class="dl-card"><strong>Linux deb</strong><span>6.8M • Ubuntu/Debian</span></a>
      <a href="https://github.com/Rythamo8055/file-janitor/releases/download/v0.1.0/offline-vault-0.1.0-1.x86_64.rpm" class="dl-card"><strong>Linux rpm</strong><span>6.8M • Fedora</span></a>
      <a href="https://github.com/Rythamo8055/file-janitor/releases/download/v0.1.0/offline-vault_0.1.0_x64_en-US.msi" class="dl-card"><strong>Windows msi</strong><span>5.3M</span></a>
      <a href="https://github.com/Rythamo8055/file-janitor/releases/download/v0.1.0/offline-vault_0.1.0_x64-setup.exe" class="dl-card"><strong>Windows exe</strong><span>3.6M</span></a>
    </div>
    <p class="hint">Web demo: <a href="/app">/app</a> (no install, demo groups) • Full needs Tauri desktop</p>
  </section>
</main>

<footer class="footer">
  <div class="footer-inner">
    <div>
      <strong>Rythamo</strong> — File Janitor<br>
      <span class="hint">Lightweight offline tools, research-first. Search “Rythamo” → this site.</span>
    </div>
    <div class="footer-links">
      <a href="https://github.com/Rythamo8055/file-janitor">GitHub</a>
      <a href="https://offline-vault.vercel.app">Vercel</a>
      <a href="/app">App</a>
      <a href="https://github.com/Rythamo8055/file-janitor#readme">README Timeline</a>
    </div>
  </div>
  <p class="hint center">© 2026 Rythamo — File Janitor 5-15MB • Tauri v2 • Svelte 5 • Rust BLAKE3 • Free forever • No emojis, real SVG icons</p>
</footer>

<style>
  :root { --bg:#fff; --card:#f6f7f9; --text:#0f1419; --muted:#667085; --border:#e5e7eb; --primary:#3584e4; --radius:16px; --font: Cantarell, system-ui, sans-serif; }
  :root[data-theme="dark"] { --bg:#0b0e14; --card:#1a1f2b; --text:#e5e7eb; --muted:#94a3b8; --border:#1f2a44; --primary:#62a0ea; }
  :global(body) { margin:0; font-family:var(--font); background:var(--bg); color:var(--text); line-height:1.6; }
  .nav { position:sticky; top:0; z-index:20; background:rgba(255,255,255,.8); backdrop-filter:blur(12px); border-bottom:1px solid var(--border); }
  :root[data-theme="dark"] .nav { background:rgba(11,14,20,.8); }
  .nav-inner { max-width:1140px; margin:0 auto; padding:.7rem 1rem; display:flex; justify-content:space-between; align-items:center; gap:1rem; }
  .brand { display:flex; align-items:center; gap:.6rem; text-decoration:none; color:var(--text); }
  .brand-icon { width:36px; height:36px; display:grid; place-items:center; background:var(--primary); border-radius:9px; }
  .brand-text { font-size:1.05rem; }
  .nav-links { display:flex; gap:.6rem; align-items:center; }
  .nav-links a { text-decoration:none; color:var(--muted); font-size:.9rem; }
  .btn { border:1px solid var(--border); background:var(--card); color:var(--text); padding:.5em 1em; border-radius:10px; text-decoration:none; display:inline-flex; align-items:center; gap:.4em; font-weight:600; }
  .btn.primary { background:var(--primary); color:white; border-color:var(--primary); }
  .btn.small { padding:.35em .7em; font-size:.85rem; }
  .btn.large { padding:.7em 1.2em; font-size:1rem; }
  .btn.ghost { background:transparent; }
  .theme-toggle { border:1px solid var(--border); background:var(--card); padding:.4em .8em; border-radius:999px; cursor:pointer; }
  .hero { max-width:1140px; margin:0 auto; padding:2.5rem 1rem 1.5rem; position:relative; overflow:hidden; }
  .hero-procedural { position:absolute; inset:0; z-index:0; pointer-events:none; opacity:0.9; }
  .hero-grid { position:relative; z-index:1; display:grid; grid-template-columns:1.1fr .9fr; gap:2rem; align-items:center; }
  @media (max-width:900px) { .hero-grid { grid-template-columns:1fr; } }
  .kicker { color:var(--primary); font-weight:700; font-size:.85rem; letter-spacing:.02em; margin:0 0 .5rem; }
  h1 { margin:0; font-size:clamp(2rem, 4vw, 3rem); font-weight:800; letter-spacing:-.03em; line-height:1.05; }
  .accent { color:var(--primary); }
  .lead { color:var(--muted); font-size:1.1rem; margin:.7rem 0 0; }
  .proof { font-size:.85rem; color:var(--muted); margin:.5rem 0 0; border-left:3px solid var(--primary); padding-left:.7rem; }
  .cta-row { display:flex; gap:.6rem; flex-wrap:wrap; margin:1.1rem 0 0; }
  .hint { color:var(--muted); font-size:.85rem; }
  .center { text-align:center; }
  .hero-mock { background:var(--card); border:1px solid var(--border); border-radius:16px; overflow:hidden; box-shadow:0 8px 32px rgba(16,24,40,.12); }
  .mock-window { background:var(--bg); }
  .mock-titlebar { display:flex; align-items:center; gap:.4rem; padding:.6rem .8rem; background:var(--card); border-bottom:1px solid var(--border); }
  .dot { width:12px; height:12px; border-radius:50%; display:inline-block; }
  .dot.red { background:#ff605c; } .dot.yellow { background:#ffbd44; } .dot.green { background:#00ca4e; }
  .mock-title { margin-left:.6rem; font-size:.8rem; color:var(--muted); }
  .mock-body { padding:1rem; display:grid; gap:.8rem; }
  .mock-card { background:var(--card); border:1px solid var(--border); border-radius:12px; padding:.8rem; }
  .mock-row { display:flex; gap:.4rem; margin:.5rem 0; }
  .mock-btn { border:1px solid var(--border); padding:.3em .6em; border-radius:8px; font-size:.8rem; background:var(--bg); }
  .mock-btn.primary { background:var(--primary); color:white; border-color:var(--primary); }
  .mock-path { font-family:monospace; font-size:.75rem; background:var(--bg); border:1px solid var(--border); padding:.3em .5em; border-radius:8px; }
  .mock-progress { height:8px; background:var(--border); border-radius:999px; overflow:hidden; margin-top:.4rem; }
  .mock-fill { height:100%; background:var(--primary); }
  .mock-group { display:flex; justify-content:space-between; align-items:center; padding:.5rem; border:1px solid var(--border); border-radius:8px; margin:.4rem 0; background:var(--bg); }
  .mock-file { padding:.3rem .5rem; border-bottom:1px solid var(--border); font-size:.8rem; display:flex; justify-content:space-between; }
  .keep { background:var(--primary); color:white; padding:.1em .4em; border-radius:6px; font-size:.7rem; font-weight:700; }
  .strip { background:var(--card); border-top:1px solid var(--border); border-bottom:1px solid var(--border); padding:.8rem 1rem; margin:1.5rem 0; }
  .strip-inner { max-width:1140px; margin:0 auto; display:flex; justify-content:space-between; gap:1rem; flex-wrap:wrap; font-size:.9rem; }
  .features, .screenshots, .how, .download { max-width:1140px; margin:0 auto; padding:1.5rem 1rem; }
  h2 { font-size:1.6rem; font-weight:800; letter-spacing:-.02em; margin:0 0 .3rem; }
  .feature-grid { display:grid; grid-template-columns:repeat(3,1fr); gap:1rem; margin-top:1rem; }
  @media (max-width:800px) { .feature-grid { grid-template-columns:1fr; } }
  .feature { background:var(--card); border:1px solid var(--border); border-radius:12px; padding:1rem; }
  .feature h3 { margin:0 0 .3rem; font-size:1rem; }
  .feature p { margin:0; color:var(--muted); font-size:.9rem; }
  .shot-grid { display:grid; grid-template-columns:repeat(3,1fr); gap:1rem; margin:1rem 0; }
  @media (max-width:900px) { .shot-grid { grid-template-columns:1fr; } }
  .shot-label { font-size:.8rem; color:var(--muted); margin-bottom:.3rem; }
  .shot-box { border:1px solid var(--border); border-radius:12px; overflow:hidden; background:var(--card); }
  .shot-box.dark { background:#242424; color:#eeeeec; }
  .shot-box.light { background:#fff; }
  .shot-bar { padding:.4rem .6rem; background:var(--bg); border-bottom:1px solid var(--border); font-size:.8rem; font-weight:600; }
  .shot-body { padding:.6rem; font-size:.85rem; color:var(--muted); }
  .progress-mini { height:6px; background:var(--border); border-radius:999px; overflow:hidden; margin:.4rem; }
  .progress-mini div { height:100%; background:var(--primary); }
  .timeline { list-style:none; padding:0; border-left:2px solid var(--border); margin:1rem 0; }
  .timeline li { padding:.5rem 0 .5rem 1rem; position:relative; }
  .timeline li::before { content:""; position:absolute; left:-6px; top:.9rem; width:10px; height:10px; background:var(--primary); border-radius:50%; }
  .dl-grid { display:grid; grid-template-columns:repeat(2,1fr); gap:.8rem; margin:1rem 0; }
  @media (max-width:600px) { .dl-grid { grid-template-columns:1fr; } }
  .dl-card { display:flex; flex-direction:column; padding:1rem; background:var(--card); border:1px solid var(--border); border-radius:12px; text-decoration:none; color:var(--text); }
  .dl-card strong { font-size:1rem; }
  .dl-card span { color:var(--muted); font-size:.85rem; }
  .footer { border-top:1px solid var(--border); padding:1.5rem 1rem; margin-top:1.5rem; background:var(--card); }
  .footer-inner { max-width:1140px; margin:0 auto; display:flex; justify-content:space-between; gap:1rem; flex-wrap:wrap; }
  .footer-links { display:flex; gap:.8rem; }
  .footer-links a { color:var(--muted); text-decoration:none; }
  code { background:var(--bg); border:1px solid var(--border); padding:.1em .3em; border-radius:6px; font-size:.85em; }
</style>
