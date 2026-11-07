import { useMemo } from "react";
import { useToolInfo } from "../helpers.js";
import type { MenuBasketPair } from "../lib/analytics/index.js";
import {
  cellStyle,
  Empty,
  intensity,
  Legend,
  Loading,
  lightTextClass,
  Panel,
  ToolError,
  toolErrorMessage,
  usePersistedJson,
} from "./shared.js";

type SortKey = "label" | number;
type SortState = { key: SortKey; dir: "asc" | "desc" };

function isSortState(value: unknown): value is SortState {
  if (!value || typeof value !== "object") return false;
  const v = value as { key?: unknown; dir?: unknown };
  const keyOk =
    v.key === "label" ||
    (typeof v.key === "number" && Number.isInteger(v.key) && v.key >= 0);
  const dirOk = v.dir === "asc" || v.dir === "desc";
  return keyOk && dirOk;
}

function cellLift(
  matrix: Array<Array<number | null>>,
  row: number,
  col: number,
): number | null {
  const rowVals = matrix[row];
  if (!rowVals) return null;
  const raw = rowVals[col];
  if (raw === null || raw === undefined) return null;
  const n = Number(raw);
  return Number.isFinite(n) ? n : null;
}

function formatLift(n: number): string {
  return n.toFixed(2);
}

function findPair(
  pairs: MenuBasketPair[],
  a: string,
  b: string,
): MenuBasketPair | undefined {
  return pairs.find(
    (p) =>
      (p.menu_a === a && p.menu_b === b) || (p.menu_a === b && p.menu_b === a),
  );
}

