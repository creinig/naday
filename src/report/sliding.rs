use crate::model::{Activity, Category, CategoryLookup, Config};
use crate::report::common::DayStats;
use crate::storage;
use chrono::{Date, Datelike, Duration, Local};
use std::collections::HashMap;
use std::vec::Vec;

pub fn sliding_days(
    end_date: Date<Local>,
    number_of_days: u32,
    config: &Config,
) -> Result<(), String> {
    let start_date = end_date - Duration::days((number_of_days - 1).into());

    let categories = storage::read_categories(config)?;
    let activities = storage::read_days(&start_date, &end_date, config)?;
    let stats = build_stats(&activities, &start_date, &end_date);
    print_stats(&stats, None, &categories);

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
fn build_stats(
    activities: &Vec<Activity>,
    start: &Date<Local>,
    end: &Date<Local>,
) -> Vec<DayStats> {
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

fn print_stats(stats: &Vec<DayStats>, category: Option<&Category>, categories: &CategoryLookup) {
    for day in stats {
        match category {
            Some(cat) => println!(
                "{:3}: {:>5} reps ({:>5} total)",
                day.day.weekday(),
                day.reps_by_category.get(&cat.name).unwrap_or(&0),
                day.reps_total(categories)
            ),

            None => println!(
                "{:3}: {:>5} total",
                day.day.weekday(),
                day.reps_total(categories)
            ),
        }
    }
}
