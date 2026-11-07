/** Hours present on a daily heatmap row (0–23). */
export type DailyHeatRow = { hour?: number; quantity?: number };

export type DailyHeatmapSource = {
  daily_heatmap?: DailyHeatRow[];
};

/**
 * Hours 0–23 that have total quantity >= 1 across the given items.
 * Used so the daily menu heatmap defaults to operating hours with demand.
 */
export function activeDailyHours(items: DailyHeatmapSource[]): number[] {
  const totals = new Array<number>(24).fill(0);
  for (const item of items) {
    for (const row of item.daily_heatmap ?? []) {
      const hour = Number(row.hour);
      const qty = Number(row.quantity ?? 0);
      if (!Number.isInteger(hour) || hour < 0 || hour > 23) continue;
      if (!Number.isFinite(qty) || qty <= 0) continue;
      totals[hour]! += qty;
    }
  }
  return totals
    .map((qty, hour) => (qty >= 1 ? hour : -1))
    .filter((hour) => hour >= 0);
}

/** Quantities for the given hours (0 when missing). */
export function dailyValuesForHours(
  item: DailyHeatmapSource,
  hours: number[],
): number[] {
  const byHour = new Map<number, number>();
  for (const row of item.daily_heatmap ?? []) {
    const hour = Number(row.hour);
    if (!Number.isInteger(hour) || hour < 0 || hour > 23) continue;
    byHour.set(hour, Number(row.quantity ?? 0));
  }
  return hours.map((h) => byHour.get(h) ?? 0);
}
