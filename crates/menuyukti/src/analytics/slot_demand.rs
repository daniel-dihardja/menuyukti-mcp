//! Venue-level demand index per day × meal-period slot.
//!
//! Each slot's demand index is order share relative to the average slot
//! (1.0 = average traffic across all 35 day × meal-period cells).

use std::collections::HashMap;

use chrono::{Datelike, NaiveDateTime, Timelike, Weekday};
use serde::Serialize;

use crate::analytics::heatmaps::WEEKDAY_ORDER;
use crate::line_item::LineItem;

pub const LOW_DEMAND_THRESHOLD: f64 = 0.9;
pub const HIGH_DEMAND_THRESHOLD: f64 = 1.1;

/// (period id, short label, hours-range label, hours in period).
const MEAL_PERIODS: [(&str, &str, &str, &[u32]); 5] = [
    (
        "breakfast",
        "Breakfast",
        "05:00–10:59",
        &[5, 6, 7, 8, 9, 10],
    ),
    ("lunch", "Lunch", "11:00–14:59", &[11, 12, 13, 14]),
    ("afternoon", "Afternoon", "15:00–16:59", &[15, 16]),
    ("dinner", "Dinner", "17:00–21:59", &[17, 18, 19, 20, 21]),
    (
        "late_night",
        "Late Night",
        "22:00–04:59",
        &[22, 23, 0, 1, 2, 3, 4],
    ),
];

const SLOT_COUNT: usize = WEEKDAY_ORDER.len() * MEAL_PERIODS.len();
const MEAN_SLOT_TRAFFIC_SHARE: f64 = 1.0 / SLOT_COUNT as f64;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SlotDemandCell {
    pub day: String,
    pub meal_period: String,
    pub meal_period_label: String,
    pub meal_period_hours_label: String,
    pub order_count: i64,
    pub traffic_share: f64,
    pub demand_index: f64,
    pub relative_demand: String,
}

fn weekday_abbr(dt: NaiveDateTime) -> &'static str {
    match dt.weekday() {
        Weekday::Mon => "mon",
        Weekday::Tue => "tue",
        Weekday::Wed => "wed",
        Weekday::Thu => "thu",
        Weekday::Fri => "fri",
        Weekday::Sat => "sat",
        Weekday::Sun => "sun",
    }
}

fn meal_period_for_hour(hour: u32) -> (&'static str, &'static str, &'static str) {
    for (period, label, hours_label, hours) in MEAL_PERIODS {
        if hours.contains(&hour) {
            return (period, label, hours_label);
        }
    }
    let (period, label, hours_label, _) = MEAL_PERIODS[4];
    (period, label, hours_label)
}

fn relative_demand(demand_index: f64) -> &'static str {
    if demand_index < LOW_DEMAND_THRESHOLD {
        "low"
    } else if demand_index > HIGH_DEMAND_THRESHOLD {
        "high"
    } else {
        "average"
    }
}

fn round4(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}

