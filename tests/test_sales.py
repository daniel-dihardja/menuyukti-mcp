"""Unit tests for local sales path gate, normalization, and aggregate summary."""

from __future__ import annotations

from pathlib import Path
from unittest.mock import patch

import pytest

from menuyukti.core.models.pos_transaction import POSTransactionLineItem
from menuyukti_gpt.sales import (
    DATA_DIR_ENV,
    PathNotAllowedError,
    list_normalized_sales,
    load_normalized_sales,
    summarize_local_sales,
)

FIXTURES = (
    Path(__file__).resolve().parents[1]
    / "packages"
    / "menuyukti"
    / "fixtures"
    / "pos-sales-reports"
)
ESB_DEMO = FIXTURES / "esb" / "SalesRecapitulationDetailReport_DEMO.xlsx"
ESB_REL = "esb/SalesRecapitulationDetailReport_DEMO.xlsx"


@pytest.fixture
def data_dir(monkeypatch: pytest.MonkeyPatch) -> Path:
    monkeypatch.setenv(DATA_DIR_ENV, str(FIXTURES))
    return FIXTURES.resolve()


def test_rejects_path_outside_data_dir_without_reading(
    data_dir: Path, tmp_path: Path
) -> None:
    outside = tmp_path / "secret.xlsx"
    outside.write_bytes(b"not-excel")

    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match="outside"):
            load_normalized_sales(str(outside), data_dir=data_dir)
        read_bytes.assert_not_called()


def test_rejects_traversal_without_reading(data_dir: Path) -> None:
    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match="outside"):
            load_normalized_sales("../README.md", data_dir=data_dir)
        read_bytes.assert_not_called()


def test_requires_data_dir_env(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.delenv(DATA_DIR_ENV, raising=False)
    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match=DATA_DIR_ENV):
            load_normalized_sales(ESB_REL)
        read_bytes.assert_not_called()


def test_esb_fixture_load_normalized_sales(data_dir: Path) -> None:
    assert ESB_DEMO.is_file()
    loaded = load_normalized_sales(ESB_REL, data_dir=data_dir)

    assert loaded["pos"] == "esb"
    frame = loaded["frame"]
    assert not frame.empty
    for column in POSTransactionLineItem.get_required_columns():
        assert column in frame.columns


def test_esb_fixture_summary_shape(data_dir: Path) -> None:
    assert ESB_DEMO.is_file()
    summary = summarize_local_sales(ESB_REL, data_dir=data_dir)

    assert summary["pos"] == "esb"
    assert summary["period_start"] is not None
    assert summary["period_end"] is not None
    assert summary["total_revenue"] > 0
    assert summary["total_items_sold"] > 0
    assert 1 <= len(summary["top_items"]) <= 10

    allowed_keys = {
        "pos",
        "period_start",
        "period_end",
        "total_revenue",
        "total_items_sold",
        "top_items",
    }
    assert set(summary.keys()) == allowed_keys

    for item in summary["top_items"]:
        assert set(item.keys()) == {"menu", "popularity", "quantity"}
        assert "bill_number" not in item
        assert "order_time" not in item

    # Ensure raw ticket fields did not leak into the summary.
    blob = str(summary)
    assert "bill_number" not in blob
    assert "total_after_bill_discount" not in blob


def test_esb_fixture_list_normalized_sales(data_dir: Path) -> None:
    assert ESB_DEMO.is_file()
    listed = list_normalized_sales(ESB_REL, data_dir=data_dir)

    assert listed["pos"] == "esb"
    assert listed["row_count"] == len(listed["rows"])
    assert listed["row_count"] > 0
    assert set(listed.keys()) == {"pos", "rows", "row_count"}

    required = set(POSTransactionLineItem.get_required_columns())
    first = listed["rows"][0]
    assert required.issubset(first.keys())
    assert first["bill_number"]
    assert first["menu"]
    assert first["qty"] > 0
    assert isinstance(first["order_time"], str)


def test_list_normalized_sales_rejects_path_outside_data_dir(
    data_dir: Path, tmp_path: Path
) -> None:
    outside = tmp_path / "secret.xlsx"
    outside.write_bytes(b"not-excel")

    with patch("pathlib.Path.read_bytes") as read_bytes:
        with pytest.raises(PathNotAllowedError, match="outside"):
            list_normalized_sales(str(outside), data_dir=data_dir)
        read_bytes.assert_not_called()
