import { useMemo, useState } from "react";
import type { MenuHeatmapPayload } from "../lib/analytics/index.js";
import { useToolInfo } from "../helpers.js";
import {
  activeDailyHours,
  dailyValuesForHours,
} from "./menu-heatmap-hours.js";
import {
  Empty,
  Legend,
  Loading,
  Panel,
  Segmented,
  ToolError,
  cellStyle,
  intensity,
  lightTextClass,
  toolErrorMessage,
  usePersistedState,
} from "./shared.js";

type ViewId = "weekly" | "daily";
type CatId = "star" | "plow_horse" | "puzzle" | "weak";

const WEEKDAYS = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"] as const;
const WEEKDAY_LABEL: Record<string, string> = {
  mon: "Mon",
  tue: "Tue",
  wed: "Wed",
  thu: "Thu",
  fri: "Fri",
  sat: "Sat",
  sun: "Sun",
};
const CAT_LABEL: Record<CatId, string> = {
  star: "Star",
  plow_horse: "Plow horse",
  puzzle: "Puzzle",
  weak: "Weak",
};

const DEFAULT_CATS: Record<CatId, boolean> = {
  star: true,
  plow_horse: true,
  puzzle: true,
  weak: false,
};

function categoryOf(item: MenuHeatmapPayload): CatId | null {
  const raw = item.matrix_category;
  if (!raw) return null;
  const key = String(raw).toLowerCase() as CatId;
  return key in CAT_LABEL ? key : null;
}

function weeklyValues(item: MenuHeatmapPayload): number[] {
  const rows = item.weekly_heatmap ?? [];
  const byDay = new Map(
    rows.map((r) => [String(r.day ?? "").toLowerCase(), Number(r.quantity ?? 0)]),
  );
  return WEEKDAYS.map((d) => byDay.get(d) ?? 0);
}

function weeklyTotal(item: MenuHeatmapPayload): number {
  return weeklyValues(item).reduce((s, n) => s + n, 0);
}

