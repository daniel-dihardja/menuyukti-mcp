"""Local POS sales loading and summarization for the Menuyukti GPT MCP server."""

from __future__ import annotations

import json
import os
from pathlib import Path
from typing import Any, TypedDict

import pandas as pd

from menuyukti.core.analytics import (
    calculate_sales_analytics,
    detect_pos_from_excel_bytes,
    get_normalizer,
)
from menuyukti.core.models.pos_mapping import get_config

DATA_DIR_ENV = "MENUYUKTI_DATA_DIR"
TOP_ITEMS_LIMIT = 10


class TopItem(TypedDict):
    menu: str
    popularity: float
    quantity: int


class SalesSummary(TypedDict):
    pos: str
    period_start: str | None
    period_end: str | None
    total_revenue: float
    total_items_sold: int
    top_items: list[TopItem]


class NormalizedSales(TypedDict):
    """Gated POS Excel load: detected system and normalized line-item frame."""

    pos: str
    frame: pd.DataFrame


class NormalizedSalesList(TypedDict):
    """Detected POS and plain normalized line-item rows."""

    pos: str
    rows: list[dict[str, Any]]
    row_count: int


class PathNotAllowedError(ValueError):
    """Raised when a sales path is outside the configured data directory."""


def get_data_dir() -> Path:
    raw = os.environ.get(DATA_DIR_ENV)
    if not raw or not raw.strip():
        raise PathNotAllowedError(
            f"{DATA_DIR_ENV} is not set. Set it to the directory that holds POS Excel files."
        )
    return Path(raw).expanduser().resolve()


def resolve_allowed_path(path: str, *, data_dir: Path | None = None) -> Path:
    """Resolve ``path`` and require it to stay inside the configured data directory.

    Does not open the file. Rejects missing env, traversal, and paths outside the
    data directory before any read.
    """
    root = data_dir if data_dir is not None else get_data_dir()
    candidate = Path(path).expanduser()
    if not candidate.is_absolute():
        candidate = root / candidate
    resolved = candidate.resolve()
    try:
        resolved.relative_to(root)
    except ValueError as exc:
        raise PathNotAllowedError(
            f"Path is outside {DATA_DIR_ENV} ({root}): {path}"
        ) from exc
    if not resolved.is_file():
        raise FileNotFoundError(f"Sales file not found: {resolved}")
    return resolved


def load_normalized_sales(
    path: str, *, data_dir: Path | None = None
) -> NormalizedSales:
    """Gate ``path``, detect POS, and return a normalized line-item DataFrame.

    Reads the file only after ``resolve_allowed_path`` succeeds. Uses the
    menuyukti POS registry (detect → normalizer → optional column rename).
    """
    allowed = resolve_allowed_path(path, data_dir=data_dir)
    payload = allowed.read_bytes()

    pos = detect_pos_from_excel_bytes(payload) or "unknown"
    normalizer = get_normalizer(pos)
    if normalizer is None:
        raise ValueError(f"Unsupported POS system detected: {pos}")

    skip_rows, rename_map = get_config(pos)
    df = normalizer(payload, skiprows=skip_rows)
    if rename_map:
        df = df.rename(columns=rename_map)

    return NormalizedSales(pos=pos, frame=df)


def _top_items(popularity_index: list[dict[str, object]]) -> list[TopItem]:
    items: list[TopItem] = []
    for row in popularity_index[:TOP_ITEMS_LIMIT]:
        items.append(
            TopItem(
                menu=str(row["menu"]),
                popularity=float(row["popularity"]),  # type: ignore[arg-type]
                quantity=int(row["quantity"]),  # type: ignore[arg-type]
            )
        )
    return items


def summarize_local_sales(path: str, *, data_dir: Path | None = None) -> SalesSummary:
    """Load a local POS Excel export and return a capped aggregate summary."""
    loaded = load_normalized_sales(path, data_dir=data_dir)
    analytics = calculate_sales_analytics(loaded["frame"])
    fundamentals = analytics["fundamental_signals"]
    datetime_signals = analytics["additional_signals"]["datetime_signals"]
    period_start = datetime_signals["period_start"] if datetime_signals else None
    period_end = datetime_signals["period_end"] if datetime_signals else None

    return SalesSummary(
        pos=loaded["pos"],
        period_start=period_start,
        period_end=period_end,
        total_revenue=float(fundamentals["total_revenue"]),
        total_items_sold=int(fundamentals["total_items_sold"]),
        top_items=_top_items(list(fundamentals["popularity_index"])),
    )


def _frame_to_plain_rows(frame: pd.DataFrame) -> list[dict[str, Any]]:
    """Serialize a normalized frame to JSON-friendly row dicts."""
    if frame.empty:
        return []
    return json.loads(frame.to_json(orient="records", date_format="iso"))


def list_normalized_sales(
    path: str, *, data_dir: Path | None = None
) -> NormalizedSalesList:
    """Load a local POS Excel export and return plain normalized line items."""
    loaded = load_normalized_sales(path, data_dir=data_dir)
    rows = _frame_to_plain_rows(loaded["frame"])
    return NormalizedSalesList(
        pos=loaded["pos"],
        rows=rows,
        row_count=len(rows),
    )


def summary_as_dict(summary: SalesSummary) -> dict[str, Any]:
    """Plain dict for MCP structured content / JSON responses."""
    return {
        "pos": summary["pos"],
        "period_start": summary["period_start"],
        "period_end": summary["period_end"],
        "total_revenue": summary["total_revenue"],
        "total_items_sold": summary["total_items_sold"],
        "top_items": [
            {
                "menu": item["menu"],
                "popularity": item["popularity"],
                "quantity": item["quantity"],
            }
            for item in summary["top_items"]
        ],
    }


def normalized_list_as_dict(listed: NormalizedSalesList) -> dict[str, Any]:
    """Plain dict for MCP structured content / JSON responses."""
    return {
        "pos": listed["pos"],
        "row_count": listed["row_count"],
        "rows": listed["rows"],
    }
