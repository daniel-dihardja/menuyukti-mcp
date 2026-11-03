use std::collections::HashMap;

use calamine::Data;

use super::NormalizeError;
use super::category::{harmonize_categories, normalize_category};
use crate::excel::{
    cell_as_datetime, cell_as_f64, cell_as_i64, cell_as_string, get_cell, read_table,
};
use crate::line_item::LineItem;

const MOKA_SOURCE_COLUMNS: &[&str] = &[
    "payment_no",
    "item_name",
    "quantity",
    "price",
    "net_sales",
    "created_at",
    "category",
    "variant_name",
];

pub fn normalize_moka_excel(data: &[u8], skip_rows: usize) -> Result<Vec<LineItem>, NormalizeError> {
    let (headers, rows) = read_table(data, skip_rows)?;
    for col in MOKA_SOURCE_COLUMNS {
        if !headers.iter().any(|h| h == col) {
            return Err(NormalizeError::Message(
                "Missing required MOKA columns for Export Item Details: \
                 Payment No, Item Name, Quantity, Price, Net Sales, Created At, \
                 Category, Variant Name. \
                 Expected Moka Reports → Transactions → Export Item Details \
                 (title row + header on row 4)."
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
    let bill_number = cell_as_string(get_cell(row, "payment_no")?)?.trim().to_string();
    let menu = cell_as_string(get_cell(row, "item_name")?)?.trim().to_string();
    if bill_number.is_empty() || menu.is_empty() {
        return None;
    }
    let qty = cell_as_i64(get_cell(row, "quantity")?)?;
    let price = cell_as_f64(get_cell(row, "price")?)?;
    let total = cell_as_f64(get_cell(row, "net_sales")?)?;
    if qty <= 0 || total <= 0.0 {
        return None;
    }
    let order_time = cell_as_datetime(get_cell(row, "created_at")?)?;
    let menu_category = normalize_category(
        get_cell(row, "category").and_then(cell_as_string).as_deref(),
        "OTHER",
    );
    let menu_category_detail = normalize_category(
        get_cell(row, "variant_name")
            .and_then(cell_as_string)
            .as_deref(),
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
