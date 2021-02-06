use crate::model::{Activity, CategoryLookup};
use chrono::{Date, Local};
use std::collections::HashMap;

/// Calculate the weighted total repetitions over the the given activities
pub fn weighted_total(activities: &Vec<Activity>, categories: &CategoryLookup) -> u32 {
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
            day: day.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Activity, Category, CategoryLookup};

    #[test]
    fn basic() {
        let mut lookup = CategoryLookup::new();

        lookup.add(newcat("Pushups", 1.0)).unwrap();
        lookup.add(newcat("Burpees", 1.5)).unwrap();
        lookup.add(newcat("Steps", 0.01)).unwrap();

        let mut activities = Vec::new();
        activities.push(Activity::new(15, "Pushups"));
        activities.push(Activity::new(20, "Burpees"));
        activities.push(Activity::new(13, "Burpees"));
        activities.push(Activity::new(3200, "Steps"));
        activities.push(Activity::new(28, "Beers"));

        assert_eq!(
            weighted_total(&activities, &lookup),
            (15.0 + ((20.0 + 13.0) * 1.5) + (3200.0 * 0.01) + 28.0) as u32
        );
    }

    fn newcat(name: &str, weight: f64) -> Category {
        Category::new(name, weight, Vec::<String>::new())
    }
}
