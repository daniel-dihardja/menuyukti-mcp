#!/usr/bin/env python3
"""Render Menuyukti get_weak_items_review JSON as a sortable HTML table.

Default sort matches v3 WeakTable: contribution_margin_share ascending.
No action column.
"""

from __future__ import annotations

import argparse
import html
import json
import sys
from pathlib import Path
from typing import Any


def load_payload(path: Path | None) -> dict[str, Any]:
    raw = sys.stdin.read() if path is None else path.read_text(encoding="utf-8")
    data = json.loads(raw)
    if not isinstance(data, dict):
        raise SystemExit("expected a JSON object from get_weak_items_review")
    return data


def as_items(payload: dict[str, Any]) -> list[dict[str, Any]]:
    items = payload.get("items")
    if items is None:
        items = payload.get("rows")
    if not isinstance(items, list):
        return []
    return [row for row in items if isinstance(row, dict)]


def fmt_share(value: Any) -> str:
    try:
        n = float(value)
    except (TypeError, ValueError):
        return "—"
    return f"{n * 100:.1f}%"


def fmt_num(value: Any, digits: int = 2) -> str:
    if value is None:
        return "—"
    try:
        n = float(value)
    except (TypeError, ValueError):
        return "—"
    if digits == 0:
        return f"{int(round(n))}"
    return f"{n:.{digits}f}"


