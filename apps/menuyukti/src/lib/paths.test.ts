import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import {
  DATA_DIR_ENV,
  listWorkspaceFiles,
  listWorkspaces,
  ResolvePathError,
  resolveAllowedDir,
  resolveAllowedPath,
} from "./paths.js";
import { getSetupStatus } from "./setup.js";

const originalDataDir = process.env[DATA_DIR_ENV];

afterEach(() => {
  if (originalDataDir === undefined) delete process.env[DATA_DIR_ENV];
  else process.env[DATA_DIR_ENV] = originalDataDir;
});

function tempDataDir(): string {
  return mkdtempSync(join(tmpdir(), "menuyukti-paths-"));
}

describe("path gating", () => {
  it("rejects traversal outside the data dir", () => {
    const root = tempDataDir();
    writeFileSync(join(root, "inside.xlsx"), "x");
    expect(() => resolveAllowedPath("../README.md", root)).toThrow(
      ResolvePathError,
    );
  });

  it("rejects absolute paths outside the data dir", () => {
    const root = tempDataDir();
    const outside = tempDataDir();
    const secret = join(outside, "secret.xlsx");
    writeFileSync(secret, "x");
    expect(() => resolveAllowedPath(secret, root)).toThrow(ResolvePathError);
  });

  it("lists workspaces grouped by folder", () => {
    const root = tempDataDir();
    mkdirSync(join(root, "nested"));
    writeFileSync(join(root, "a.xlsx"), "a");
    writeFileSync(join(root, "nested", "b.xls"), "b");
    writeFileSync(join(root, "skip.txt"), "no");
    writeFileSync(join(root, "~$lock.xlsx"), "lock");

    const listed = listWorkspaces(root);
    expect(listed.workspace_count).toBe(2);
    expect(listed.workspaces[0]?.path).toBe(".");
    expect(listed.workspaces[0]?.files[0]?.path).toBe("a.xlsx");
    expect(listed.workspaces[1]?.path).toBe("nested");
    expect(listed.workspaces[1]?.files[0]?.path).toBe("nested/b.xls");
  });

  it("lists all files including non-excel and rejects traversal", () => {
    const root = tempDataDir();
    const cafe = join(root, "cafe");
    mkdirSync(join(cafe, "notes"), { recursive: true });
    writeFileSync(join(cafe, "sales.xlsx"), "a");
    writeFileSync(join(cafe, "workspace.yaml"), "version: 1\n");
    writeFileSync(join(cafe, "notes", "readme.md"), "hi");
    writeFileSync(join(cafe, "~$lock.xlsx"), "lock");

    const listed = listWorkspaceFiles("cafe", root);
    expect(listed.file_count).toBe(3);
    expect(listed.files.map((f) => f.path)).toEqual([
      "cafe/notes/readme.md",
      "cafe/sales.xlsx",
      "cafe/workspace.yaml",
    ]);

    expect(() => listWorkspaceFiles("../etc", root)).toThrow(ResolvePathError);
  });

  it("setup status sees demo workspace when present", () => {
    const root = tempDataDir();
    const demo = join(root, "demo-cafe-july-2026");
    mkdirSync(demo);
    writeFileSync(join(demo, "workspace.yaml"), "version: 1\n");
    writeFileSync(join(demo, "sales.xlsx"), "x");
    mkdirSync(join(root, "needs-setup"));
    writeFileSync(join(root, "needs-setup", "sales.xlsx"), "y");

    const status = getSetupStatus(root);
    expect(status.workspace_count).toBe(2);
    expect(status.demo_workspace_present).toBe(true);
    expect(status.folders_missing_workspace_yaml).toEqual(["needs-setup"]);
  });

  it("resolveAllowedDir accepts demo folder name", () => {
    const root = tempDataDir();
    mkdirSync(join(root, "demo-cafe-july-2026"));
    const dir = resolveAllowedDir("demo-cafe-july-2026", root);
    expect(dir.endsWith("demo-cafe-july-2026")).toBe(true);
  });
});
