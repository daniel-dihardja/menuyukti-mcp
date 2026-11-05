#!/usr/bin/env python3
"""Compute Menuyukti venue slot strength from a POS Excel/CSV + column mapping.

Matches crates/menuyukti/src/analytics/slot_demand.rs.
"""

from __future__ import annotations

import argparse
import csv
import json
import re
import sys
from collections import defaultdict
from datetime import datetime, timedelta
from pathlib import Path
from typing import Any

REQUIRED_COLUMNS = (
    "bill_number",
    "menu",
    "qty",
    "price",
    "total_after_bill_discount",
    "order_time",
    "menu_category",
    "menu_category_detail",
)

WEEKDAY_ORDER = ("mon", "tue", "wed", "thu", "fri", "sat", "sun")
WEEKDAY_ABBR = ("mon", "tue", "wed", "thu", "fri", "sat", "sun")

MEAL_PERIODS: list[tuple[str, str, str, frozenset[int]]] = [
    ("breakfast", "Breakfast", "05:00–10:59", frozenset({5, 6, 7, 8, 9, 10})),
    ("lunch", "Lunch", "11:00–14:59", frozenset({11, 12, 13, 14})),
    ("afternoon", "Afternoon", "15:00–16:59", frozenset({15, 16})),
    ("dinner", "Dinner", "17:00–21:59", frozenset({17, 18, 19, 20, 21})),
    ("late_night", "Late Night", "22:00–04:59", frozenset({22, 23, 0, 1, 2, 3, 4})),
]

LOW_DEMAND_THRESHOLD = 0.9
HIGH_DEMAND_THRESHOLD = 1.1
SLOT_COUNT = len(WEEKDAY_ORDER) * len(MEAL_PERIODS)
MEAN_SLOT_TRAFFIC_SHARE = 1.0 / SLOT_COUNT

EXCEL_EPOCH = datetime(1899, 12, 30)


def normalize_header(value: str) -> str:
    return (
        value.strip()
        .replace("\u00a0", " ")
        .replace("\t", "")
        .lower()
        .replace(" ", "_")
    )


def round4(value: float) -> float:
    return round(value * 10_000.0) / 10_000.0


def meal_period_for_hour(hour: int) -> tuple[str, str, str]:
    for period, label, hours_label, hours in MEAL_PERIODS:
        if hour in hours:
            return period, label, hours_label
    period, label, hours_label, _ = MEAL_PERIODS[4]
    return period, label, hours_label


def relative_demand(demand_index: float) -> str:
    if demand_index < LOW_DEMAND_THRESHOLD:
        return "low"
    if demand_index > HIGH_DEMAND_THRESHOLD:
        return "high"
    return "average"


def load_mapping(path: Path) -> dict[str, Any]:
    config = json.loads(path.read_text(encoding="utf-8"))
    columns = config.get("columns") or {}
    cleaned: dict[str, str] = {}
    for field, source in columns.items():
        field_key = str(field).strip()
        source_key = str(source).strip()
        if not field_key or not source_key:
            raise SystemExit(f"Empty mapping entry in {path}")
        if "<YOUR_" in source_key:
            raise SystemExit(
                f"Mapping still has placeholder '{source_key}'. "
                "Replace every <YOUR_…_COLUMN> with a real header."
            )
        cleaned[field_key] = source_key

    missing = [f for f in REQUIRED_COLUMNS if f not in cleaned]
    if missing:
        raise SystemExit(f"Mapping missing required fields: {missing}")
    unknown = [k for k in cleaned if k not in REQUIRED_COLUMNS]
    if unknown:
        raise SystemExit(f"Mapping has unknown fields: {unknown}")

    name = str(config.get("name") or "custom").strip().lower()
    name = re.sub(r"[^a-z0-9_-]+", "-", name).strip("-") or "custom"
    return {
        "name": name,
        "skip_rows": int(config.get("skip_rows") or 0),
        "columns": cleaned,
    }


def parse_datetime(value: Any) -> datetime | None:
    if value is None or value == "":
        return None
    if isinstance(value, datetime):
        return value.replace(tzinfo=None)
    if isinstance(value, (int, float)):
        # Excel serial date
        try:
            return EXCEL_EPOCH + timedelta(days=float(value))
        except (OverflowError, ValueError):
            return None
    text = str(value).strip()
    if not text:
        return None
    for fmt in (
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M",
        "%d/%m/%Y %H:%M:%S",
        "%d/%m/%Y %H:%M",
        "%m/%d/%Y %H:%M:%S",
        "%m/%d/%Y %H:%M",
        "%d-%m-%Y %H:%M:%S",
        "%d-%m-%Y %H:%M",
    ):
        try:
            return datetime.strptime(text, fmt)
        except ValueError:
            continue
    try:
        return datetime.fromisoformat(text.replace("Z", "+00:00")).replace(tzinfo=None)
    except ValueError:
        return None


def cell_str(value: Any) -> str:
    if value is None:
        return ""
    if isinstance(value, float) and value.is_integer():
        return str(int(value))
    return str(value).strip()


