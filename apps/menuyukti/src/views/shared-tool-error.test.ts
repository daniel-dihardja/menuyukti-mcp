import { describe, expect, it } from "vitest";
import { toolErrorMessage } from "./shared.js";

describe("toolErrorMessage", () => {
  it("returns trimmed error strings from errJson-shaped output", () => {
    expect(toolErrorMessage({ error: "  COGS required  " })).toBe(
      "COGS required",
    );
  });

  it("returns null for successful or empty payloads", () => {
    expect(toolErrorMessage(null)).toBeNull();
    expect(toolErrorMessage({})).toBeNull();
    expect(toolErrorMessage({ error: "" })).toBeNull();
    expect(toolErrorMessage({ error: 42 })).toBeNull();
    expect(
      toolErrorMessage({ slot_demand_profile: [], pos: "esb" }),
    ).toBeNull();
  });
});
