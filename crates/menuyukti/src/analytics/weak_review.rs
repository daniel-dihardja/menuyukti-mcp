//! Weak items review adapter: filter menu-engineering matrix to the weak quadrant
//! and derive avg qty/day + portfolio KPIs (parity with v3 `adaptWeakPage`).

use chrono::NaiveDate;

use serde::Serialize;

use crate::analytics::menu_engineering::{
    MenuEngineeringDistributionItem, MenuEngineeringMatrixItem, MenuEngineeringMatrixResult,
};
use crate::line_item::LineItem;

const VALID_ACTIONS: &[&str] = &["keep", "promote", "reprice", "remove"];

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WeakDisplayRow {
    pub menu: String,
    pub units_sold: i64,
    /// Null when period length is unknown.
    pub avg_qty_per_day: Option<f64>,
    pub contribution_margin: f64,
    pub contribution_margin_share: f64,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WeakPortfolioStats {
    pub item_count: usize,
    pub item_share: f64,
    pub margin_share: f64,
    pub margin_amount: f64,
    pub total_menu_items: usize,
    pub period_days: Option<i64>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WeakDistributionStats {
    pub item_count: usize,
    pub item_share: f64,
    pub margin_share: f64,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WeakReviewResult {
    pub rows: Vec<WeakDisplayRow>,
    pub stats: WeakPortfolioStats,
    pub distribution_stats: WeakDistributionStats,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
}

fn normalize_action(raw: &str) -> Option<String> {
    let lower = raw.trim().to_lowercase();
    if VALID_ACTIONS.contains(&lower.as_str()) {
        Some(lower)
    } else {
        None
    }
}

fn parse_iso_date_only(value: &str) -> Option<NaiveDate> {
    let trimmed = value.trim();
    NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").ok()
}

/// Inclusive calendar-day count between two ISO dates (`YYYY-MM-DD`).
/// Same day → 1. Returns `None` when either date is missing/invalid or start > end.
pub fn inclusive_calendar_days(
    period_start: Option<&str>,
    period_end: Option<&str>,
) -> Option<i64> {
    let start = period_start.and_then(parse_iso_date_only)?;
    let end = period_end.and_then(parse_iso_date_only)?;
    let diff = (end - start).num_days();
    if diff < 0 {
        None
    } else {
        Some(diff + 1)
    }
}

/// Derive inclusive sales period (`YYYY-MM-DD`) from line-item order times.
pub fn period_from_line_items(rows: &[LineItem]) -> (Option<String>, Option<String>) {
    let mut min_date: Option<NaiveDate> = None;
    let mut max_date: Option<NaiveDate> = None;
    for row in rows {
        let day = row.order_time.date();
        min_date = Some(match min_date {
            Some(m) if m <= day => m,
            _ => day,
        });
        max_date = Some(match max_date {
            Some(m) if m >= day => m,
            _ => day,
        });
    }
    (
        min_date.map(|d| d.format("%Y-%m-%d").to_string()),
        max_date.map(|d| d.format("%Y-%m-%d").to_string()),
    )
}

fn weak_distribution(distribution: &[MenuEngineeringDistributionItem]) -> (usize, f64, f64) {
    distribution
        .iter()
        .find(|d| d.category.eq_ignore_ascii_case("weak"))
        .map(|d| (d.item_count, d.item_share, d.margin_share))
        .unwrap_or((0, 0.0, 0.0))
}

fn to_display_row(item: &MenuEngineeringMatrixItem, period_days: Option<i64>) -> WeakDisplayRow {
    let units_sold = item.quantity;
    let contribution_margin_share = if item.contribution_margin_percentage.is_finite()
        && (0.0..=1.0).contains(&item.contribution_margin_percentage)
    {
        item.contribution_margin_percentage
    } else {
        0.0
    };
    let contribution_margin = if item.contribution_margin.is_finite() {
        item.contribution_margin
    } else {
        0.0
    };
    let avg_qty_per_day = match period_days {
        Some(days) if days > 0 => Some(units_sold as f64 / days as f64),
        _ => None,
    };
    let menu = {
        let trimmed = item.menu.trim();
        if trimmed.is_empty() {
            "Unknown".to_string()
        } else {
            trimmed.to_string()
        }
    };
    WeakDisplayRow {
        menu,
        units_sold,
        avg_qty_per_day,
        contribution_margin,
        contribution_margin_share,
        action: normalize_action(&item.action),
    }
}

/// Build Weak items review rows and portfolio stats from a full matrix payload.
pub fn adapt_weak_review(
    items: &[MenuEngineeringMatrixItem],
    distribution: &[MenuEngineeringDistributionItem],
    period_start: Option<&str>,
    period_end: Option<&str>,
) -> WeakReviewResult {
    let period_days = inclusive_calendar_days(period_start, period_end);
    let (weak_dist_count, weak_dist_item_share, weak_dist_margin_share) =
        weak_distribution(distribution);

    let total_menu_items = items.len();
    let rows: Vec<WeakDisplayRow> = items
        .iter()
        .filter(|item| item.category.eq_ignore_ascii_case("weak"))
        .map(|item| to_display_row(item, period_days))
        .collect();

    let margin_amount: f64 = rows.iter().map(|r| r.contribution_margin).sum();
    let total_margin: f64 = items
        .iter()
        .map(|i| {
            if i.contribution_margin.is_finite() {
                i.contribution_margin
            } else {
                0.0
            }
        })
        .sum();

    let item_count = rows.len();
    let item_share = if weak_dist_count > 0 {
        weak_dist_item_share
    } else if total_menu_items > 0 {
        item_count as f64 / total_menu_items as f64
    } else {
        0.0
    };
    let margin_share = if weak_dist_count > 0 || weak_dist_margin_share > 0.0 {
        weak_dist_margin_share
    } else if total_margin > 0.0 {
        margin_amount / total_margin
    } else {
        0.0
    };

    WeakReviewResult {
        rows,
        stats: WeakPortfolioStats {
            item_count,
            item_share,
            margin_share,
            margin_amount,
            total_menu_items,
            period_days,
        },
        distribution_stats: WeakDistributionStats {
            item_count: if weak_dist_count > 0 {
                weak_dist_count
            } else {
                item_count
            },
            item_share,
            margin_share,
        },
        period_start: period_start
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()),
        period_end: period_end
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()),
    }
}

/// Adapt a matrix result using an explicit ISO period (`YYYY-MM-DD`).
pub fn adapt_weak_review_from_matrix(
    matrix: &MenuEngineeringMatrixResult,
    period_start: Option<&str>,
    period_end: Option<&str>,
) -> WeakReviewResult {
    adapt_weak_review(
        &matrix.items,
        &matrix.distribution,
        period_start,
        period_end,
    )
}

/// Adapt a matrix result, deriving the sales period from line-item order times.
pub fn adapt_weak_review_from_orders(
    matrix: &MenuEngineeringMatrixResult,
    rows: &[LineItem],
) -> WeakReviewResult {
    let (start, end) = period_from_line_items(rows);
    adapt_weak_review(
        &matrix.items,
        &matrix.distribution,
        start.as_deref(),
        end.as_deref(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(
        menu: &str,
        quantity: i64,
        contribution_margin: f64,
        contribution_margin_percentage: f64,
        category: &str,
        action: &str,
    ) -> MenuEngineeringMatrixItem {
        MenuEngineeringMatrixItem {
            menu: menu.into(),
            quantity,
            total_revenue: 0.0,
            cogs: 0.0,
            total_cogs: 0.0,
            contribution_margin,
            contribution_margin_percentage,
            margin_per_unit: 0.0,
            we_value: 0.0,
            category: category.into(),
            action: action.into(),
            menu_category: None,
            menu_category_detail: None,
        }
    }

    fn sample_items() -> Vec<MenuEngineeringMatrixItem> {
        vec![
            item("Star Burger", 100, 800.0, 0.8, "star", "keep"),
            item("Weak Salad", 10, 20.0, 0.02, "weak", "remove"),
            item("Quiet Soup", 20, 40.0, 0.04, "weak", "reprice"),
        ]
    }

    fn sample_distribution() -> Vec<MenuEngineeringDistributionItem> {
        vec![
            MenuEngineeringDistributionItem {
                category: "star".into(),
                item_count: 1,
                item_share: 1.0 / 3.0,
                margin_share: 0.8,
            },
            MenuEngineeringDistributionItem {
                category: "weak".into(),
                item_count: 2,
                item_share: 2.0 / 3.0,
                margin_share: 0.06,
            },
        ]
    }

    #[test]
    fn inclusive_calendar_days_counts_same_day_as_one() {
        assert_eq!(
            inclusive_calendar_days(Some("2024-01-01"), Some("2024-01-01")),
            Some(1)
        );
        assert_eq!(
            inclusive_calendar_days(Some("2024-01-01"), Some("2024-01-10")),
            Some(10)
        );
        assert_eq!(inclusive_calendar_days(None, Some("2024-01-10")), None);
        assert_eq!(
            inclusive_calendar_days(Some("2024-01-10"), Some("2024-01-01")),
            None
        );
    }

    #[test]
    fn filters_to_weak_rows_and_derives_avg_qty_per_day() {
        let model = adapt_weak_review(
            &sample_items(),
            &sample_distribution(),
            Some("2024-01-01"),
            Some("2024-01-10"),
        );

        assert_eq!(model.rows.len(), 2);
        assert_eq!(model.stats.period_days, Some(10));
        assert_eq!(model.stats.item_count, 2);
        assert_eq!(model.stats.total_menu_items, 3);
        assert!((model.stats.item_share - 2.0 / 3.0).abs() < 1e-9);
        assert!((model.stats.margin_share - 0.06).abs() < 1e-9);
        assert!((model.stats.margin_amount - 60.0).abs() < 1e-9);
        assert_eq!(model.period_start.as_deref(), Some("2024-01-01"));
        assert_eq!(model.period_end.as_deref(), Some("2024-01-10"));

        let salad = model
            .rows
            .iter()
            .find(|r| r.menu == "Weak Salad")
            .expect("salad");
        assert!((salad.avg_qty_per_day.unwrap() - 1.0).abs() < 1e-9);
        assert_eq!(salad.action.as_deref(), Some("remove"));
        assert!((salad.contribution_margin_share - 0.02).abs() < 1e-9);

        let soup = model
            .rows
            .iter()
            .find(|r| r.menu == "Quiet Soup")
            .expect("soup");
        assert!((soup.avg_qty_per_day.unwrap() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn omits_avg_qty_per_day_when_period_missing() {
        let model = adapt_weak_review(&sample_items(), &sample_distribution(), None, None);
        assert_eq!(model.stats.period_days, None);
        assert!(model.rows.iter().all(|r| r.avg_qty_per_day.is_none()));
        assert_eq!(model.rows.len(), 2);
    }

    #[test]
    fn empty_weak_quadrant_returns_structured_empty() {
        let items = vec![item("Star Burger", 100, 800.0, 0.8, "star", "keep")];
        let distribution = vec![MenuEngineeringDistributionItem {
            category: "star".into(),
            item_count: 1,
            item_share: 1.0,
            margin_share: 1.0,
        }];
        let model = adapt_weak_review(
            &items,
            &distribution,
            Some("2024-01-01"),
            Some("2024-01-07"),
        );
        assert!(model.rows.is_empty());
        assert_eq!(model.stats.item_count, 0);
        assert_eq!(model.stats.margin_amount, 0.0);
        assert_eq!(model.stats.total_menu_items, 1);
        assert_eq!(model.stats.period_days, Some(7));
    }

    #[test]
    fn period_from_line_items_uses_order_dates() {
        use chrono::NaiveDate;

        let rows = vec![
            LineItem {
                bill_number: "1".into(),
                menu: "A".into(),
                qty: 1,
                price: 1.0,
                total_after_bill_discount: 1.0,
                order_time: NaiveDate::from_ymd_opt(2026, 7, 1)
                    .unwrap()
                    .and_hms_opt(10, 0, 0)
                    .unwrap(),
                menu_category: String::new(),
                menu_category_detail: String::new(),
            },
            LineItem {
                bill_number: "2".into(),
                menu: "B".into(),
                qty: 1,
                price: 1.0,
                total_after_bill_discount: 1.0,
                order_time: NaiveDate::from_ymd_opt(2026, 7, 15)
                    .unwrap()
                    .and_hms_opt(18, 0, 0)
                    .unwrap(),
                menu_category: String::new(),
                menu_category_detail: String::new(),
            },
        ];
        let (start, end) = period_from_line_items(&rows);
        assert_eq!(start.as_deref(), Some("2026-07-01"));
        assert_eq!(end.as_deref(), Some("2026-07-15"));
        assert_eq!(
            inclusive_calendar_days(start.as_deref(), end.as_deref()),
            Some(15)
        );
    }

    #[test]
    fn falls_back_to_row_shares_when_distribution_lacks_weak() {
        let items = sample_items();
        let distribution = vec![MenuEngineeringDistributionItem {
            category: "star".into(),
            item_count: 1,
            item_share: 1.0,
            margin_share: 0.8,
        }];
        let model = adapt_weak_review(&items, &distribution, None, None);
        assert_eq!(model.stats.item_count, 2);
        assert!((model.stats.item_share - 2.0 / 3.0).abs() < 1e-9);
        // margin_amount 60 / total_margin 860
        assert!((model.stats.margin_share - (60.0 / 860.0)).abs() < 1e-9);
    }
}
