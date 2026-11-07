import { loadWorkspaceCogs } from "../cogs.js";
import type { LineItem } from "../line-item.js";
import {
  applyMatrixCategories,
  calculateMenuHeatmaps,
  type MenuHeatmapPayload,
} from "./heatmaps.js";
import { computeMenuEngineeringFromOrders } from "./menu-engineering.js";

/** Menu → matrix category when workspace COGS can classify the menu. */
export function optionalMatrixCategories(
  path: string,
  rows: LineItem[],
  dataDir?: string,
): Map<string, string> | null {
  try {
    const cogs = loadWorkspaceCogs(path, dataDir);
    const matrix = computeMenuEngineeringFromOrders(rows, cogs.unit_costs);
    return new Map(matrix.items.map((item) => [item.menu, item.category]));
  } catch {
    return null;
  }
}

/** Prefer menu-engineering stars as focus when workspace COGS is available. */
export function optionalStarFocus(
  path: string,
  rows: LineItem[],
  dataDir?: string,
): string[] | undefined {
  const categories = optionalMatrixCategories(path, rows, dataDir);
  if (!categories) return undefined;
  const stars = [...categories.entries()]
    .filter(([, category]) => category === "star")
    .map(([menu]) => menu);
  return stars.length >= 2 ? stars : undefined;
}

export function enrichedMenuHeatmaps(
  path: string,
  rows: LineItem[],
  dataDir?: string,
): MenuHeatmapPayload[] {
  const heatmaps = calculateMenuHeatmaps(rows);
  const categories = optionalMatrixCategories(path, rows, dataDir);
  if (categories) applyMatrixCategories(heatmaps, categories);
  return heatmaps;
}
