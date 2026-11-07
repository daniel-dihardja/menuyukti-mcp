import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { loadWorkspaceCogs } from "./cogs.js";
import { loadNormalizedSales } from "./load.js";
import { listNormalizedRows, summarizeSales } from "./summary.js";

const workspaces = resolve(import.meta.dirname, "../../../../workspaces");

describe("demo-cafe-july-2026 sales load", () => {
  it("matches Rust summarize baselines via workspace path", () => {
    const loaded = loadNormalizedSales("demo-cafe-july-2026", workspaces);
    expect(loaded.pos).toBe("esb");

    const summary = summarizeSales(loaded.pos, loaded.rows);
    expect(summary.total_items_sold).toBe(1748);
    expect(Math.abs(summary.total_revenue - 29_795_300)).toBeLessThan(0.01);
    expect(summary.period_start).toBe("2026-07-01");
    expect(summary.period_end).toBe("2026-07-31");
    expect(summary.top_items[0]?.menu).toBe("Iced Milk Coffee Aren");
    expect(summary.top_items[0]?.quantity).toBe(167);

    const listed = listNormalizedRows(loaded.pos, loaded.rows);
    expect(listed.row_count).toBe(1441);

    const viaFile = loadNormalizedSales(
      "demo-cafe-july-2026/sales.xlsx",
      workspaces,
    );
    expect(viaFile.rows.length).toBe(loaded.rows.length);
  });

  it("loads demo COGS unit costs", () => {
    const cogs = loadWorkspaceCogs("demo-cafe-july-2026", workspaces);
    expect(cogs.path).toBe("demo-cafe-july-2026/cogs.xlsx");
    expect(cogs.unit_costs["Avocado Chocolate"]).toBe(8000);
    expect(Object.keys(cogs.unit_costs).length).toBeGreaterThan(0);
  });
});
