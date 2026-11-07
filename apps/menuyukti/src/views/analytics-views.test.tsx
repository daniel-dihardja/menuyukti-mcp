/** @vitest-environment jsdom */
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  menuHeatmapsError,
  menuHeatmapsPopulated,
  pairLiftError,
  pairLiftPopulated,
  venueSlotError,
  venueSlotPopulated,
  weakItemsError,
  weakItemsPopulated,
} from "./fixtures/index.js";

const toolInfo = vi.hoisted(() => ({
  output: null as unknown,
  isPending: false,
}));

vi.mock("../helpers.js", () => ({
  useToolInfo: () => ({
    output: toolInfo.output,
    isPending: toolInfo.isPending,
    toolInput: null,
  }),
  useCallTool: () => async () => {
    throw new Error("not used");
  },
}));

import MenuHeatmaps from "./menu-heatmaps.js";
import PairLiftMatrix from "./pair-lift-matrix.js";
import VenueSlotStrength from "./venue-slot-strength.js";
import WeakItemsReview from "./weak-items-review.js";

describe("analytics widget views", () => {
  beforeEach(() => {
    toolInfo.output = null;
    toolInfo.isPending = false;
  });

  afterEach(() => {
    cleanup();
  });

  it("venue slot strength renders populated fixture", () => {
    toolInfo.output = venueSlotPopulated;
    render(<VenueSlotStrength />);
    expect(screen.getByText(/Venue slot strength/i)).toBeTruthy();
    expect(screen.getByText(/Showing demand index/i)).toBeTruthy();
  });

  it("venue slot strength surfaces tool errors", () => {
    toolInfo.output = venueSlotError;
    render(<VenueSlotStrength />);
    expect(screen.getByText(venueSlotError.error)).toBeTruthy();
  });

  it("pair lift matrix renders populated fixture", () => {
    toolInfo.output = pairLiftPopulated;
    render(<PairLiftMatrix />);
    expect(screen.getByText(/Pair lift index/i)).toBeTruthy();
    expect(screen.getAllByText("Latte").length).toBeGreaterThan(0);
  });

  it("pair lift matrix surfaces tool errors", () => {
    toolInfo.output = pairLiftError;
    render(<PairLiftMatrix />);
    expect(screen.getByText(pairLiftError.error)).toBeTruthy();
  });

  it("menu heatmaps renders populated fixture", () => {
    toolInfo.output = menuHeatmapsPopulated;
    render(<MenuHeatmaps />);
    expect(screen.getByText(/Menu heatmaps/i)).toBeTruthy();
    expect(screen.getByText("Latte")).toBeTruthy();
  });

  it("menu heatmaps surfaces tool errors", () => {
    toolInfo.output = menuHeatmapsError;
    render(<MenuHeatmaps />);
    expect(screen.getByText(menuHeatmapsError.error)).toBeTruthy();
  });

  it("weak items review renders populated fixture", () => {
    toolInfo.output = weakItemsPopulated;
    render(<WeakItemsReview />);
    expect(screen.getByText(/Weak items review/i)).toBeTruthy();
    expect(screen.getByText("Mystery Soup")).toBeTruthy();
  });

  it("weak items review surfaces tool errors", () => {
    toolInfo.output = weakItemsError;
    render(<WeakItemsReview />);
    expect(screen.getByText(weakItemsError.error)).toBeTruthy();
  });
});
