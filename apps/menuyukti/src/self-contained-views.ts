/**
 * Serve production MCP Apps HTML from dist/self-contained/*.html (no localhost
 * asset fetches) so Codex Desktop can mount widgets.
 */
import { existsSync, readFileSync } from "node:fs";
import { createRequire } from "node:module";
import path from "node:path";
import { pathToFileURL } from "node:url";

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
  const helperPath = path.join(
    path.dirname(serverEntry),
    "templateHelper.js",
  );
  return pathToFileURL(helperPath).href;
}

/** Map Skybridge viewFile (e.g. assets/venue-slot-strength-xxxx.js) → view name. */
export function viewNameFromFile(viewFile: string): string | null {
  const base = path.basename(viewFile);
  const match = /^([a-z0-9-]+)-[A-Za-z0-9_-]+\.js$/i.exec(base);
  return match?.[1] ?? null;
}

export function selfContainedHtmlPath(
  viewName: string,
  cwd = process.cwd(),
): string {
  return path.join(cwd, "dist", "self-contained", `${viewName}.html`);
}

export function readSelfContainedHtml(
  viewName: string,
  cwd = process.cwd(),
): string | null {
  const file = selfContainedHtmlPath(viewName, cwd);
  if (!existsSync(file)) return null;
  return readFileSync(file, "utf8");
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
      const html = readSelfContainedHtml(name);
      if (html) return html;
      console.warn(
        `[menuyukti] missing self-contained widget for "${name}" (expected ${selfContainedHtmlPath(name)}). Falling back to Skybridge asset shell — Codex Desktop may time out. Run: pnpm --filter menuyukti build`,
      );
    }
    return original(data);
  };
}