def build_html(payload: dict[str, Any]) -> str:
    items = as_items(payload)
    stats = payload.get("stats") if isinstance(payload.get("stats"), dict) else {}
    period_start = payload.get("period_start") or ""
    period_end = payload.get("period_end") or ""
    show_avg = any(row.get("avg_qty_per_day") is not None for row in items)

    # Default sort: margin share asc, then menu
    items = sorted(
        items,
        key=lambda r: (
            float(r.get("contribution_margin_share") or 0.0),
            str(r.get("menu") or ""),
        ),
    )

    rows_json = json.dumps(items, ensure_ascii=False)
    show_avg_js = "true" if show_avg else "false"

    item_count = stats.get("item_count", len(items))
    total_menu = stats.get("total_menu_items", "—")
    item_share = fmt_share(stats.get("item_share"))
    margin_share = fmt_share(stats.get("margin_share"))
    margin_amount = fmt_num(stats.get("margin_amount"), 2)
    period_days = stats.get("period_days")
    period_label = " — ".join(p for p in (str(period_start), str(period_end)) if p) or "—"
    if period_days is not None:
        period_label = f"{period_label} ({period_days} days)"

    title = html.escape(str(payload.get("workspace") or payload.get("pos") or "Weak items"))

    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Weak items — {title}</title>
  <style>
    :root {{
      color-scheme: light;
      --ink: #0f172a;
      --muted: #64748b;
      --line: #e2e8f0;
      --head: #f8fafc;
      --accent: #1f6f4a;
      --chip: #ecfdf5;
    }}
    body {{
      margin: 0;
      font: 14px/1.45 system-ui, -apple-system, Segoe UI, sans-serif;
      color: var(--ink);
      background: #fff;
      padding: 24px;
    }}
    h1 {{ font-size: 1.25rem; margin: 0 0 8px; }}
    .meta {{ color: var(--muted); margin-bottom: 16px; }}
    .kpis {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
      gap: 12px;
      margin-bottom: 20px;
    }}
    .kpi {{
      border: 1px solid var(--line);
      border-radius: 10px;
      padding: 12px 14px;
      background: var(--head);
    }}
    .kpi .label {{ color: var(--muted); font-size: 0.8rem; }}
    .kpi .value {{ font-size: 1.25rem; font-variant-numeric: tabular-nums; margin-top: 4px; }}
    .hint {{ color: var(--muted); font-size: 0.85rem; margin-bottom: 12px; }}
    table {{
      width: 100%;
      border-collapse: collapse;
      border: 1px solid var(--line);
      border-radius: 10px;
      overflow: hidden;
    }}
    th, td {{
      padding: 8px 10px;
      border-bottom: 1px solid var(--line);
      text-align: left;
    }}
    th {{
      background: var(--head);
      font-weight: 600;
      cursor: pointer;
      user-select: none;
      white-space: nowrap;
    }}
    th:hover {{ background: var(--chip); color: var(--accent); }}
    th .arrow {{ color: var(--accent); margin-left: 4px; }}
    td.num {{ text-align: right; font-variant-numeric: tabular-nums; }}
    tr:nth-child(even) td {{ background: #fafafa; }}
    caption {{ caption-side: bottom; text-align: left; color: var(--muted); padding-top: 8px; }}
  </style>
</head>
<body>
  <h1>Weak items review</h1>
  <p class="meta">{html.escape(title)} · click column headers to sort</p>
  <div class="kpis">
    <div class="kpi"><div class="label">Weak items</div><div class="value">{html.escape(str(item_count))} / {html.escape(str(total_menu))}</div></div>
    <div class="kpi"><div class="label">Item share</div><div class="value">{html.escape(item_share)}</div></div>
    <div class="kpi"><div class="label">Margin share</div><div class="value">{html.escape(margin_share)}</div></div>
    <div class="kpi"><div class="label">Weak margin</div><div class="value">{html.escape(margin_amount)}</div></div>
    <div class="kpi"><div class="label">Period</div><div class="value" style="font-size:1rem">{html.escape(period_label)}</div></div>
  </div>
  <p class="hint">Default sort: margin share ascending (v3). No action column.</p>
  <table id="weak-table">
    <thead><tr id="head-row"></tr></thead>
    <tbody id="body"></tbody>
    <caption>Data from get_weak_items_review · local file only</caption>
  </table>
  <script>
    const SHOW_AVG = {show_avg_js};
    const ROWS = {rows_json};
    let sortKey = "contribution_margin_share";
    let sortDir = "asc";

    const COLS = [
      {{ key: "menu", label: "Menu", type: "str", align: "left" }},
      ...(SHOW_AVG ? [{{ key: "avg_qty_per_day", label: "Avg qty/day", type: "num", align: "num" }}] : []),
      {{ key: "units_sold", label: "Units", type: "num", align: "num" }},
      {{ key: "contribution_margin", label: "Margin", type: "num", align: "num" }},
      {{ key: "contribution_margin_share", label: "Share", type: "share", align: "num" }},
    ];

    function fmtShare(v) {{
      const n = Number(v);
      return Number.isFinite(n) ? (n * 100).toFixed(1) + "%" : "—";
    }}
    function fmtNum(v, digits) {{
      if (v === null || v === undefined) return "—";
      const n = Number(v);
      if (!Number.isFinite(n)) return "—";
      return digits === 0 ? String(Math.round(n)) : n.toFixed(digits);
    }}
    function cellText(col, row) {{
      const v = row[col.key];
      if (col.type === "share") return fmtShare(v);
      if (col.key === "units_sold") return fmtNum(v, 0);
      if (col.key === "avg_qty_per_day") return fmtNum(v, 2);
      if (col.key === "contribution_margin") return fmtNum(v, 2);
      return v == null ? "—" : String(v);
    }}
    function cmp(a, b) {{
      const av = a[sortKey];
      const bv = b[sortKey];
      let diff = 0;
      if (sortKey === "menu") {{
        diff = String(av ?? "").localeCompare(String(bv ?? ""));
      }} else {{
        const an = av == null ? Number.NEGATIVE_INFINITY : Number(av);
        const bn = bv == null ? Number.NEGATIVE_INFINITY : Number(bv);
        diff = an - bn;
      }}
      if (diff === 0) diff = String(a.menu ?? "").localeCompare(String(b.menu ?? ""));
      return sortDir === "asc" ? diff : -diff;
    }}
    function render() {{
      const head = document.getElementById("head-row");
      head.innerHTML = "";
      for (const col of COLS) {{
        const th = document.createElement("th");
        th.textContent = col.label;
        if (col.key === sortKey) {{
          const arrow = document.createElement("span");
          arrow.className = "arrow";
          arrow.textContent = sortDir === "asc" ? "↑" : "↓";
          th.appendChild(arrow);
        }}
        th.addEventListener("click", () => {{
          if (sortKey === col.key) sortDir = sortDir === "asc" ? "desc" : "asc";
          else {{ sortKey = col.key; sortDir = col.key === "menu" ? "asc" : "asc"; }}
          render();
        }});
        head.appendChild(th);
      }}
      const body = document.getElementById("body");
      body.innerHTML = "";
      const sorted = [...ROWS].sort(cmp);
      if (sorted.length === 0) {{
        const tr = document.createElement("tr");
        const td = document.createElement("td");
        td.colSpan = COLS.length;
        td.textContent = "No weak items in this period.";
        td.style.textAlign = "center";
        td.style.color = "#64748b";
        td.style.padding = "24px";
        tr.appendChild(td);
        body.appendChild(tr);
        return;
      }}
      for (const row of sorted) {{
        const tr = document.createElement("tr");
        for (const col of COLS) {{
          const td = document.createElement("td");
          if (col.align === "num") td.className = "num";
          td.textContent = cellText(col, row);
          tr.appendChild(td);
        }}
        body.appendChild(tr);
      }}
    }}
    render();
  </script>
</body>
</html>
"""


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Render get_weak_items_review JSON as a sortable HTML table."
    )
    parser.add_argument(
        "--json",
        type=Path,
        help="Path to JSON from get_weak_items_review (default: stdin)",
    )
    parser.add_argument(
        "--out",
        type=Path,
        required=True,
        help="Output HTML path",
    )
    args = parser.parse_args()
    payload = load_payload(args.json)
    html_doc = build_html(payload)
    args.out.write_text(html_doc, encoding="utf-8")
    print(f"Wrote {args.out} ({len(as_items(payload))} weak items)")


if __name__ == "__main__":
    main()
