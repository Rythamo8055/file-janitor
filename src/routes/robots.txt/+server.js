export const prerender = true;
export async function GET() {
  const site = "https://offline-vault.vercel.app";
  const body = `User-agent: *
Allow: /
Sitemap: ${site}/sitemap.xml
`;
  return new Response(body, { headers: { "Content-Type": "text/plain" } });
}
