use std::collections::HashMap;

use chrono::{Datelike, Timelike};
use serde::Serialize;

use crate::line_item::LineItem;

pub const WEEKDAY_ORDER: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DailyHeatmapRow {
    pub hour: u32,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WeeklyHeatmapRow {
    pub day: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MenuHeatmapPayload {
    pub menu: String,
    pub menu_category: Option<String>,
    pub menu_category_detail: Option<String>,
    /// Menu-engineering quadrant (`star` / `plow_horse` / `puzzle` / `weak`) when COGS enrichment ran.
    pub matrix_category: Option<String>,
    pub daily_heatmap: Vec<DailyHeatmapRow>,
    pub weekly_heatmap: Vec<WeeklyHeatmapRow>,
    pub reporting_period: String,
}

/// Attach menu-engineering categories onto heatmap rows by menu name.
///
/// Menus missing from `categories_by_menu` keep `matrix_category = None`.
pub fn apply_matrix_categories(
    heatmaps: &mut [MenuHeatmapPayload],
    categories_by_menu: &HashMap<String, String>,
) {
    for item in heatmaps {
        item.matrix_category = categories_by_menu.get(&item.menu).cloned();
    }
}

pub fn calculate_menu_heatmaps(rows: &[LineItem]) -> Result<Vec<MenuHeatmapPayload>, String> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }

    let reporting_period = rows
        .iter()
        .map(|r| r.order_time)
        .min()
        .map(|dt| dt.format("%Y-%m").to_string())
        .unwrap_or_default();

    let mut categories: HashMap<String, (Option<String>, Option<String>)> = HashMap::new();
    let mut hourly: HashMap<String, [i64; 24]> = HashMap::new();
    let mut weekly: HashMap<String, [i64; 7]> = HashMap::new();

    for row in rows {
        let entry = categories.entry(row.menu.clone()).or_insert((
            Some(row.menu_category.clone()),
            Some(row.menu_category_detail.clone()),
        ));
        if entry.0.as_deref() != Some(row.menu_category.as_str()) {
            return Err(format!(
                "Menu '{}' has multiple categories: [{:?}, {:?}]",
                row.menu, entry.0, row.menu_category
            ));
        }
        if entry.1.as_deref() != Some(row.menu_category_detail.as_str()) {
            return Err(format!(
                "Menu '{}' has multiple category details: [{:?}, {:?}]",
                row.menu, entry.1, row.menu_category_detail
            ));
        }

        let hour = row.order_time.hour() as usize;
        hourly.entry(row.menu.clone()).or_insert([0; 24])[hour] += row.qty;

        let weekday = weekday_index(row.order_time.weekday().num_days_from_monday());
        weekly.entry(row.menu.clone()).or_insert([0; 7])[weekday] += row.qty;
    }

    let mut payloads = Vec::new();
    for (menu, (menu_category, menu_category_detail)) in categories {
        let hours = hourly.get(&menu).copied().unwrap_or([0; 24]);
        let days = weekly.get(&menu).copied().unwrap_or([0; 7]);
        payloads.push(MenuHeatmapPayload {
            menu,
            menu_category,
            menu_category_detail,
            matrix_category: None,
            daily_heatmap: (0..24)
                .map(|hour| DailyHeatmapRow {
                    hour,
                    quantity: hours[hour as usize],
                })
                .collect(),
            weekly_heatmap: WEEKDAY_ORDER
                .iter()
                .enumerate()
                .map(|(idx, day)| WeeklyHeatmapRow {
                    day: (*day).to_string(),
                    quantity: days[idx],
                })
                .collect(),
            reporting_period: reporting_period.clone(),
        });
    }

    payloads.sort_by(|a, b| {
        let a_total: i64 = a.daily_heatmap.iter().map(|r| r.quantity).sum();
        let b_total: i64 = b.daily_heatmap.iter().map(|r| r.quantity).sum();
        b_total.cmp(&a_total).then_with(|| a.menu.cmp(&b.menu))
    });
    Ok(payloads)
}

fn weekday_index(num_days_from_monday: u32) -> usize {
    num_days_from_monday as usize
}
