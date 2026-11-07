import { describe, expect, it } from "vitest";
import {
  activeDailyHours,
  dailyValuesForHours,
} from "./menu-heatmap-hours.js";

describe("activeDailyHours", () => {
  it("returns only hours with quantity >= 1 across items", () => {
    const hours = activeDailyHours([
      {
        daily_heatmap: [
          { hour: 8, quantity: 2 },
          { hour: 9, quantity: 0 },
          { hour: 12, quantity: 1 },
        ],
      },
      {
        daily_heatmap: [
          { hour: 9, quantity: 3 },
          { hour: 22, quantity: 0 },
        ],
      },
    ]);
    expect(hours).toEqual([8, 9, 12]);
  });

  it("returns empty when no hour has sales", () => {
    expect(
      activeDailyHours([
        {
          daily_heatmap: [
            { hour: 10, quantity: 0 },
            { hour: 11, quantity: 0 },
          ],
        },
      ]),
    ).toEqual([]);
    expect(activeDailyHours([])).toEqual([]);
  });
});

describe("dailyValuesForHours", () => {
  it("projects quantities onto the selected hours", () => {
    expect(
      dailyValuesForHours(
        {
          daily_heatmap: [
            { hour: 8, quantity: 2 },
            { hour: 12, quantity: 5 },
          ],
        },
        [8, 9, 12],
      ),
    ).toEqual([2, 0, 5]);
  });
});
