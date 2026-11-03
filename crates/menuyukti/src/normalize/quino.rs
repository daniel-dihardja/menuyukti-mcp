use std::collections::HashMap;

use calamine::Data;

use super::NormalizeError;
use super::category::{harmonize_categories, normalize_category};
use crate::excel::{
    cell_as_datetime, cell_as_f64, cell_as_i64, cell_as_string, get_cell, read_table,
};
use crate::line_item::LineItem;

const QUINO_SOURCE_COLUMNS: &[&str] = &[
    "inv_no.",
    "name",
    "qty",
    "subtotal",
    "discount",
    "order_time",
    "department",
    "category",
];

pub fn normalize_quino_excel(
    data: &[u8],
    skip_rows: usize,
) -> Result<Vec<LineItem>, NormalizeError> {
    let (headers, rows) = read_table(data, skip_rows)?;
    for col in QUINO_SOURCE_COLUMNS {
        if !headers.iter().any(|h| h == col) {
            return Err(NormalizeError::Message(
                "Missing required QUINO columns for Transaction Item Detail Report: \
                 Inv No., Name, Qty, Subtotal, Discount, Order Time, Department, Category. \
                 Expected Quino Transaction Item Detail Report export (title row + header on row 4)."
                    .into(),
            ));
        }
    }

    let mut candidates = Vec::new();
    for row in rows {
        let Some(item) = try_line_item(&row) else {
            continue;
        };
        candidates.push(item);
    }
    Ok(harmonize_categories(candidates))
}

fn try_line_item(row: &HashMap<String, Data>) -> Option<LineItem> {
    let bill_number = cell_as_string(get_cell(row, "inv_no.")?)?.trim().to_string();
    let menu = cell_as_string(get_cell(row, "name")?)?.trim().to_string();
    if bill_number.is_empty() || menu.is_empty() {
        return None;
    }
    let menu_upper = menu.to_uppercase();
    if menu_upper.starts_with("TOTAL ") || menu_upper == "GRAND TTL" {
        return None;
    }
    let qty = cell_as_i64(get_cell(row, "qty")?)?;
    let subtotal = cell_as_f64(get_cell(row, "subtotal")?)?;
    let discount = get_cell(row, "discount")
        .and_then(cell_as_f64)
        .unwrap_or(0.0);
    let total = subtotal - discount;
    if qty <= 0 || total <= 0.0 {
        return None;
    }
    let price = total / qty as f64;
    let order_time = cell_as_datetime(get_cell(row, "order_time")?)?;
    let menu_category = normalize_category(
        get_cell(row, "department")
            .and_then(cell_as_string)
            .as_deref(),
        "OTHER",
    );
    let menu_category_detail = normalize_category(
        get_cell(row, "category").and_then(cell_as_string).as_deref(),
        "UNCATEGORIZED",
    );
    Some(LineItem {
        bill_number,
        menu,
        qty,
        price,
        total_after_bill_discount: total,
        order_time,
        menu_category,
        menu_category_detail,
    })
}
