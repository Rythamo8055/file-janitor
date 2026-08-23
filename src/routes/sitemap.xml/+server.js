export const prerender = true;
export async function GET() {
  const site = "https://offline-vault.vercel.app";
  const now = new Date().toISOString();
  const urls = [
    { loc: `${site}/`, lastmod: now, changefreq: "daily", priority: "1.0" },
    { loc: `${site}/app`, lastmod: now, changefreq: "weekly", priority: "0.8" }
  ];
  const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.map(u => `  <url><loc>${u.loc}</loc><lastmod>${u.lastmod}</lastmod><changefreq>${u.changefreq}</changefreq><priority>${u.priority}</priority></url>`).join("\n")}
</urlset>`;
  return new Response(body, { headers: { "Content-Type": "application/xml", "Cache-Control": "max-age=0, s-maxage=3600" } });
}
