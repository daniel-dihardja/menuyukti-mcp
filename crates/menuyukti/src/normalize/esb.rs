use calamine::Data;

use super::NormalizeError;
use crate::excel::{
    cell_as_datetime, cell_as_f64, cell_as_i64, cell_as_string, get_cell, read_table,
};
use crate::line_item::{LineItem, REQUIRED_COLUMNS};

pub fn normalize_esb_excel(data: &[u8], skip_rows: usize) -> Result<Vec<LineItem>, NormalizeError> {
    let (headers, rows) = read_table(data, skip_rows)?;
    for col in REQUIRED_COLUMNS {
        if !headers.iter().any(|h| h == col) {
            return Err(NormalizeError::Message(format!(
                "Missing required columns: [{col}]"
            )));
        }
    }

    let mut out = Vec::new();
    for row in rows {
        let Some(item) = try_line_item(&row) else {
            continue;
        };
        out.push(item);
    }
    Ok(out)
}

fn try_line_item(row: &std::collections::HashMap<String, Data>) -> Option<LineItem> {
    let bill_number = cell_as_string(get_cell(row, "bill_number")?)?.trim().to_string();
    let menu = cell_as_string(get_cell(row, "menu")?)?.trim().to_string();
    if bill_number.is_empty() || menu.is_empty() {
        return None;
    }
    let qty = cell_as_i64(get_cell(row, "qty")?)?;
    let price = cell_as_f64(get_cell(row, "price")?)?;
    let total = cell_as_f64(get_cell(row, "total_after_bill_discount")?)?;
    let order_time = cell_as_datetime(get_cell(row, "order_time")?)?;
    let menu_category = cell_as_string(get_cell(row, "menu_category")?)?
        .trim()
        .to_string();
    let menu_category_detail = cell_as_string(get_cell(row, "menu_category_detail")?)?
        .trim()
        .to_string();
    if qty <= 0 || total <= 0.0 || menu_category.is_empty() || menu_category_detail.is_empty() {
        // ESB python only drops NaN conversions; keep parity loose for totals usefulness.
        // Still require parseable required fields.
    }
    // Match python: any NaN after coerce drops the row; empty strings already handled.
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
