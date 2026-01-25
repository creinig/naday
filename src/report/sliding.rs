use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

use chrono::{Date, Datelike, Duration, Local};

use crate::model::{Activity, Category, CategoryLookup, Config};
use crate::report::common::DayStats;
use crate::storage;

pub fn sliding_days(
    end_date: Date<Local>,
    number_of_days: u32,
    category: Option<String>,
    config: &Config,
) -> Result<(), String> {
    let start_date = end_date - Duration::days((number_of_days - 1).into());

    let categories = storage::read_categories(config)?;
    let activities = storage::read_days(&start_date, &end_date, config)?;
    let stats = build_stats(&activities, &start_date, &end_date);

    print_stats(&stats, category, &categories);

    Ok(())
}

//
// Internals -----------------------------------
//

/// Build daily statistics for the given category
///
///  # Arguments
///  * `activities`: All recorded activities in the given interval
///  * `start`: Interval start date
///  * `end`: Interval end date
///
///  # Returns
///  A vector with one entry per day in (start..=end), each holding the total number of reps for
///  all categories
fn build_stats(activities: &[Activity], start: &Date<Local>, end: &Date<Local>) -> Vec<DayStats> {
    let mut by_day: HashMap<Date<Local>, DayStats> = HashMap::new();

    for activity in activities {
        let today = activity.timestamp.date();

        let stats = by_day.entry(today).or_insert_with(|| DayStats::new(&today));
        stats.add(activity);
    }

    let mut results = Vec::new();
    let mut day = *start;
    while day <= *end {
        let stats = by_day.remove(&day).unwrap_or_else(|| DayStats::new(&day));
        results.push(stats);
        day = day.succ();
    }

    results
}

/// Render the given DayStats into a collection of output lines (including header)
fn render_stats(
    stats: &[DayStats],
    category: Option<String>,
    categories: &CategoryLookup,
) -> Vec<String> {
    let cat = category.and_then(|c| categories.find(c));
    let mut lines = Vec::with_capacity(stats.len() + 1);

    lines.push(match cat {
        Some(ref cat) => format!("Report on {} for the past {} days\n", cat.name, stats.len()),

        None => format!(
            "Report on the weighted total for the past {} days\n",
            stats.len()
        ),
    });

    for day in stats {
        lines.push(render_daystat(day, &cat, categories));
    }

    lines
}

/// Render the given single DayStats object to a line
fn render_daystat(
    day: &DayStats,
    category: &Option<Rc<Category>>,
    categories: &CategoryLookup,
) -> String {
    match category {
        Some(cat) => {
            format!(
                "{:3}: {:>5} reps ({:>5} total)",
                day.day.weekday(),
                day.reps_by_category.get(&cat.name).unwrap_or(&0),
                day.reps_total(categories)
            )
        }

        None => {
            format!(
                "{:3}: {:>5} total",
                day.day.weekday(),
                day.reps_total(categories)
            )
        }
    }
}

fn print_stats(stats: &[DayStats], category: Option<String>, categories: &CategoryLookup) {
    render_stats(stats, category, categories)
        .iter()
        .for_each(|line| println!("{}", line));
}

//
// Tests ---------------------------------------------------
//

#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};

    use crate::model::Activity;

    use super::*;
    use super::super::test_common::*;

    #[test]
    fn build_stats_basic() {
        let mut activities = Vec::new();

        // - create activities over multiple days (with multiple A per day & category, and multiple
        // categories per day)
        activities.push(activity(5, 13, "Pushups"));
        activities.push(activity(5, 23, "Pushups"));
        activities.push(activity(5, 23, "Burpees"));
        activities.push(activity(6, 15, "Burpees"));
        activities.push(activity(7, 14, "Burpees"));
        activities.push(activity(7, 23, "Burpees"));

        let start = Local.ymd(2020, 7, 1);
        let end = Local.ymd(2020, 7, 30);
        let stats = build_stats(&activities, &start, &end);

        assert_eq!(30, stats.len());

        for daystat in stats {
            let rbc = &daystat.reps_by_category;

            match daystat.day.day() {
                5 => {
                    assert_eq!(2, rbc.len());
                    assert_eq!(&36, rbc.get("Pushups").unwrap());
                    assert_eq!(&23, rbc.get("Burpees").unwrap());
                }
                6 => {
                    assert_eq!(1, rbc.len());
                    assert_eq!(&15, rbc.get("Burpees").unwrap());
                }
                7 => {
                    assert_eq!(1, rbc.len());
                    assert_eq!(&37, rbc.get("Burpees").unwrap());
                }
                _ => {
                    assert_eq!(0, daystat.reps_by_category.len());
                }
            }
        }
    }

    fn activity(day_of_month: u32, reps: u32, category: &str) -> Activity {
        let time = Local.ymd(2020, 7, day_of_month).and_hms(13, 45, 34);
        Activity {
            timestamp: time,
            reps,
            category: category.to_string(),
        }
    }

    #[test]
    fn render_daystat() {
        let time = Local.ymd(2020, 7, 23);
        let lookup = new_lookup();
        let mut stats = DayStats::new(&time);
        stats.add(&Activity::new(15, "Pushups"));
        stats.add(&Activity::new(23, "Burpees"));
        stats.add(&Activity::new(16, "Burpees"));

        let rendered = super::render_daystat(&stats, &lookup.find("Pushups"), &lookup);
        assert_eq!(rendered, "Thu:    15 reps (   73 total)");

        let rendered = super::render_daystat(&stats, &lookup.find("Burpees"), &lookup);
        assert_eq!(rendered, "Thu:    39 reps (   73 total)");

        let rendered = super::render_daystat(&stats, &lookup.find("Steps"), &lookup);
        assert_eq!(rendered, "Thu:     0 reps (   73 total)");

        let rendered = super::render_daystat(&stats, &None, &lookup);
        assert_eq!(rendered, "Thu:    73 total");
    }

    fn new_lookup() -> CategoryLookup {
        let mut lookup = CategoryLookup::new();
        lookup.add(Category::new("Pushups", 1.0, vec!["pu"])).unwrap();
        lookup.add(Category::new("Burpees", 1.5, vec!["bu"])).unwrap();
        lookup.add(Category::new("Steps", 0.02,  vec!["st"])).unwrap();

        lookup
    }
}
