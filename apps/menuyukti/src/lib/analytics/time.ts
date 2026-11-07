/** Parse LineItem.order_time (`YYYY-MM-DDTHH:mm:ss`) without shifting timezones. */

export type ParsedOrderTime = {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  second: number;
  /** 0 = Monday … 6 = Sunday (chrono num_days_from_monday). */
  weekdayFromMonday: number;
  dateKey: string;
};

export function parseOrderTime(orderTime: string): ParsedOrderTime | null {
  const m = orderTime
    .trim()
    .match(/^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})/);
  if (!m) return null;
  const year = Number(m[1]);
  const month = Number(m[2]);
  const day = Number(m[3]);
  const hour = Number(m[4]);
  const minute = Number(m[5]);
  const second = Number(m[6]);
  // Use UTC noon math for weekday to avoid DST edge cases on date-only.
  const utc = Date.UTC(year, month - 1, day, 12, 0, 0);
  const jsDay = new Date(utc).getUTCDay(); // 0=Sun … 6=Sat
  const weekdayFromMonday = (jsDay + 6) % 7;
  return {
    year,
    month,
    day,
    hour,
    minute,
    second,
    weekdayFromMonday,
    dateKey: `${m[1]}-${m[2]}-${m[3]}`,
  };
}

export const WEEKDAY_ORDER = [
  "mon",
  "tue",
  "wed",
  "thu",
  "fri",
  "sat",
  "sun",
] as const;

export function weekdayAbbr(orderTime: string): string {
  const parsed = parseOrderTime(orderTime);
  if (!parsed) return "mon";
  return WEEKDAY_ORDER[parsed.weekdayFromMonday] ?? "mon";
}
