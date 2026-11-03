"""Tests for owner column-mapping config and custom POS Excel loading."""

from __future__ import annotations

import json
from pathlib import Path

import pandas as pd
import pytest

from menuyukti.core.models.pos_transaction import POSTransactionLineItem
from menuyukti_gpt.column_mapping import (
    ColumnMappingError,
    load_column_mapping,
    normalize_mapped_excel,
)
from menuyukti_gpt.sales import DATA_DIR_ENV, load_normalized_sales


def _write_mapping(path: Path, **overrides: object) -> Path:
    payload: dict[str, object] = {
        "name": "cafe-pos",
        "skip_rows": 0,
        "columns": {
            "bill_number": "Order ID",
            "menu": "Item Name",
            "qty": "Qty",
            "price": "Unit Price",
            "total_after_bill_discount": "Net Sales",
            "order_time": "Sold At",
            "menu_category": "Category",
            "menu_category_detail": "Subcategory",
        },
    }
    payload.update(overrides)
    path.write_text(json.dumps(payload), encoding="utf-8")
    return path


def _write_custom_excel(path: Path) -> Path:
    frame = pd.DataFrame(
        [
            {
                "Order ID": "B-1",
                "Item Name": "Nasi Goreng",
                "Qty": 2,
                "Unit Price": 45000,
                "Net Sales": 90000,
                "Sold At": "2026-07-01 12:30:00",
                "Category": "Mains",
                "Subcategory": "Rice",
            },
            {
                "Order ID": "B-2",
                "Item Name": "Es Teh",
                "Qty": 1,
                "Unit Price": 8000,
                "Net Sales": 8000,
                "Sold At": "2026-07-01 13:00:00",
                "Category": "Drinks",
                "Subcategory": "Cold",
            },
        ]
    )
    frame.to_excel(path, index=False)
    return path


def test_load_column_mapping_requires_all_fields(tmp_path: Path) -> None:
    path = _write_mapping(
        tmp_path / "pos-column-mapping.json",
        columns={
            "bill_number": "Order ID",
            "menu": "Item Name",
        },
    )
    with pytest.raises(ColumnMappingError, match="missing required fields"):
        load_column_mapping(path)


def test_normalize_mapped_excel_and_load_normalized_sales(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    data_dir = tmp_path / "sales"
    data_dir.mkdir()
    mapping_path = _write_mapping(data_dir / "pos-column-mapping.json")
    excel_path = _write_custom_excel(data_dir / "custom-export.xlsx")

    config = load_column_mapping(mapping_path)
    frame = normalize_mapped_excel(excel_path.read_bytes(), config)
    assert list(frame.columns) == POSTransactionLineItem.get_required_columns()
    assert len(frame) == 2
    assert frame.iloc[0]["menu"] == "Nasi Goreng"

    monkeypatch.setenv(DATA_DIR_ENV, str(data_dir))
    loaded = load_normalized_sales("custom-export.xlsx", data_dir=data_dir.resolve())
    assert loaded["pos"] == "cafe-pos"
    assert len(loaded["frame"]) == 2
    assert "bill_number" in loaded["frame"].columns


def test_load_normalized_sales_errors_without_mapping_for_unknown_pos(
    tmp_path: Path,
) -> None:
    data_dir = tmp_path / "sales"
    data_dir.mkdir()
    excel_path = _write_custom_excel(data_dir / "custom-export.xlsx")

    with pytest.raises(ValueError, match="pos-column-mapping.json"):
        load_normalized_sales(str(excel_path.name), data_dir=data_dir.resolve())


def test_normalize_mapped_excel_missing_header(tmp_path: Path) -> None:
    mapping_path = _write_mapping(tmp_path / "pos-column-mapping.json")
    config = load_column_mapping(mapping_path)
    bad = pd.DataFrame([{"Order ID": "B-1", "Item Name": "Tea"}])
    excel = tmp_path / "bad.xlsx"
    bad.to_excel(excel, index=False)

    with pytest.raises(ColumnMappingError, match="headers were not found"):
        normalize_mapped_excel(excel.read_bytes(), config)