export default function PairLiftMatrix() {
  const { output, isPending } = useToolInfo<"render_pair_lift_matrix">();
  const [sort, setSort] = usePersistedJson<SortState>(
    "menuyukti-pair-lift-sort",
    { key: "label", dir: "asc" },
    isSortState,
  );

  const focus: string[] = Array.isArray(output?.focus_menus)
    ? output.focus_menus
    : [];
  const matrix: Array<Array<number | null>> = Array.isArray(output?.matrix_lift)
    ? output.matrix_lift
    : [];
  const pairs: MenuBasketPair[] = Array.isArray(output?.pairs)
    ? output.pairs
    : [];

  const sortKey: SortKey =
    typeof sort.key === "number" && sort.key >= focus.length
      ? "label"
      : sort.key;
  const sortDir = sort.dir;

  const { min, max, order } = useMemo(() => {
    const lifts: number[] = [];
    for (let r = 0; r < focus.length; r += 1) {
      for (let c = 0; c < focus.length; c += 1) {
        if (r === c) continue;
        const lift = cellLift(matrix, r, c);
        if (lift !== null) lifts.push(lift);
      }
    }
    const indices = focus.map((_, i) => i);
    indices.sort((i, j) => {
      if (sortKey === "label") {
        const cmp = focus[i]!.localeCompare(focus[j]!);
        return sortDir === "asc" ? cmp : -cmp;
      }
      const a = i === sortKey ? null : cellLift(matrix, i, sortKey);
      const b = j === sortKey ? null : cellLift(matrix, j, sortKey);
      const aVal = a === null ? Number.NEGATIVE_INFINITY : a;
      const bVal = b === null ? Number.NEGATIVE_INFINITY : b;
      const cmp = aVal - bVal;
      if (cmp !== 0) return sortDir === "asc" ? cmp : -cmp;
      return focus[i]!.localeCompare(focus[j]!);
    });
    return {
      min: lifts.length ? Math.min(...lifts) : 0,
      max: lifts.length ? Math.max(...lifts) : 1,
      order: indices,
    };
  }, [focus, matrix, sortKey, sortDir]);

  function toggleSort(next: SortKey) {
    setSort((prev) => {
      if (prev.key === next) {
        return { key: next, dir: prev.dir === "asc" ? "desc" : "asc" };
      }
      return { key: next, dir: next === "label" ? "asc" : "desc" };
    });
  }

  if (isPending || !output) {
    return <Loading label="Pair lift index" />;
  }

  const toolError = toolErrorMessage(output);
  if (toolError) {
    return (
      <ToolError
        title="Pair lift index"
        description="Focus menu × focus menu co-purchase lift."
        message={toolError}
      />
    );
  }

  if (focus.length < 2 || matrix.length === 0) {
    return (
      <Panel
        title="Pair lift index"
        description="Focus menu × focus menu co-purchase lift."
      >
        <Empty>
          {focus.length < 2
            ? "Need at least two focus menus to show pair lift."
            : "Empty pair lift matrix for this period."}
        </Empty>
      </Panel>
    );
  }

  const pos = output.pos ? String(output.pos) : "";
  const scope = output.scope ? String(output.scope) : "";
  const meta = [
    pos ? `POS: ${pos}` : null,
    scope || null,
    output.total_orders != null ? `${output.total_orders} orders` : null,
    `lift range ${formatLift(min)}–${formatLift(max)}`,
  ]
    .filter(Boolean)
    .join(" · ");

  return (
    <Panel
      title="Pair lift index"
      description="Co-purchase lift among focus menus. Lift > 1 means ordered together more than chance; use for combos and upsells."
    >
      <p className="myk-meta">{meta}</p>
      <Legend caption="Color + number = lift" />
      <p className="myk-sort-hint">
        Click a column header to sort rows. Menu sorts A–Z; lift columns sort by
        that pair partner.
      </p>
      <div className="myk-scroll">
        <table className="myk-table">
          <thead>
            <tr>
              <th
                className="sortable row-label"
                aria-sort={
                  sortKey === "label"
                    ? sortDir === "asc"
                      ? "ascending"
                      : "descending"
                    : "none"
                }
              >
                <button type="button" onClick={() => toggleSort("label")}>
                  <span>Menu</span>
                  <span className="sort-ind">
                    {sortKey === "label"
                      ? sortDir === "asc"
                        ? "↑"
                        : "↓"
                      : "↕"}
                  </span>
                </button>
              </th>
              {focus.map((menu, col) => (
                <th
                  key={menu}
                  className="sortable"
                  aria-sort={
                    sortKey === col
                      ? sortDir === "asc"
                        ? "ascending"
                        : "descending"
                      : "none"
                  }
                >
                  <button
                    type="button"
                    title={`Sort rows by lift with ${menu}`}
                    onClick={() => toggleSort(col)}
                  >
                    <span>{menu}</span>
                    <span className="sort-ind">
                      {sortKey === col ? (sortDir === "asc" ? "↑" : "↓") : "↕"}
                    </span>
                  </button>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {order.map((r) => {
              const rowMenu = focus[r]!;
              return (
                <tr key={rowMenu}>
                  <td className="row-label">{rowMenu}</td>
                  {focus.map((colMenu, c) => {
                    if (r === c) {
                      return (
                        <td
                          key={colMenu}
                          className="cell null-cell"
                          title={`${rowMenu} · same item (no lift)`}
                        >
                          —
                        </td>
                      );
                    }
                    const lift = cellLift(matrix, r, c);
                    if (lift === null) {
                      return (
                        <td
                          key={colMenu}
                          className="cell null-cell"
                          title={`${rowMenu} × ${colMenu}: no lift (too few co-orders)`}
                        >
                          —
                        </td>
                      );
                    }
                    const t = intensity(lift, min, max);
                    const pair = findPair(pairs, rowMenu, colMenu);
                    const co = pair?.co_order_count;
                    return (
                      <td
                        key={colMenu}
                        className={`cell ${lightTextClass(t)}`}
                        style={cellStyle(t)}
                        title={[
                          `${rowMenu} × ${colMenu}`,
                          `Lift: ${formatLift(lift)}`,
                          co != null ? `Co-orders: ${co}` : null,
                          lift > 1
                            ? "Above chance"
                            : lift < 1
                              ? "Below chance"
                              : "At chance",
                        ]
                          .filter(Boolean)
                          .join(" · ")}
                      >
                        {formatLift(lift)}
                      </td>
                    );
                  })}
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </Panel>
  );
}
