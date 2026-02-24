export function publicCacheHeaders(sMaxAgeSeconds: number, staleWhileRevalidateSeconds = 0): Record<string, string> {
  const sMaxAge = Math.max(0, Math.floor(sMaxAgeSeconds));
  const stale = Math.max(0, Math.floor(staleWhileRevalidateSeconds));
  return {
    "Cache-Control": `public, s-maxage=${sMaxAge}, stale-while-revalidate=${stale}`,
  };
}
