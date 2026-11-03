//! Canonical POS transaction line item.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Required column names on normalized frames / row maps.
pub const REQUIRED_COLUMNS: &[&str] = &[
    "bill_number",
    "menu",
    "qty",
    "price",
    "total_after_bill_discount",
    "order_time",
    "menu_category",
    "menu_category_detail",
];

/// One cleaned POS line item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    pub bill_number: String,
    pub menu: String,
    pub qty: i64,
    pub price: f64,
    pub total_after_bill_discount: f64,
    pub order_time: NaiveDateTime,
    pub menu_category: String,
    pub menu_category_detail: String,
}
