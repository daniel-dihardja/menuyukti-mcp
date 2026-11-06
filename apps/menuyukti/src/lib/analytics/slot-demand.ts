import type { LineItem } from "../line-item.js";
import { parseOrderTime, WEEKDAY_ORDER, weekdayAbbr } from "./time.js";

export const LOW_DEMAND_THRESHOLD = 0.9;
export const HIGH_DEMAND_THRESHOLD = 1.1;

const MEAL_PERIODS: Array<{
  id: string;
  label: string;
  hoursLabel: string;
  hours: number[];
}> = [
  {
    id: "breakfast",
    label: "Breakfast",
    hoursLabel: "05:00–10:59",
    hours: [5, 6, 7, 8, 9, 10],
  },
  {
    id: "lunch",
    label: "Lunch",
    hoursLabel: "11:00–14:59",
    hours: [11, 12, 13, 14],
  },
  {
    id: "afternoon",
    label: "Afternoon",
    hoursLabel: "15:00–16:59",
    hours: [15, 16],
  },
  {
    id: "dinner",
    label: "Dinner",
    hoursLabel: "17:00–21:59",
    hours: [17, 18, 19, 20, 21],
  },
  {
    id: "late_night",
    label: "Late Night",
    hoursLabel: "22:00–04:59",
    hours: [22, 23, 0, 1, 2, 3, 4],
  },
];

const SLOT_COUNT = WEEKDAY_ORDER.length * MEAL_PERIODS.length;
const MEAN_SLOT_TRAFFIC_SHARE = 1 / SLOT_COUNT;

export type SlotDemandCell = {
  day: string;
  meal_period: string;
  meal_period_label: string;
  meal_period_hours_label: string;
  order_count: number;
  traffic_share: number;
  demand_index: number;
  relative_demand: string;
};

function mealPeriodForHour(hour: number) {
  for (const p of MEAL_PERIODS) {
    if (p.hours.includes(hour)) {
      return { id: p.id, label: p.label, hoursLabel: p.hoursLabel };
    }
  }
  const late = MEAL_PERIODS[4]!;
  return { id: late.id, label: late.label, hoursLabel: late.hoursLabel };
}

function relativeDemand(demandIndex: number): string {
  if (demandIndex < LOW_DEMAND_THRESHOLD) return "low";
  if (demandIndex > HIGH_DEMAND_THRESHOLD) return "high";
  return "average";
}

function round4(value: number): number {
  return Math.round(value * 10_000) / 10_000;
}

export function computeSlotDemandProfileFromOrders(
  rows: LineItem[],
): SlotDemandCell[] {
  if (rows.length === 0) return [];

  const billMinTime = new Map<string, string>();
  for (const row of rows) {
    const bill = row.bill_number.trim();
    if (!bill) continue;
    const prev = billMinTime.get(bill);
    if (!prev || row.order_time < prev) billMinTime.set(bill, row.order_time);
  }
  const totalOrders = billMinTime.size;
  if (totalOrders === 0) return [];

  const slotCounts = new Map<string, number>();
  for (const orderTime of billMinTime.values()) {
    const parsed = parseOrderTime(orderTime);
    if (!parsed) continue;
    const day = weekdayAbbr(orderTime);
    const period = mealPeriodForHour(parsed.hour).id;
    const key = `${day}|${period}`;
    slotCounts.set(key, (slotCounts.get(key) ?? 0) + 1);
  }

  const cells: SlotDemandCell[] = [];
  for (const day of WEEKDAY_ORDER) {
    for (const period of MEAL_PERIODS) {
      const orderCount = slotCounts.get(`${day}|${period.id}`) ?? 0;
      const trafficShare = orderCount / totalOrders;
      const demandIndex =
        MEAN_SLOT_TRAFFIC_SHARE > 0
          ? trafficShare / MEAN_SLOT_TRAFFIC_SHARE
          : 0;
      cells.push({
        day,
        meal_period: period.id,
        meal_period_label: period.label,
        meal_period_hours_label: period.hoursLabel,
        order_count: orderCount,
        traffic_share: round4(trafficShare),
        demand_index: round4(demandIndex),
        relative_demand: relativeDemand(demandIndex),
      });
    }
  }
  return cells;
}