def read_table_excel(path: Path, skip_rows: int) -> tuple[list[str], list[dict[str, Any]]]:
    try:
        from openpyxl import load_workbook
    except ImportError as exc:
        raise SystemExit(
            "openpyxl is required for Excel files. Install with: pip install openpyxl"
        ) from exc

    wb = load_workbook(path, read_only=True, data_only=True)
    ws = wb.active
    rows_iter = ws.iter_rows(values_only=True)
    for _ in range(skip_rows):
        next(rows_iter, None)
    header_row = next(rows_iter, None)
    if header_row is None:
        raise SystemExit("Excel has no header row after skip_rows")
    headers = [normalize_header(cell_str(c)) for c in header_row]
    out: list[dict[str, Any]] = []
    for row in rows_iter:
        item: dict[str, Any] = {}
        empty = True
        for idx, header in enumerate(headers):
            if not header:
                continue
            value = row[idx] if idx < len(row) else None
            if value is not None and value != "":
                empty = False
            item[header] = value
        if not empty:
            out.append(item)
    wb.close()
    return headers, out


def read_table_csv(path: Path, skip_rows: int) -> tuple[list[str], list[dict[str, Any]]]:
    with path.open(newline="", encoding="utf-8-sig") as fh:
        reader = csv.reader(fh)
        for _ in range(skip_rows):
            next(reader, None)
        header_row = next(reader, None)
        if header_row is None:
            raise SystemExit("CSV has no header row after skip_rows")
        headers = [normalize_header(cell_str(c)) for c in header_row]
        out: list[dict[str, Any]] = []
        for row in reader:
            item: dict[str, Any] = {}
            empty = True
            for idx, header in enumerate(headers):
                if not header:
                    continue
                value = row[idx] if idx < len(row) else ""
                if value != "":
                    empty = False
                item[header] = value
            if not empty:
                out.append(item)
    return headers, out


def read_table(path: Path, skip_rows: int) -> tuple[list[str], list[dict[str, Any]]]:
    suffix = path.suffix.lower()
    if suffix in {".xlsx", ".xlsm", ".xltx", ".xltm"}:
        return read_table_excel(path, skip_rows)
    if suffix == ".csv":
        return read_table_csv(path, skip_rows)
    raise SystemExit(f"Unsupported file type: {suffix} (use .xlsx or .csv)")


def extract_bill_times(
    rows: list[dict[str, Any]],
    mapping: dict[str, Any],
    headers: list[str],
) -> dict[str, datetime]:
    rename: dict[str, str] = {}
    missing_headers: list[str] = []
    for field, header in mapping["columns"].items():
        normalized = normalize_header(header)
        if normalized not in headers:
            missing_headers.append(header)
            continue
        rename[normalized] = field
    if missing_headers:
        raise SystemExit(
            f"Mapped headers not found: {missing_headers}. Available: {headers}"
        )

    bill_min: dict[str, datetime] = {}
    for row in rows:
        remapped = {
            rename[h]: v for h, v in row.items() if h in rename and v is not None and v != ""
        }
        bill = cell_str(remapped.get("bill_number"))
        if not bill:
            continue
        dt = parse_datetime(remapped.get("order_time"))
        if dt is None:
            continue
        prev = bill_min.get(bill)
        if prev is None or dt < prev:
            bill_min[bill] = dt
    return bill_min


def compute_slot_demand(bill_min: dict[str, datetime]) -> list[dict[str, Any]]:
    total_orders = len(bill_min)
    if total_orders == 0:
        return []

    counts: dict[tuple[str, str], int] = defaultdict(int)
    for dt in bill_min.values():
        day = WEEKDAY_ABBR[dt.weekday()]
        period, _, _ = meal_period_for_hour(dt.hour)
        counts[(day, period)] += 1

    cells: list[dict[str, Any]] = []
    for day in WEEKDAY_ORDER:
        for period, label, hours_label, _ in MEAL_PERIODS:
            order_count = counts.get((day, period), 0)
            traffic_share = order_count / total_orders
            demand_index = (
                traffic_share / MEAN_SLOT_TRAFFIC_SHARE if MEAN_SLOT_TRAFFIC_SHARE else 0.0
            )
            cells.append(
                {
                    "day": day,
                    "meal_period": period,
                    "meal_period_label": label,
                    "meal_period_hours_label": hours_label,
                    "order_count": order_count,
                    "traffic_share": round4(traffic_share),
                    "demand_index": round4(demand_index),
                    "relative_demand": relative_demand(demand_index),
                }
            )
    return cells


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Compute Menuyukti venue slot strength from POS Excel/CSV"
    )
    parser.add_argument("--excel", required=True, type=Path, help="Sales .xlsx or .csv path")
    parser.add_argument("--mapping", required=True, type=Path, help="pos-column-mapping.json")
    parser.add_argument(
        "--skip-rows",
        type=int,
        default=None,
        help="Override mapping skip_rows (rows before header)",
    )
    args = parser.parse_args()

    if not args.excel.is_file():
        raise SystemExit(f"Sales file not found: {args.excel}")
    if not args.mapping.is_file():
        raise SystemExit(f"Mapping file not found: {args.mapping}")

    mapping = load_mapping(args.mapping)
    skip_rows = mapping["skip_rows"] if args.skip_rows is None else args.skip_rows
    headers, rows = read_table(args.excel, skip_rows)
    bill_min = extract_bill_times(rows, mapping, headers)
    profile = compute_slot_demand(bill_min)

    payload = {
        "pos": mapping["name"],
        "total_orders": len(bill_min),
        "slot_demand_profile": profile,
    }
    json.dump(payload, sys.stdout, indent=2)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
