/**
 * Serve production MCP Apps HTML from dist/self-contained/*.html (no localhost
 * asset fetches) so Codex Desktop can mount widgets.
 */
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

type RenderProductionData = {
  serverUrl: string;
  viewFile: string;
  styleFile: string;
};

type TemplateHelper = {
  renderProduction: (data: RenderProductionData) => string;
  renderDevelopment: (data: { serverUrl: string; viewName: string }) => string;
};

const require = createRequire(import.meta.url);

function skybridgeTemplateHelperUrl(): string {
  // skybridge package exports omit package.json; resolve via ./server entry.
  const serverEntry = require.resolve("skybridge/server");
  const helperPath = path.join(path.dirname(serverEntry), "templateHelper.js");
  return pathToFileURL(helperPath).href;
}

/**
 * Absolute path to the apps/menuyukti package root (directory with package.json
 * name "menuyukti"). Anchored to this module so cwd does not matter.
 */
export function menuyuktiPackageRoot(
  fromUrl: string | URL = import.meta.url,
): string {
  let dir = path.dirname(fileURLToPath(fromUrl));
  for (let i = 0; i < 8; i++) {
    const pkgPath = path.join(dir, "package.json");
    if (existsSync(pkgPath)) {
      try {
        const name = JSON.parse(readFileSync(pkgPath, "utf8")).name as unknown;
        if (name === "menuyukti") return dir;
      } catch {
        // keep walking
      }
    }
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  // src/ or dist/ → package root
  return path.resolve(path.dirname(fileURLToPath(fromUrl)), "..");
}

/** Map Skybridge viewFile (e.g. assets/venue-slot-strength-xxxx.js) → view name. */
export function viewNameFromFile(viewFile: string): string | null {
  const base = path.basename(viewFile);
  const match = /^([a-z0-9-]+)-[A-Za-z0-9_-]+\.js$/i.exec(base);
  return match?.[1] ?? null;
}

export function selfContainedHtmlPath(
  viewName: string,
  packageRoot = menuyuktiPackageRoot(),
): string {
  return path.join(packageRoot, "dist", "self-contained", `${viewName}.html`);
}

export function readSelfContainedHtml(
  viewName: string,
  packageRoot = menuyuktiPackageRoot(),
): string | null {
  const file = selfContainedHtmlPath(viewName, packageRoot);
  if (!existsSync(file)) return null;
  return readFileSync(file, "utf8");
}

export function missingSelfContainedMessage(
  viewName: string,
  expectedPath: string,
): string {
  return (
    `[menuyukti] missing self-contained widget for "${viewName}" ` +
    `(expected ${expectedPath}). ` +
    "Run: pnpm --filter menuyukti build (includes inline-widgets) " +
    "or pnpm mcp:prod. " +
    "Codex Desktop cannot use the default Skybridge asset shell."
  );
}

/**
 * Return inlined production HTML for a view, or throw so resources/read fails
 * loudly instead of serving a CSP-broken Skybridge asset shell.
 */
export function requireSelfContainedHtml(
  viewName: string,
  packageRoot = menuyuktiPackageRoot(),
): string {
  const html = readSelfContainedHtml(viewName, packageRoot);
  if (html) return html;
  throw new Error(
    missingSelfContainedMessage(
      viewName,
      selfContainedHtmlPath(viewName, packageRoot),
    ),
  );
}

let patched = false;

/** Patch Skybridge production HTML renderer once (no-op if already patched). */
export async function enableSelfContainedProductionViews(): Promise<void> {
  if (patched) return;
  patched = true;

  const mod = (await import(skybridgeTemplateHelperUrl())) as {
    templateHelper: TemplateHelper;
  };
  const helper = mod.templateHelper;
  const original = helper.renderProduction.bind(helper);

  helper.renderProduction = (data: RenderProductionData) => {
    const name = viewNameFromFile(data.viewFile);
    if (name) {
      // Skybridge only calls renderProduction when NODE_ENV=production.
      // Never fall back to the localhost asset shell — Codex CSP times out.
      return requireSelfContainedHtml(name);
    }
    return original(data);
  };
}
