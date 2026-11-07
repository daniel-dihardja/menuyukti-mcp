import { describe, expect, it } from "vitest";
import { viewNameFromFile } from "./self-contained-views.js";

describe("viewNameFromFile", () => {
  it("parses hashed Vite asset filenames", () => {
    expect(viewNameFromFile("assets/venue-slot-strength-nXJ0Ondf.js")).toBe(
      "venue-slot-strength",
    );
    expect(viewNameFromFile("assets/hello-world-BOn4Um8t.js")).toBe(
      "hello-world",
    );
    expect(viewNameFromFile("assets/weak-items-review-Bg1e2VWx.js")).toBe(
      "weak-items-review",
    );
  });

  it("returns null for unexpected paths", () => {
    expect(viewNameFromFile("assets/style-DU-lG4zV.css")).toBeNull();
    expect(viewNameFromFile("")).toBeNull();
  });
});