export default function MenuHeatmaps() {
  const { output, isPending } = useToolInfo<"render_menu_heatmaps">();
  const [view, setView] = usePersistedState<ViewId>(
    "menuyukti-menu-heatmaps-view",
    "weekly",
    ["weekly", "daily"],
  );
  const [enabledCats, setEnabledCats] = useState<Record<CatId, boolean>>(
    () => {
      try {
        const raw = localStorage.getItem("menuyukti-menu-heatmaps-cats");
        if (raw) {
          return {
            ...DEFAULT_CATS,
            ...(JSON.parse(raw) as Partial<Record<CatId, boolean>>),
          };
        }
      } catch {
        /* ignore */
      }
      return { ...DEFAULT_CATS };
    },
  );

  function setCat(id: CatId, on: boolean) {
    setEnabledCats((prev) => {
      const next = { ...prev, [id]: on };
      try {
        localStorage.setItem(
          "menuyukti-menu-heatmaps-cats",
          JSON.stringify(next),
        );
      } catch {
        /* ignore */
      }
      return next;
    });
  }

  const heatmaps: MenuHeatmapPayload[] = Array.isArray(output?.heatmaps)
    ? output.heatmaps
    : [];
  const categoriesPresent = heatmaps.some((h) => categoryOf(h) != null);

  const visible = useMemo(() => {
    const pool = [...heatmaps].sort(
      (a, b) => weeklyTotal(b) - weeklyTotal(a),
    );
    if (!categoriesPresent) return pool;
    return pool.filter((item) => {
      const cat = categoryOf(item);
      if (!cat) return true;
      return enabledCats[cat];
    });
  }, [heatmaps, categoriesPresent, enabledCats]);

  const dailyHours = useMemo(
    () => (view === "daily" ? activeDailyHours(visible) : []),
    [view, visible],
  );

  const columns =
    view === "daily"
      ? dailyHours.map((h) => String(h))
      : WEEKDAYS.map((d) => WEEKDAY_LABEL[d]!);

  const grid = useMemo(() => {
    return visible.map((item) =>
      view === "daily"
        ? dailyValuesForHours(item, dailyHours)
        : weeklyValues(item),
    );
  }, [visible, view, dailyHours]);

  const { min, max } = useMemo(() => {
    const vals = grid.flat().filter(Number.isFinite);
    return {
      min: vals.length ? Math.min(...vals) : 0,
      max: vals.length ? Math.max(...vals) : 1,
    };
  }, [grid]);

  if (isPending || !output) {
    return <Loading label="Menu heatmaps" />;
  }

  const toolError = toolErrorMessage(output);
  if (toolError) {
    return (
      <ToolError
        title="Menu heatmaps"
        description="Unit quantity by dish — daily or weekly."
        message={toolError}
      />
    );
  }

  if (!Array.isArray(output.heatmaps) || heatmaps.length === 0) {
    return (
      <Panel
        title="Menu heatmaps"
        description="Unit quantity by dish — daily or weekly."
      >
        <Empty>No menu heatmap data for this period.</Empty>
      </Panel>
    );
  }

  if (visible.length === 0) {
    return (
      <Panel
        title="Menu heatmaps"
        description="Unit quantity by dish — daily or weekly."
      >
        <Empty>
          No menus match the selected matrix categories. Enable a category
          above.
        </Empty>
        {categoriesPresent ? (
          <div className="myk-filter-bar" aria-label="Matrix categories">
            {(Object.keys(CAT_LABEL) as CatId[]).map((id) => (
              <label key={id}>
                <input
                  type="checkbox"
                  checked={enabledCats[id]}
                  onChange={(e) => setCat(id, e.target.checked)}
                />{" "}
                {CAT_LABEL[id]}
              </label>
            ))}
          </div>
        ) : null}
      </Panel>
    );
  }

  const pos = output.pos ? String(output.pos) : "";
  const period = heatmaps[0]?.reporting_period ?? "";
  const meta = [
    pos ? `POS: ${pos}` : null,
    period || null,
    `${visible.length} of ${heatmaps.length} menus`,
    view === "daily"
      ? dailyHours.length > 0
        ? `by hour (${dailyHours.length} with sales)`
        : "by hour (none with sales)"
      : "by weekday",
  ]
    .filter(Boolean)
    .join(" · ");

  return (
    <Panel
      title="Menu heatmaps"
      description="Unit quantity by dish — daily (hour) or weekly (weekday). Toggle matrix categories when COGS enrichment is present."
    >
      <p className="myk-meta">{meta}</p>
      <Segmented
        label="View"
        value={view}
        onChange={setView}
        options={[
          { id: "weekly", label: "Weekly" },
          { id: "daily", label: "Daily" },
        ]}
      />
      {categoriesPresent ? (
        <div className="myk-filter-bar" aria-label="Matrix categories">
          {(Object.keys(CAT_LABEL) as CatId[]).map((id) => (
            <label key={id}>
              <input
                type="checkbox"
                checked={enabledCats[id]}
                onChange={(e) => setCat(id, e.target.checked)}
              />{" "}
              {CAT_LABEL[id]}
            </label>
          ))}
        </div>
      ) : null}
      <Legend caption="Color + number = units sold" />
      <div className="myk-scroll">
        <table className="myk-table">
          <thead>
            <tr>
              <th className="row-label">Menu</th>
              {columns.map((col) => (
                <th
                  key={col}
                  className={view === "daily" ? "hour" : undefined}
                >
                  {col}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {visible.map((item, r) => {
              const values = grid[r] ?? [];
              const cat = categoryOf(item);
              const menu = item.menu ?? "Unknown";
              const total = values.reduce((s, n) => s + n, 0);
              return (
                <tr key={`${menu}-${r}`}>
                  <td
                    className="row-label"
                    title={[
                      menu,
                      cat ? CAT_LABEL[cat] : null,
                      `${total} units`,
                    ]
                      .filter(Boolean)
                      .join(" · ")}
                  >
                    {menu}
                    {cat ? (
                      <span className="myk-badge">{CAT_LABEL[cat]}</span>
                    ) : null}
                  </td>
                  {values.map((value, c) => {
                    const t = intensity(value, min, max);
                    return (
                      <td
                        key={c}
                        className={`cell ${lightTextClass(t)}`}
                        style={cellStyle(t)}
                      >
                        {Math.round(value)}
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
