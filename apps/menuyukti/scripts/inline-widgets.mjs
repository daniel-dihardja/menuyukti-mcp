#!/usr/bin/env node
/**
 * After `skybridge build`, emit self-contained HTML per view under
 * dist/self-contained/<name>.html for Codex Desktop MCP Apps.
 *
 * Skybridge's default production shell does `import(serverUrl + "/assets/…")`,
 * which Codex CSP blocks for http://127.0.0.1. Rust widgets on main were one
 * HTML blob; this restores that contract.
 */
import * as esbuild from "esbuild";
import {
  existsSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const assetsRoot = path.join(appRoot, "dist", "assets");
const manifestPath = path.join(assetsRoot, ".vite", "manifest.json");
const outDir = path.join(appRoot, "dist", "self-contained");

function assertManifest() {
  if (!existsSync(manifestPath)) {
    throw new Error(
      `Missing ${manifestPath}. Run skybridge build before inline-widgets.`,
    );
  }
}

function loadCss(manifest) {
  const styleEntry = manifest["style.css"];
  if (!styleEntry?.file) {
    throw new Error("Vite manifest missing style.css entry");
  }
  const cssPath = path.join(assetsRoot, styleEntry.file);
  return readFileSync(cssPath, "utf8");
}

function entryViews(manifest) {
  /** @type {{ name: string, file: string }[]} */
  const views = [];
  for (const entry of Object.values(manifest)) {
    if (entry?.isEntry && entry.name && entry.file) {
      views.push({ name: entry.name, file: entry.file });
    }
  }
  views.sort((a, b) => a.name.localeCompare(b.name));
  return views;
}

function wrapHtml(css, js) {
  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<style>${css}</style>
</head>
<body>
<div id="root"></div>
<script type="module">
window.skybridge = window.skybridge || { serverUrl: "" };
${js}
</script>
</body>
</html>
`;
}

export async function inlineWidgets({ root = appRoot } = {}) {
  const localAssets = path.join(root, "dist", "assets");
  const localManifest = path.join(localAssets, ".vite", "manifest.json");
  if (!existsSync(localManifest)) {
    throw new Error(
      `Missing ${localManifest}. Run skybridge build before inline-widgets.`,
    );
  }
  const manifest = JSON.parse(readFileSync(localManifest, "utf8"));
  const css = (() => {
    const styleEntry = manifest["style.css"];
    if (!styleEntry?.file) throw new Error("Vite manifest missing style.css");
    return readFileSync(path.join(localAssets, styleEntry.file), "utf8");
  })();
  const views = entryViews(manifest);
  if (views.length === 0) {
    throw new Error("No Vite view entries found to inline");
  }

  const out = path.join(root, "dist", "self-contained");
  rmSync(out, { recursive: true, force: true });
  mkdirSync(out, { recursive: true });

  const written = [];
  for (const view of views) {
    const entryPath = path.join(localAssets, view.file);
    const bundled = await esbuild.build({
      entryPoints: [entryPath],
      bundle: true,
      write: false,
      format: "esm",
      platform: "browser",
      logLevel: "warning",
    });
    const js = bundled.outputFiles[0]?.text;
    if (!js) throw new Error(`esbuild produced no output for ${view.name}`);
    const html = wrapHtml(css, js);
    if (/https?:\/\/(?:127\.0\.0\.1|localhost)(?::\d+)?\/(?:assets|@vite|_skybridge)/i.test(html)) {
      throw new Error(
        `Self-contained HTML for ${view.name} still references localhost Vite/assets URLs`,
      );
    }
    const htmlPath = path.join(out, `${view.name}.html`);
    writeFileSync(htmlPath, html);
    written.push({ name: view.name, bytes: html.length, path: htmlPath });
  }
  return written;
}

async function main() {
  assertManifest();
  const written = await inlineWidgets();
  for (const w of written) {
    console.log(`inlined ${w.name} (${w.bytes} bytes) → ${path.relative(appRoot, w.path)}`);
  }
  console.log(`Self-contained widgets: ${written.length} under dist/self-contained/`);
}

const isMain =
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isMain) {
  main().catch((err) => {
    console.error(err);
    process.exit(1);
  });
}
