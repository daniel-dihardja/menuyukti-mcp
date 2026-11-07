import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { afterEach, describe, expect, it } from "vitest";
import { menuyuktiPackageRoot } from "./self-contained-views.js";

const scriptsDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  "../scripts",
);

async function loadCsp() {
  return import(
    pathToFileURL(path.join(scriptsDir, "self-contained-csp.mjs")).href
  ) as Promise<{
    SELF_CONTAINED_VIEW_NAMES: readonly string[];
    assertSelfContainedDist: (packageRoot: string) => void;
    assertSelfContainedHtml: (viewName: string, html: string) => void;
    hasLocalhostAssetUrls: (html: string) => boolean;
  }>;
}

async function loadInliner() {
  return import(
    pathToFileURL(path.join(scriptsDir, "inline-widgets.mjs")).href
  ) as Promise<{
    inlineWidgets: (opts?: {
      root?: string;
    }) => Promise<{ name: string; bytes: number; path: string }[]>;
  }>;
}

describe("self-contained CSP helpers", () => {
  const temps: string[] = [];

  afterEach(() => {
    for (const dir of temps.splice(0)) {
      rmSync(dir, { recursive: true, force: true });
    }
  });

  it("lists the same stems as src/views/*.tsx (except shared)", async () => {
    const { SELF_CONTAINED_VIEW_NAMES } = await loadCsp();
    const viewsDir = path.join(menuyuktiPackageRoot(), "src", "views");
    const fromDisk = readdirSync(viewsDir)
      .filter(
        (f) =>
          f.endsWith(".tsx") && !f.endsWith(".test.tsx") && f !== "shared.tsx",
      )
      .map((f) => f.replace(/\.tsx$/, ""))
      .sort();
    expect([...SELF_CONTAINED_VIEW_NAMES].sort()).toEqual(fromDisk);
  });

  it("detects blocked localhost Vite/asset URLs", async () => {
    const { hasLocalhostAssetUrls, assertSelfContainedHtml } = await loadCsp();
    expect(hasLocalhostAssetUrls("<html>clean</html>")).toBe(false);
    expect(
      hasLocalhostAssetUrls(
        `import('http://127.0.0.1:8787/assets/venue-slot-strength-abc.js')`,
      ),
    ).toBe(true);
    expect(
      hasLocalhostAssetUrls(
        `href="http://localhost:8787/_skybridge/view/hello-world"`,
      ),
    ).toBe(true);
    expect(() =>
      assertSelfContainedHtml(
        "hello-world",
        `import('http://127.0.0.1:8787/assets/x.js')`,
      ),
    ).toThrow(/localhost Vite\/assets/);
  });

  it("inlines a fake Vite dist without localhost asset URLs", async () => {
    const { assertSelfContainedHtml } = await loadCsp();
    const { inlineWidgets } = await loadInliner();
    const root = mkdtempSync(path.join(tmpdir(), "menuyukti-inline-"));
    temps.push(root);

    const assets = path.join(root, "dist", "assets");
    mkdirSync(path.join(assets, ".vite"), { recursive: true });
    writeFileSync(path.join(assets, "style-test.css"), "body{margin:0}");
    writeFileSync(
      path.join(assets, "hello-world-TESTHASH.js"),
      "export default function Hello(){ return null }",
    );
    writeFileSync(
      path.join(assets, ".vite", "manifest.json"),
      JSON.stringify({
        "style.css": { file: "style-test.css" },
        "src/views/hello-world.tsx": {
          file: "hello-world-TESTHASH.js",
          name: "hello-world",
          isEntry: true,
        },
      }),
    );

    const written = await inlineWidgets({ root });
    expect(written.map((w) => w.name)).toEqual(["hello-world"]);
    const html = readFileSync(written[0]!.path, "utf8");
    expect(html).toContain("<!DOCTYPE html>");
    expect(html).not.toMatch(/127\.0\.0\.1|localhost/);
    assertSelfContainedHtml("hello-world", html);
  });

  it("fails assertSelfContainedDist when a registered view is missing", async () => {
    const { assertSelfContainedDist } = await loadCsp();
    const root = mkdtempSync(path.join(tmpdir(), "menuyukti-missing-views-"));
    temps.push(root);
    mkdirSync(path.join(root, "dist", "self-contained"), { recursive: true });
    expect(() => assertSelfContainedDist(root)).toThrow(
      /Missing self-contained widget HTML/,
    );
  });

  it("validates real dist/self-contained when present after build", async () => {
    const { SELF_CONTAINED_VIEW_NAMES, assertSelfContainedDist } =
      await loadCsp();
    const root = menuyuktiPackageRoot();
    const dir = path.join(root, "dist", "self-contained");
    // Skip when unset or empty (e.g. wiped dist, or CI without skybridge build).
    // Fixture tests above always cover CSP; this path runs after a real build.
    if (
      !existsSync(dir) ||
      !SELF_CONTAINED_VIEW_NAMES.some((name) =>
        existsSync(path.join(dir, `${name}.html`)),
      )
    ) {
      return;
    }
    assertSelfContainedDist(root);
    for (const name of SELF_CONTAINED_VIEW_NAMES) {
      expect(existsSync(path.join(dir, `${name}.html`))).toBe(true);
    }
  });
});
