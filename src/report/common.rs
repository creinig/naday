use crate::model::{Activity, CategoryLookup};
use chrono::{Date, Local};
use std::collections::HashMap;

/// Calculate the weighted total repetitions over the the given activities
pub fn weighted_total(activities: &[Activity], categories: &CategoryLookup) -> u32 {
    let mut total = 0;

    for activity in activities {
        let cat = &activity.category;
        if let Some(category) = categories.find(cat) {
            total += ((activity.reps as f64) * category.weight) as u32;
        } else {
            // default to a weight of 1 (e.g. for categories that don't exist anymore)
            total += activity.reps;
        }
    }

    total
}

/// Struct representing the aggregated stats for one day
#[derive(Debug)]
pub struct DayStats {
    pub day: Date<Local>,
    pub reps_by_category: HashMap<String, u32>,
}

impl DayStats {
    pub fn new(day: &Date<Local>) -> DayStats {
        DayStats {
            day: *day,
            reps_by_category: HashMap::new(),
        }
    }

    /// Calculate the weighted total of all repetitions in this day
    pub fn reps_total(&self, categories: &CategoryLookup) -> u32 {
        let mut total = 0;

        for (cat, reps) in self.reps_by_category.iter() {
            if let Some(category) = categories.find(cat) {
                total += ((*reps as f64) * category.weight) as u32;
            } else {
                // default to a weight of 1 (e.g. for categories that don't exist anymore)
                total += reps;
            }
        }

        total
    }

    /// Add the given activity to the reps in this day
    pub fn add(&mut self, activity: &Activity) {
        self.reps_by_category
            .entry(activity.category.to_string())
            .and_modify(|e| *e += activity.reps)
            .or_insert(activity.reps);
    }
}
