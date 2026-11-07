import { useMemo } from "react";
import { useToolInfo } from "../helpers.js";
import {
  Empty,
  Legend,
  Loading,
  Panel,
  Segmented,
  cellStyle,
  intensity,
  lightTextClass,
  usePersistedState,
} from "./shared.js";

type SlotCell = {
  day?: string;
  meal_period?: string;
  meal_period_label?: string;
  meal_period_hours_label?: string;
  order_count?: number;
  traffic_share?: number;
  demand_index?: number;
  relative_demand?: string;
};

type MetricId = "demand" | "orders" | "share";

const DAYS = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"] as const;
const DAY_LABEL: Record<string, string> = {
  mon: "Mon",
  tue: "Tue",
  wed: "Wed",
  thu: "Thu",
  fri: "Fri",
  sat: "Sat",
  sun: "Sun",
};
const PERIODS = [
  "breakfast",
  "lunch",
  "afternoon",
  "dinner",
  "late_night",
] as const;

const METRICS: Record<
  MetricId,
  {
    showing: string;
    legend: string;
    field: (c: SlotCell) => number;
    format: (n: number) => string;
  }
> = {
  demand: {
    showing: "Showing demand index",
    legend: "Color + number = demand index",
    field: (c) => Number(c.demand_index ?? 0),
    format: (n) => (Number.isFinite(n) ? n.toFixed(2) : "—"),
  },
  orders: {
    showing: "Showing order count",
    legend: "Color + number = orders",
    field: (c) => Number(c.order_count ?? 0),
    format: (n) => (Number.isFinite(n) ? String(Math.round(n)) : "—"),
  },
  share: {
    showing: "Showing traffic share",
    legend: "Color + number = traffic share",
    field: (c) => Number(c.traffic_share ?? 0),
    format: (n) =>
      Number.isFinite(n) ? `${(n * 100).toFixed(1)}%` : "—",
  },
};

function periodLabel(cell: SlotCell | undefined, period: string): string {
  if (!cell) return period;
  const label = cell.meal_period_label ?? "";
  const hours = cell.meal_period_hours_label ?? "";
  if (label && hours) return `${label} (${hours})`;
  return label || cell.meal_period || period;
}

export default function VenueSlotStrength() {
  const { output, isPending } = useToolInfo<"render_venue_slot_strength">();
  const [metricId, setMetricId] = usePersistedState<MetricId>(
    "menuyukti-venue-slot-metric",
    "demand",
    ["demand", "orders", "share"],
  );

  const profile = (output?.slot_demand_profile ?? []) as SlotCell[];
  const metric = METRICS[metricId];

  const { byKey, min, max, periodLabels } = useMemo(() => {
    const map = new Map<string, SlotCell>();
    for (const cell of profile) {
      const day = String(cell.day ?? "").toLowerCase();
      const period = String(cell.meal_period ?? "").toLowerCase();
      if (!day || !period) continue;
      map.set(`${day}:${period}`, cell);
    }
    const values = profile.map((c) => metric.field(c)).filter(Number.isFinite);
    const periodLabelsLocal = PERIODS.map((period) => {
      const sample = DAYS.map((d) => map.get(`${d}:${period}`)).find(Boolean);
      return periodLabel(sample, period);
    });
    return {
      byKey: map,
      min: values.length ? Math.min(...values) : 0,
      max: values.length ? Math.max(...values) : 1,
      periodLabels: periodLabelsLocal,
    };
  }, [profile, metric]);

  if (isPending || !output) {
    return <Loading label="Venue slot strength" />;
  }

  if (!Array.isArray(output.slot_demand_profile)) {
    return (
      <Panel
        title="Venue slot strength"
        description="Weekday × meal-period demand."
      >
        <Empty>Missing slot_demand_profile in tool result.</Empty>
      </Panel>
    );
  }

  if (profile.length === 0) {
    return (
      <Panel
        title="Venue slot strength"
        description="Weekday × meal-period demand."
      >
        <Empty>No demand slots in this period (empty profile).</Empty>
      </Panel>
    );
  }

  const pos = output.pos ? String(output.pos) : "";
  const meta = [
    pos ? `POS: ${pos}` : null,
    `${profile.length} slots`,
  ]
    .filter(Boolean)
    .join(" · ");

  return (
    <Panel
      title="Venue slot strength"
      description="Weekday × meal-period demand. Index 1.0 = average traffic across all slots. Use for staffing and when to promote."
    >
      <p className="myk-meta">{meta}</p>
      <Segmented
        label="Show"
        value={metricId}
        onChange={setMetricId}
        options={[
          { id: "demand", label: "Demand", ariaLabel: "Demand index" },
          { id: "orders", label: "Orders", ariaLabel: "Order count" },
          { id: "share", label: "Share", ariaLabel: "Traffic share" },
        ]}
      />
      <p className="myk-showing" style={{ marginTop: 8 }}>
        {metric.showing}
      </p>
      <Legend caption={metric.legend} />
      <div className="myk-scroll">
        <table className="myk-table" style={{ minWidth: 520 }}>
          <thead>
            <tr>
              <th className="row-label">Meal period</th>
              {DAYS.map((day) => (
                <th key={day}>{DAY_LABEL[day]}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {PERIODS.map((period, periodIdx) => (
              <tr key={period}>
                <td className="row-label">{periodLabels[periodIdx]}</td>
                {DAYS.map((day) => {
                  const cell = byKey.get(`${day}:${period}`);
                  const value = cell ? metric.field(cell) : 0;
                  const t = intensity(value, min, max);
                  return (
                    <td
                      key={day}
                      className={`cell ${lightTextClass(t)}`}
                      style={cellStyle(t)}
                      title={
                        cell
                          ? `${DAY_LABEL[day]} ${periodLabel(cell, period)}: ${metric.format(value)}${cell.relative_demand ? ` (${cell.relative_demand})` : ""}`
                          : undefined
                      }
                    >
                      {metric.format(value)}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </Panel>
  );
}
