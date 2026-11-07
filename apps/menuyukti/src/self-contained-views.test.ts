import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import {
  menuyuktiPackageRoot,
  missingSelfContainedMessage,
  requireSelfContainedHtml,
  selfContainedHtmlPath,
  viewNameFromFile,
} from "./self-contained-views.js";

describe("viewNameFromFile", () => {
  it("parses hashed Vite asset filenames", () => {
    expect(viewNameFromFile("assets/venue-slot-strength-nXJ0Ondf.js")).toBe(
      "venue-slot-strength",
    );
    expect(viewNameFromFile("assets/hello-world-BOn4Um8t.js")).toBe(
      "hello-world",
    );
    expect(viewNameFromFile("assets/weak-items-review-Bg1e2VWx.js")).toBe(
      "weak-items-review",
    );
  });

  it("returns null for unexpected paths", () => {
    expect(viewNameFromFile("assets/style-DU-lG4zV.css")).toBeNull();
    expect(viewNameFromFile("")).toBeNull();
  });
});

describe("menuyuktiPackageRoot", () => {
  it("resolves the menuyukti package from this module URL", () => {
    const root = menuyuktiPackageRoot();
    expect(path.basename(root)).toBe("menuyukti");
    expect(selfContainedHtmlPath("hello-world")).toBe(
      path.join(root, "dist", "self-contained", "hello-world.html"),
    );
  });
});

describe("requireSelfContainedHtml", () => {
  const temps: string[] = [];
  const probeName = `__cwd-probe-${process.pid}__`;

  afterEach(() => {
    for (const dir of temps.splice(0)) {
      rmSync(dir, { recursive: true, force: true });
    }
    rmSync(selfContainedHtmlPath(probeName), { force: true });
  });

  it("returns HTML when the self-contained file exists under an explicit root", () => {
    const root = mkdtempSync(path.join(tmpdir(), "menuyukti-sc-"));
    temps.push(root);
    const out = path.join(root, "dist", "self-contained");
    mkdirSync(out, { recursive: true });
    writeFileSync(path.join(out, "venue-slot-strength.html"), "<html>ok</html>");

    expect(requireSelfContainedHtml("venue-slot-strength", root)).toBe(
      "<html>ok</html>",
    );
  });

  it("throws an actionable error when the file is missing", () => {
    const root = mkdtempSync(path.join(tmpdir(), "menuyukti-sc-missing-"));
    temps.push(root);
    const expected = selfContainedHtmlPath("menu-heatmaps", root);

    expect(() => requireSelfContainedHtml("menu-heatmaps", root)).toThrow(
      missingSelfContainedMessage("menu-heatmaps", expected),
    );
    expect(() => requireSelfContainedHtml("menu-heatmaps", root)).toThrow(
      /pnpm --filter menuyukti build/,
    );
    expect(() => requireSelfContainedHtml("menu-heatmaps", root)).toThrow(
      /inline-widgets/,
    );
  });

  it("finds HTML via package root when cwd is the package directory", () => {
    const packageRoot = menuyuktiPackageRoot();
    const out = path.join(packageRoot, "dist", "self-contained");
    mkdirSync(out, { recursive: true });
    writeFileSync(path.join(out, `${probeName}.html`), "<html>pkg-cwd</html>");

    const prev = process.cwd();
    try {
      process.chdir(packageRoot);
      expect(requireSelfContainedHtml(probeName)).toBe("<html>pkg-cwd</html>");
    } finally {
      process.chdir(prev);
    }
  });

  it("finds HTML via package root when cwd is the repo root", () => {
    const packageRoot = menuyuktiPackageRoot();
    const repoRoot = path.resolve(packageRoot, "../..");
    const out = path.join(packageRoot, "dist", "self-contained");
    mkdirSync(out, { recursive: true });
    writeFileSync(path.join(out, `${probeName}.html`), "<html>repo-cwd</html>");

    const prev = process.cwd();
    try {
      process.chdir(repoRoot);
      expect(requireSelfContainedHtml(probeName)).toBe("<html>repo-cwd</html>");
      expect(selfContainedHtmlPath(probeName)).toBe(
        path.join(out, `${probeName}.html`),
      );
    } finally {
      process.chdir(prev);
    }
  });
});
