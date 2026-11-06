import type { LineItem } from "../line-item.js";

export type MenuEngineeringThresholds = {
  avg_popularity: number;
  avg_contribution_margin: number;
  total_cogs: number;
  total_profit: number;
  total_margin: number;
};

export type MenuEngineeringDistributionItem = {
  category: string;
  item_count: number;
  item_share: number;
  margin_share: number;
};

export type MenuEngineeringMatrixItem = {
  menu: string;
  quantity: number;
  total_revenue: number;
  cogs: number;
  total_cogs: number;
  contribution_margin: number;
  contribution_margin_percentage: number;
  margin_per_unit: number;
  we_value: number;
  category: string;
  action: string;
  menu_category: string | null;
  menu_category_detail: string | null;
};

export type MenuEngineeringCoverage = {
  matched_menus: number;
  unmatched_menus: string[];
};

export type MenuEngineeringMatrixResult = {
  thresholds: MenuEngineeringThresholds;
  distribution: MenuEngineeringDistributionItem[];
  items: MenuEngineeringMatrixItem[];
  coverage: MenuEngineeringCoverage;
};

type AggRow = {
  menu: string;
  quantity: number;
  total_revenue: number;
  cogs: number;
  menu_category: string | null;
  menu_category_detail: string | null;
};

export function computeMenuEngineeringFromOrders(
  rows: LineItem[],
  cogsByMenu: Record<string, number>,
): MenuEngineeringMatrixResult {
  if (rows.length === 0) {
    throw new Error("Sales rows are empty. Cannot calculate menu engineering.");
  }
  if (Object.keys(cogsByMenu).length === 0) {
    throw new Error(
      "COGS map is empty. Add Menu/COGS unit costs before running menu engineering.",
    );
  }

  const byMenu = new Map<string, AggRow>();
  for (const row of rows) {
    const entry = byMenu.get(row.menu) ?? {
      menu: row.menu,
      quantity: 0,
      total_revenue: 0,
      cogs: 0,
      menu_category: null as string | null,
      menu_category_detail: null as string | null,
    };
    entry.quantity += row.qty;
    entry.total_revenue += row.total_after_bill_discount;
    if (!entry.menu_category && row.menu_category) {
      entry.menu_category = row.menu_category;
    }
    if (!entry.menu_category_detail && row.menu_category_detail) {
      entry.menu_category_detail = row.menu_category_detail;
    }
    byMenu.set(row.menu, entry);
  }

  const unmatched = new Set<string>();
  for (const agg of byMenu.values()) {
    const unit = cogsByMenu[agg.menu] ?? 0;
    if (unit > 0 && Number.isFinite(unit)) {
      agg.cogs = unit;
    } else {
      agg.cogs = 0;
      unmatched.add(agg.menu);
    }
  }
  const unmatchedMenus = [...unmatched].sort();
  return calculateMenuEngineeringMatrix([...byMenu.values()], unmatchedMenus);
}

