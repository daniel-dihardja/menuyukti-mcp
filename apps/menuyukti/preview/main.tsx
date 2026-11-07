import type React from "react";
import { StrictMode, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import {
  type FixtureKind,
  type WidgetFixtureId,
  widgetFixtures,
} from "../src/views/fixtures/index.js";
import MenuHeatmaps from "../src/views/menu-heatmaps.js";
import PairLiftMatrix from "../src/views/pair-lift-matrix.js";
import VenueSlotStrength from "../src/views/venue-slot-strength.js";
import WeakItemsReview from "../src/views/weak-items-review.js";
import "../src/views/widget.css";

const VIEWS: Record<
  WidgetFixtureId,
  { label: string; Component: () => React.ReactNode }
> = {
  "venue-slot-strength": {
    label: "Venue slot strength",
    Component: VenueSlotStrength,
  },
  "pair-lift-matrix": {
    label: "Pair lift matrix",
    Component: PairLiftMatrix,
  },
  "menu-heatmaps": { label: "Menu heatmaps", Component: MenuHeatmaps },
  "weak-items-review": {
    label: "Weak items review",
    Component: WeakItemsReview,
  },
};

const KINDS: FixtureKind[] = ["populated", "empty", "error"];

function readQuery(): { view: WidgetFixtureId; kind: FixtureKind } {
  const params = new URLSearchParams(window.location.search);
  const viewParam = params.get("view") as WidgetFixtureId | null;
  const kindParam = params.get("kind") as FixtureKind | null;
  const view =
    viewParam && viewParam in widgetFixtures
      ? viewParam
      : "venue-slot-strength";
  const kind = kindParam && KINDS.includes(kindParam) ? kindParam : "populated";
  return { view, kind };
}

function PreviewApp() {
  const initial = useMemo(() => readQuery(), []);
  const [view, setView] = useState<WidgetFixtureId>(initial.view);
  const [kind, setKind] = useState<FixtureKind>(initial.kind);

  const output = widgetFixtures[view][kind];
  window.__MENUYUKTI_PREVIEW__ = { output, isPending: false };

  const { Component } = VIEWS[view];

  function update(nextView: WidgetFixtureId, nextKind: FixtureKind) {
    setView(nextView);
    setKind(nextKind);
    const url = new URL(window.location.href);
    url.searchParams.set("view", nextView);
    url.searchParams.set("kind", nextKind);
    window.history.replaceState({}, "", url);
  }

  return (
    <div style={{ fontFamily: "system-ui, sans-serif" }}>
      <header
        style={{
          display: "flex",
          flexWrap: "wrap",
          gap: 12,
          alignItems: "center",
          padding: "12px 16px",
          borderBottom: "1px solid #ddd",
          background: "#fafafa",
        }}
      >
        <strong>Menuyukti widget fixtures</strong>
        <label>
          View{" "}
          <select
            value={view}
            onChange={(e) => update(e.target.value as WidgetFixtureId, kind)}
          >
            {(Object.keys(VIEWS) as WidgetFixtureId[]).map((id) => (
              <option key={id} value={id}>
                {VIEWS[id].label}
              </option>
            ))}
          </select>
        </label>
        <label>
          Fixture{" "}
          <select
            value={kind}
            onChange={(e) => update(view, e.target.value as FixtureKind)}
          >
            {KINDS.map((k) => (
              <option key={k} value={k}>
                {k}
              </option>
            ))}
          </select>
        </label>
        <span style={{ color: "#666", fontSize: 13 }}>
          No Excel / MCP — fixture structuredContent only
        </span>
      </header>
      <Component key={`${view}:${kind}`} />
    </div>
  );
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <PreviewApp />
  </StrictMode>,
);
