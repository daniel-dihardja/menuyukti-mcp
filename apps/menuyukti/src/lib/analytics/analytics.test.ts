import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { loadWorkspaceCogs } from "../cogs.js";
import { loadNormalizedSales } from "../load.js";
import {
  adaptWeakReviewFromOrders,
  applyMatrixCategories,
  calculateMenuHeatmaps,
  computeMenuBasketAffinitiesFromOrders,
  computeMenuEngineeringFromOrders,
  computeSlotDemandProfileFromOrders,
  enrichedMenuHeatmaps,
  optionalStarFocus,
} from "./index.js";

const workspaces = resolve(
  import.meta.dirname,
  "../../../../../workspaces",
);

describe("demo-cafe-july-2026 analytics", () => {
  it("get_menu_heatmaps: 98 menus; COGS enriches matrix_category", () => {
    const sales = loadNormalizedSales("demo-cafe-july-2026", workspaces);
    const heatmaps = calculateMenuHeatmaps(sales.rows);
    expect(heatmaps).toHaveLength(98);
    expect(heatmaps[0]?.daily_heatmap).toHaveLength(24);
    expect(heatmaps[0]?.weekly_heatmap).toHaveLength(7);
    expect(heatmaps.every((h) => h.matrix_category === null)).toBe(true);

    const viaFile = calculateMenuHeatmaps(
      loadNormalizedSales("demo-cafe-july-2026/sales.xlsx", workspaces).rows,
    );
    expect(viaFile).toHaveLength(98);
    expect(viaFile.every((h) => h.matrix_category === null)).toBe(true);

    const enriched = enrichedMenuHeatmaps(
      "demo-cafe-july-2026",
      sales.rows,
      workspaces,
    );
    const cogs = loadWorkspaceCogs("demo-cafe-july-2026", workspaces);
    const matrix = computeMenuEngineeringFromOrders(sales.rows, cogs.unit_costs);
    const expected = new Map(
      matrix.items.map((item) => [item.menu, item.category]),
    );
    let matched = 0;
    for (const item of enriched) {
      if (!item.matrix_category) continue;
      matched += 1;
      expect(expected.get(item.menu)).toBe(item.matrix_category);
      expect(["star", "plow_horse", "puzzle", "weak"]).toContain(
        item.matrix_category,
      );
    }
    expect(matched).toBe(expected.size);
    expect(matched).toBeGreaterThan(0);

    const copy = calculateMenuHeatmaps(sales.rows);
    applyMatrixCategories(copy, expected);
    expect(
      copy.filter((h) => h.matrix_category).map((h) => h.menu).sort(),
    ).toEqual(
      enriched.filter((h) => h.matrix_category).map((h) => h.menu).sort(),
    );
  });

  it("get_venue_slot_strength: 35 day×meal cells", () => {
    const sales = loadNormalizedSales("demo-cafe-july-2026", workspaces);
    const profile = computeSlotDemandProfileFromOrders(sales.rows);
    expect(profile).toHaveLength(35);
    expect(profile.some((c) => c.order_count > 0)).toBe(true);
    const relatives = new Set(profile.map((c) => c.relative_demand));
    expect(
      ["low", "high", "average"].some((label) => relatives.has(label)),
    ).toBe(true);
  });

  it("analyze_menu_engineering: all four quadrants on demo cafe", () => {
    const sales = loadNormalizedSales("demo-cafe-july-2026", workspaces);
    const cogs = loadWorkspaceCogs("demo-cafe-july-2026", workspaces);
    const matrix = computeMenuEngineeringFromOrders(sales.rows, cogs.unit_costs);
    expect(matrix.items.length).toBeGreaterThan(0);
    expect(matrix.coverage.matched_menus).toBeGreaterThan(0);
    const cats = new Set(matrix.items.map((i) => i.category));
    for (const required of ["star", "plow_horse", "puzzle", "weak"]) {
      expect(cats.has(required)).toBe(true);
    }
  });

  it("get_weak_items_review: weak quadrant + period depth", () => {
    const sales = loadNormalizedSales("demo-cafe-july-2026", workspaces);
    const cogs = loadWorkspaceCogs("demo-cafe-july-2026", workspaces);
    const matrix = computeMenuEngineeringFromOrders(sales.rows, cogs.unit_costs);
    const review = adaptWeakReviewFromOrders(matrix, sales.rows);
    expect(review.rows.length).toBeGreaterThan(0);
    expect(review.stats.item_count).toBe(review.rows.length);
    expect(review.stats.period_days).toBe(31);
    expect(review.period_start).toBe("2026-07-01");
    expect(review.period_end).toBe("2026-07-31");
    expect(review.distribution_stats.item_count).toBeGreaterThan(0);
  });

  it("get_pair_lift_matrix: sales-only top_by_presence; COGS uses stars", () => {
    const sales = loadNormalizedSales("demo-cafe-july-2026", workspaces);
    const withoutFocus = computeMenuBasketAffinitiesFromOrders(sales.rows);
    expect(withoutFocus.focus_menus.length).toBeGreaterThanOrEqual(2);
    expect(withoutFocus.scope).toBe("top_by_presence");
    expect(withoutFocus.matrix_lift).toHaveLength(withoutFocus.focus_menus.length);
    expect(withoutFocus.matrix_lift[0]?.[0]).toBeNull();

    const cogs = loadWorkspaceCogs("demo-cafe-july-2026", workspaces);
    const matrix = computeMenuEngineeringFromOrders(sales.rows, cogs.unit_costs);
    const stars = matrix.items
      .filter((i) => i.category === "star")
      .map((i) => i.menu);
    expect(stars.length).toBeGreaterThanOrEqual(2);

    const withStars = computeMenuBasketAffinitiesFromOrders(sales.rows, stars);
    expect(withStars.scope).toBe("stars");
    expect(withStars.focus_menus).toEqual(stars.slice(0, 12));

    const viaEnrichment = optionalStarFocus(
      "demo-cafe-july-2026",
      sales.rows,
      workspaces,
    );
    expect(viaEnrichment?.length).toBeGreaterThanOrEqual(2);
    const enrichedLift = computeMenuBasketAffinitiesFromOrders(
      sales.rows,
      viaEnrichment,
    );
    expect(enrichedLift.scope).toBe("stars");
  });
});
