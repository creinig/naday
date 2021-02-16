use crate::model::{Activity, CategoryLookup, Config};
use crate::report::common::DayStats;
use crate::storage;
use chrono::{Date, Datelike, Duration, Local};
use std::collections::HashMap;
use std::vec::Vec;

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
        stats.add(&activity);
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

fn print_stats(stats: &[DayStats], category: Option<String>, categories: &CategoryLookup) {
    match category {
        Some(ref cat) => {
            println!(
                "Report on {} for the past {} days\n",
                &categories.find(cat).unwrap().name,
                stats.len()
            );

            for day in stats {
                println!(
                    "{:3}: {:>5} reps ({:>5} total)",
                    day.day.weekday(),
                    day.reps_by_category.get(cat).unwrap_or(&0),
                    day.reps_total(categories)
                );
            }
        }

        None => {
            println!(
                "Report on the weighted total for the past {} days\n",
                stats.len()
            );

            for day in stats {
                println!(
                    "{:3}: {:>5} total",
                    day.day.weekday(),
                    day.reps_total(categories)
                );
            }
        }
    }
}

//
// Tests ---------------------------------------------------
//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Activity;
    use chrono::{Local, TimeZone};

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
}
