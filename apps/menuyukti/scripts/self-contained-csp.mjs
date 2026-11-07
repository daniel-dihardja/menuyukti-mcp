/**
 * Shared CSP / self-contained widget helpers for the post-build inliner and tests.
 */
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

/** View stems that must ship as dist/self-contained/<name>.html for Codex Desktop. */
export const SELF_CONTAINED_VIEW_NAMES = Object.freeze([
  "hello-world",
  "menu-heatmaps",
  "pair-lift-matrix",
  "venue-slot-strength",
  "weak-items-review",
]);

/** Matches Skybridge/Vite asset URLs that Codex Desktop CSP blocks. */
export const LOCALHOST_ASSET_URL_RE =
  /https?:\/\/(?:127\.0\.0\.1|localhost)(?::\d+)?\/(?:assets|@vite|_skybridge)/i;

export function hasLocalhostAssetUrls(html) {
  return LOCALHOST_ASSET_URL_RE.test(html);
}

export function assertSelfContainedHtml(viewName, html) {
  if (hasLocalhostAssetUrls(html)) {
    throw new Error(
      `Self-contained HTML for ${viewName} still references localhost Vite/assets URLs`,
    );
  }
}

export function selfContainedHtmlFile(packageRoot, viewName) {
  return path.join(packageRoot, "dist", "self-contained", `${viewName}.html`);
}

/**
 * Fail if any registered view is missing from dist/self-contained or contains
 * blocked localhost asset URLs.
 */
export function assertSelfContainedDist(packageRoot) {
  const missing = [];
  for (const name of SELF_CONTAINED_VIEW_NAMES) {
    const file = selfContainedHtmlFile(packageRoot, name);
    if (!existsSync(file)) {
      missing.push(name);
      continue;
    }
    assertSelfContainedHtml(name, readFileSync(file, "utf8"));
  }
  if (missing.length > 0) {
    throw new Error(
      `Missing self-contained widget HTML for: ${missing.join(", ")}. ` +
        `Run: pnpm --filter menuyukti build`,
    );
  }
}
