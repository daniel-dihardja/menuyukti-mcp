use std::collections::HashMap;

use serde::Serialize;
use serde_json::{Value, json};

use crate::line_item::LineItem;

pub const TOP_ITEMS_LIMIT: usize = 10;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TopItem {
    pub menu: String,
    pub popularity: f64,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SalesSummary {
    pub pos: String,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub total_revenue: f64,
    pub total_items_sold: i64,
    pub top_items: Vec<TopItem>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct NormalizedSalesList {
    pub pos: String,
    pub rows: Vec<Value>,
    pub row_count: usize,
}

pub fn summarize_sales(pos: impl Into<String>, rows: &[LineItem]) -> Result<SalesSummary, String> {
    if rows.is_empty() {
        return Err("DataFrame is empty. Cannot calculate analytics.".into());
    }
    let total_items_sold: i64 = rows.iter().map(|r| r.qty).sum();
    if total_items_sold == 0 {
        return Err("Total quantity is zero. Cannot calculate analytics.".into());
    }
    let total_revenue: f64 = rows.iter().map(|r| r.total_after_bill_discount).sum();
    let top_items = popularity_index(rows)?
        .into_iter()
        .take(TOP_ITEMS_LIMIT)
        .collect();

    let mut period_start = None;
    let mut period_end = None;
    if let (Some(min), Some(max)) = (
        rows.iter().map(|r| r.order_time.date()).min(),
        rows.iter().map(|r| r.order_time.date()).max(),
    ) {
        // Quino-style sentinel: single 1970-01-01 means no real datetime.
        let all_epoch = rows
            .iter()
            .all(|r| r.order_time == chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap());
        if !all_epoch {
            period_start = Some(min.format("%Y-%m-%d").to_string());
            period_end = Some(max.format("%Y-%m-%d").to_string());
        }
    }

    Ok(SalesSummary {
        pos: pos.into(),
        period_start,
        period_end,
        total_revenue,
        total_items_sold,
        top_items,
    })
}

pub fn list_normalized_rows(pos: impl Into<String>, rows: &[LineItem]) -> NormalizedSalesList {
    let serialized: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "bill_number": r.bill_number,
                "menu": r.menu,
                "qty": r.qty,
                "price": r.price,
                "total_after_bill_discount": r.total_after_bill_discount,
                "order_time": r.order_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
                "menu_category": r.menu_category,
                "menu_category_detail": r.menu_category_detail,
            })
        })
        .collect();
    NormalizedSalesList {
        pos: pos.into(),
        row_count: serialized.len(),
        rows: serialized,
    }
}

fn popularity_index(rows: &[LineItem]) -> Result<Vec<TopItem>, String> {
    let total_qty: i64 = rows.iter().map(|r| r.qty).sum();
    if total_qty == 0 {
        return Err("Total quantity is zero. Cannot calculate popularity index.".into());
    }
    let mut by_menu: HashMap<String, i64> = HashMap::new();
    for row in rows {
        *by_menu.entry(row.menu.clone()).or_default() += row.qty;
    }
    let mut items: Vec<TopItem> = by_menu
        .into_iter()
        .map(|(menu, quantity)| TopItem {
            popularity: ((quantity as f64 / total_qty as f64) * 1_000_000.0).round() / 1_000_000.0,
            menu,
            quantity,
        })
        .collect();
    items.sort_by(|a, b| {
        b.quantity
            .cmp(&a.quantity)
            .then_with(|| a.menu.cmp(&b.menu))
    });
    Ok(items)
}
