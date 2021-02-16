use crate::model::{Activity, CategoryLookup, Config};
use crate::report::common;
use crate::storage;
use itertools::Itertools;
use std::collections::HashMap;

/// Print the report for today
pub fn run(config: &Config) -> Result<(), String> {
    let categories = storage::read_categories(config)?;
    let activities = storage::read_today(config)?;

    println!("\n{}", report(&activities, &categories));
    Ok(())
}

//
// Internals -----------------------------
//

/// Generate the report for today as string
fn report(activities: &[Activity], categories: &CategoryLookup) -> String {
    let mut by_category = HashMap::new();
    let mut individual: HashMap<String, Vec<u32>> = HashMap::new();
    let total = common::weighted_total(activities, categories);

    for activity in activities {
        let cat = &activity.category;

        if by_category.contains_key(cat) {
            let parts = individual.get_mut(cat).unwrap();
            parts.push(activity.reps);
            let sum = by_category.get(cat).unwrap() + activity.reps;
            by_category.insert(cat.to_string(), sum);
        } else {
            let mut parts = Vec::new();
            parts.push(activity.reps);
            individual.insert(cat.to_string(), parts);

            by_category.insert(cat.to_string(), activity.reps);
        };
    }

    let mut result = String::new();

    result.push_str("Stats for today:\n");
    for category in by_category.keys().sorted() {
        let reps = by_category.get(category).unwrap();

        let details = if individual.get(category).unwrap().len() > 1 {
            format!(
                " ({})",
                individual.get(category).unwrap().iter().join(" + ")
            )
        } else {
            "".to_string()
        };

        result.push_str(&format!("  {:<15}: {} reps{}\n", category, reps, details));
    }

    if by_category.len() > 1 {
        result.push_str(&format!("  Weighted total : {}", total));
    }

    result
}

//
// Tests ------------------------------------
//
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

        let report = report(&activities, &lookup);

        assert_eq!(
            report,
            "\
Stats for today:
  Beers          : 28 reps
  Burpees        : 33 reps (20 + 13)
  Pushups        : 15 reps
  Steps          : 3200 reps
  Weighted total : 124"
        );
    }

    fn newcat(name: &str, weight: f64) -> Category {
        Category::new(name, weight, Vec::<String>::new())
    }
}