function calculateMenuEngineeringMatrix(
  rowsIn: AggRow[],
  unmatchedMenus: string[],
): MenuEngineeringMatrixResult {
  const matchedMenus = rowsIn.filter((r) => r.cogs > 0).length;
  const totalRevenueAll = rowsIn.reduce((s, r) => s + r.total_revenue, 0);
  const totalCogsAll = rowsIn.reduce((s, r) => s + r.cogs * r.quantity, 0);
  const totalProfitAll = totalRevenueAll - totalCogsAll;
  const totalMarginRatio =
    totalRevenueAll > 0 ? totalProfitAll / totalRevenueAll : 0;

  let rows = rowsIn.filter((r) => r.cogs > 0 && r.total_revenue > 0);
  if (rows.length === 0) {
    throw new Error(
      `No valid menu items with cogs > 0 and revenue > 0 (unmatched menus: ${unmatchedMenus.length ? unmatchedMenus.join(", ") : "none"}). Add or fix COGS for sold items.`,
    );
  }

  type MatrixRow = {
    menu: string;
    quantity: number;
    total_revenue: number;
    cogs: number;
    total_cogs: number;
    contribution_margin: number;
    contribution_margin_percentage: number;
    margin_per_unit: number;
    we_value: number;
    category: string;
    action: string;
    menu_category: string | null;
    menu_category_detail: string | null;
  };

  const matrix: MatrixRow[] = rows.map((r) => {
    const totalCogs = r.cogs * r.quantity;
    const contributionMargin = r.total_revenue - totalCogs;
    const weValue =
      r.total_revenue > 0 ? totalCogs / r.total_revenue : 0;
    const marginPerUnit =
      r.quantity > 0 ? contributionMargin / r.quantity : 0;
    return {
      menu: r.menu,
      quantity: r.quantity,
      total_revenue: r.total_revenue,
      cogs: r.cogs,
      total_cogs: totalCogs,
      contribution_margin: contributionMargin,
      contribution_margin_percentage: 0,
      margin_per_unit: marginPerUnit,
      we_value: weValue,
      category: "",
      action: "",
      menu_category: r.menu_category,
      menu_category_detail: r.menu_category_detail,
    };
  });

  const totalMarginMatrix = matrix.reduce(
    (s, r) => s + r.contribution_margin,
    0,
  );
  for (const row of matrix) {
    row.contribution_margin_percentage =
      totalMarginMatrix > 0
        ? row.contribution_margin / totalMarginMatrix
        : 0;
  }

  const avgPopularity =
    matrix.reduce((s, r) => s + r.quantity, 0) / matrix.length;
  const avgMargin =
    matrix.reduce((s, r) => s + r.contribution_margin, 0) / matrix.length;

  for (const row of matrix) {
    const popular = row.quantity >= avgPopularity;
    const profitable = row.contribution_margin >= avgMargin;
    row.category =
      popular && profitable
        ? "star"
        : popular && !profitable
          ? "plow_horse"
          : !popular && profitable
            ? "puzzle"
            : "weak";
    row.action = classifyAction(
      row.category,
      row.contribution_margin_percentage,
      row.margin_per_unit,
      row.quantity,
      avgMargin,
      avgPopularity,
    );
  }

  const totalItems = matrix.length;
  const distribution: MenuEngineeringDistributionItem[] = [];
  for (const category of ["star", "plow_horse", "puzzle", "weak"]) {
    const group = matrix.filter((r) => r.category === category);
    if (group.length === 0) continue;
    const categoryMargin = group.reduce(
      (s, r) => s + r.contribution_margin,
      0,
    );
    distribution.push({
      category,
      item_count: group.length,
      item_share: group.length / totalItems,
      margin_share:
        totalMarginMatrix > 0 ? categoryMargin / totalMarginMatrix : 0,
    });
  }
  distribution.sort(
    (a, b) =>
      b.margin_share - a.margin_share ||
      b.item_share - a.item_share ||
      a.category.localeCompare(b.category),
  );

  matrix.sort(
    (a, b) =>
      b.quantity - a.quantity ||
      b.total_revenue - a.total_revenue ||
      a.menu.localeCompare(b.menu),
  );

  const items: MenuEngineeringMatrixItem[] = matrix.map((row) => ({
    menu: row.menu,
    quantity: Math.round(row.quantity),
    total_revenue: round2(row.total_revenue),
    cogs: round2(row.cogs),
    total_cogs: round2(row.total_cogs),
    contribution_margin: round2(row.contribution_margin),
    contribution_margin_percentage: round4(row.contribution_margin_percentage),
    margin_per_unit: round2(row.margin_per_unit),
    we_value: round4(row.we_value),
    category: row.category,
    action: row.action,
    menu_category: row.menu_category,
    menu_category_detail: row.menu_category_detail,
  }));

  return {
    thresholds: {
      avg_popularity: round2(avgPopularity),
      avg_contribution_margin: round2(avgMargin),
      total_cogs: round2(totalCogsAll),
      total_profit: round2(totalProfitAll),
      total_margin: round4(totalMarginRatio),
    },
    distribution,
    items,
    coverage: {
      matched_menus: matchedMenus,
      unmatched_menus: unmatchedMenus,
    },
  };
}

function classifyAction(
  category: string,
  contributionMarginPercentage: number,
  marginPerUnit: number,
  quantity: number,
  avgMargin: number,
  avgPopularity: number,
): string {
  const remove =
    category === "weak" &&
    contributionMarginPercentage < 0.005 &&
    quantity < avgPopularity;
  const reprice =
    category === "weak" && marginPerUnit >= avgMargin && !remove;
  const promote = category === "puzzle" && !remove && !reprice;
  if (promote) return "promote";
  if (reprice) return "reprice";
  if (remove) return "remove";
  return "keep";
}

function round2(v: number): number {
  return Math.round(v * 100) / 100;
}

function round4(v: number): number {
  return Math.round(v * 10_000) / 10_000;
}