/// Compute venue demand index for each day × meal-period slot.
///
/// Uses distinct bills via minimum `order_time` per `bill_number`.
pub fn compute_slot_demand_profile_from_orders(rows: &[LineItem]) -> Vec<SlotDemandCell> {
    if rows.is_empty() {
        return Vec::new();
    }

    let mut bill_min_time: HashMap<String, NaiveDateTime> = HashMap::new();
    for row in rows {
        let bill = row.bill_number.trim();
        if bill.is_empty() {
            continue;
        }
        bill_min_time
            .entry(bill.to_string())
            .and_modify(|t| {
                if row.order_time < *t {
                    *t = row.order_time;
                }
            })
            .or_insert(row.order_time);
    }

    let total_orders = bill_min_time.len();
    if total_orders == 0 {
        return Vec::new();
    }

    let mut slot_order_counts: HashMap<(String, String), i64> = HashMap::new();
    for dt in bill_min_time.values() {
        let day = weekday_abbr(*dt).to_string();
        let (period, _, _) = meal_period_for_hour(dt.hour());
        *slot_order_counts
            .entry((day, period.to_string()))
            .or_insert(0) += 1;
    }

    let mut cells = Vec::with_capacity(SLOT_COUNT);
    for day in WEEKDAY_ORDER {
        for (period, label, hours_label, _) in MEAL_PERIODS {
            let order_count = slot_order_counts
                .get(&(day.to_string(), period.to_string()))
                .copied()
                .unwrap_or(0);
            let traffic_share = order_count as f64 / total_orders as f64;
            let demand_index = if MEAN_SLOT_TRAFFIC_SHARE > 0.0 {
                traffic_share / MEAN_SLOT_TRAFFIC_SHARE
            } else {
                0.0
            };
            cells.push(SlotDemandCell {
                day: day.to_string(),
                meal_period: period.to_string(),
                meal_period_label: label.to_string(),
                meal_period_hours_label: hours_label.to_string(),
                order_count,
                traffic_share: round4(traffic_share),
                demand_index: round4(demand_index),
                relative_demand: relative_demand(demand_index).to_string(),
            });
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    fn item(bill: &str, order_time: NaiveDateTime) -> LineItem {
        LineItem {
            bill_number: bill.into(),
            menu: "Item".into(),
            qty: 1,
            price: 1.0,
            total_after_bill_discount: 1.0,
            order_time,
            menu_category: String::new(),
            menu_category_detail: String::new(),
        }
    }

    fn dt(year: i32, month: u32, day: u32, hour: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, 0, 0)
            .unwrap()
    }

    #[test]
    fn empty_slot_profile_returns_empty() {
        assert!(compute_slot_demand_profile_from_orders(&[]).is_empty());
    }

    #[test]
    fn slot_profile_returns_35_cells() {
        let rows: Vec<_> = (0..20)
            .map(|i| item(&format!("B{i}"), dt(2024, 1, 1 + (i % 7), 12)))
            .collect();
        let profile = compute_slot_demand_profile_from_orders(&rows);
        assert_eq!(profile.len(), 35);
    }

    #[test]
    fn uses_min_order_time_per_bill() {
        // Two lines on same bill at lunch then dinner → counts once as lunch.
        let rows = vec![
            item("B1", dt(2024, 1, 1, 12)),
            item("B1", dt(2024, 1, 1, 19)),
        ];
        let profile = compute_slot_demand_profile_from_orders(&rows);
        let by_slot: HashMap<(_, _), _> = profile
            .iter()
            .map(|c| ((c.day.as_str(), c.meal_period.as_str()), c))
            .collect();
        assert_eq!(by_slot[&("mon", "lunch")].order_count, 1);
        assert_eq!(by_slot[&("mon", "dinner")].order_count, 0);
    }

    #[test]
    fn weak_mon_lunch_vs_strong_fri_dinner() {
        let mut rows = Vec::new();
        for i in 0..3 {
            rows.push(item(&format!("ML{i}"), dt(2024, 1, 1, 12)));
        }
        for i in 0..20 {
            rows.push(item(&format!("MD{i}"), dt(2024, 1, 1, 19)));
        }
        for day in [2, 3, 4, 6, 7] {
            for i in 0..25 {
                rows.push(item(&format!("L{day}{i}"), dt(2024, 1, day, 12)));
            }
        }
        for i in 0..200 {
            rows.push(item(&format!("FD{i}"), dt(2024, 1, 5, 19)));
        }

        let profile = compute_slot_demand_profile_from_orders(&rows);
        let by_slot: HashMap<(_, _), _> = profile
            .iter()
            .map(|c| ((c.day.as_str(), c.meal_period.as_str()), c))
            .collect();

        let mon_lunch = by_slot[&("mon", "lunch")];
        let fri_dinner = by_slot[&("fri", "dinner")];
        assert!(mon_lunch.demand_index < fri_dinner.demand_index);
        assert_eq!(mon_lunch.relative_demand, "low");
        assert_eq!(fri_dinner.relative_demand, "high");
    }

    #[test]
    fn sparse_breakfast_slots_stay_below_average() {
        let mut rows = Vec::new();
        for i in 0..12 {
            rows.push(item(&format!("B{i}"), dt(2024, 1, 1 + (i % 7), 8)));
        }
        for i in 0..200 {
            rows.push(item(&format!("L{i}"), dt(2024, 1, 1 + (i % 7), 12)));
        }
        for i in 0..300 {
            rows.push(item(&format!("D{i}"), dt(2024, 1, 1 + (i % 7), 19)));
        }

        let profile = compute_slot_demand_profile_from_orders(&rows);
        let breakfast: Vec<_> = profile
            .iter()
            .filter(|c| c.meal_period == "breakfast")
            .collect();
        assert!(!breakfast.is_empty());
        assert!(breakfast
            .iter()
            .all(|c| c.demand_index < LOW_DEMAND_THRESHOLD));
        assert!(breakfast.iter().all(|c| c.relative_demand == "low"));
        assert!(breakfast.iter().map(|c| c.demand_index).fold(0.0, f64::max) < 1.5);
    }
}
