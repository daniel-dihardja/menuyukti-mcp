"""Owner-facing column mapping for custom / unrecognized POS Excel exports."""

from __future__ import annotations

import json
import os
import re
from io import BytesIO
from pathlib import Path
from typing import TypedDict

import pandas as pd

from menuyukti.core.analytics.utils import normalize_columns
from menuyukti.core.models.pos_transaction import POSTransactionLineItem

COLUMN_MAPPING_ENV = "MENUYUKTI_COLUMN_MAPPING"
DEFAULT_MAPPING_FILENAME = "pos-column-mapping.json"
DEFAULT_POS_NAME = "custom"

REQUIRED_FIELDS = POSTransactionLineItem.get_required_columns()


class ColumnMappingConfig(TypedDict):
    name: str
    skip_rows: int
    columns: dict[str, str]


class ColumnMappingError(ValueError):
    """Raised when a column-mapping config is missing, invalid, or incomplete."""


def _normalize_header(value: str) -> str:
    """Match spreadsheet headers the same way ``normalize_columns`` does."""
    text = str(value).strip().replace("\u00a0", " ").replace("\t", "")
    text = text.lower().replace(" ", "_")
    return text


def mapping_config_path(*, data_dir: Path) -> Path | None:
    """Resolve the mapping file path when one is configured or present."""
    raw = os.environ.get(COLUMN_MAPPING_ENV)
    if raw and raw.strip():
        return Path(raw).expanduser().resolve()
    candidate = (data_dir / DEFAULT_MAPPING_FILENAME).resolve()
    return candidate if candidate.is_file() else None


def load_column_mapping(path: Path) -> ColumnMappingConfig:
    """Load and validate an owner column-mapping JSON file."""
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ColumnMappingError(
            f"Invalid column-mapping JSON in {path}: {exc}"
        ) from exc
    if not isinstance(payload, dict):
        raise ColumnMappingError(
            f"Column-mapping config must be a JSON object: {path}"
        )

    columns_raw = payload.get("columns")
    if not isinstance(columns_raw, dict) or not columns_raw:
        raise ColumnMappingError(
            f"Column-mapping config needs a non-empty 'columns' object: {path}"
        )

    columns: dict[str, str] = {}
    for field, source in columns_raw.items():
        if not isinstance(field, str) or not isinstance(source, str):
            raise ColumnMappingError(
                "Column-mapping 'columns' must map normalized field names "
                f"to spreadsheet header strings: {path}"
            )
        field_key = field.strip()
        source_key = source.strip()
        if not field_key or not source_key:
            raise ColumnMappingError(
                f"Column-mapping entries cannot be empty: {path}"
            )
        columns[field_key] = source_key

    missing = [field for field in REQUIRED_FIELDS if field not in columns]
    if missing:
        raise ColumnMappingError(
            "Column-mapping is missing required fields: "
            f"{missing}. Required: {REQUIRED_FIELDS}"
        )

    unknown = sorted(set(columns) - set(REQUIRED_FIELDS))
    if unknown:
        raise ColumnMappingError(
            f"Column-mapping has unknown fields: {unknown}. "
            f"Allowed: {REQUIRED_FIELDS}"
        )

    skip_rows_raw = payload.get("skip_rows", 0)
    if not isinstance(skip_rows_raw, int) or isinstance(skip_rows_raw, bool):
        raise ColumnMappingError(
            f"Column-mapping 'skip_rows' must be an integer: {path}"
        )
    if skip_rows_raw < 0:
        raise ColumnMappingError(
            f"Column-mapping 'skip_rows' cannot be negative: {path}"
        )

    name_raw = payload.get("name", DEFAULT_POS_NAME)
    if not isinstance(name_raw, str) or not name_raw.strip():
        raise ColumnMappingError(
            f"Column-mapping 'name' must be a non-empty string when set: {path}"
        )
    name = _safe_pos_name(name_raw.strip())

    return ColumnMappingConfig(
        name=name,
        skip_rows=skip_rows_raw,
        columns=columns,
    )


def _safe_pos_name(name: str) -> str:
    slug = re.sub(r"[^a-z0-9_-]+", "-", name.lower()).strip("-")
    return slug or DEFAULT_POS_NAME


def normalize_mapped_excel(
    data: bytes, config: ColumnMappingConfig
) -> pd.DataFrame:
    """Read Excel bytes and map owner columns onto the normalized line-item schema."""
    source = pd.read_excel(BytesIO(data), skiprows=config["skip_rows"])
    source = normalize_columns(source)

    rename_map: dict[str, str] = {}
    missing_headers: list[str] = []
    for field, header in config["columns"].items():
        normalized_header = _normalize_header(header)
        if normalized_header not in source.columns:
            missing_headers.append(header)
            continue
        rename_map[normalized_header] = field

    if missing_headers:
        available = ", ".join(map(str, source.columns.tolist()))
        raise ColumnMappingError(
            "Mapped spreadsheet headers were not found after reading the export: "
            f"{missing_headers}. Available columns: [{available}]"
        )

    df = source.rename(columns=rename_map)
    required = REQUIRED_FIELDS
    missing_cols = [col for col in required if col not in df.columns]
    if missing_cols:
        raise ColumnMappingError(
            f"Mapped export is missing required columns after rename: {missing_cols}"
        )

    COL = POSTransactionLineItem
    frame = df[required].copy()
    frame[COL.QTY] = pd.to_numeric(frame[COL.QTY], errors="coerce")
    frame[COL.PRICE] = pd.to_numeric(frame[COL.PRICE], errors="coerce")
    frame[COL.TOTAL_AFTER_BILL_DISCOUNT] = pd.to_numeric(
        frame[COL.TOTAL_AFTER_BILL_DISCOUNT],
        errors="coerce",
    )
    frame[COL.ORDER_TIME] = pd.to_datetime(frame[COL.ORDER_TIME], errors="coerce")
    for text_col in (
        COL.BILL_NUMBER,
        COL.MENU,
        COL.MENU_CATEGORY,
        COL.MENU_CATEGORY_DETAIL,
    ):
        frame[text_col] = frame[text_col].astype("string").str.strip()

    invalid = frame.isna().any(axis=1) | frame[COL.MENU].eq("")
    cleaned = frame[~invalid].copy()
    if cleaned.empty:
        raise ColumnMappingError(
            "Mapped export produced no valid rows after type conversion."
        )
    return cleaned.reset_index(drop=True)


def try_load_mapping_for_data_dir(data_dir: Path) -> ColumnMappingConfig | None:
    """Return a validated mapping config when a mapping file is available."""
    path = mapping_config_path(data_dir=data_dir)
    if path is None:
        return None
    if not path.is_file():
        raise ColumnMappingError(
            f"Column-mapping file from {COLUMN_MAPPING_ENV} was not found: {path}"
        )
    return load_column_mapping(path)
