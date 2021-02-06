use crate::model::{Activity, Category, CategoryLookup};
use crate::report::common::*;
use chrono::Local;

#[test]
fn weighted_total_basic() {
    let lookup = new_lookup();

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

#[test]
fn daystats_basic() {
    let mut stats = DayStats::new(&Local::now().date());
    let lookup = new_lookup();

    assert_eq!(0, stats.reps_total(&lookup));
    stats.add(&Activity::new(15, "Pushups"));
    stats.add(&Activity::new(20, "Pushups"));
    stats.add(&Activity::new(23, "Pullups"));
    stats.add(&Activity::new(20, "Burpees"));
    stats.add(&Activity::new(1500, "Steps"));

    assert_eq!(
        stats.reps_total(&lookup),
        (15.0 + 20.0 + 23.0 + (20.0 * 1.5) + (1500.0 * 0.01)) as u32
    );

    assert_eq!(*stats.reps_by_category.get("Pushups").unwrap(), 15 + 20);
    assert_eq!(*stats.reps_by_category.get("Pullups").unwrap(), 23);
    assert_eq!(*stats.reps_by_category.get("Burpees").unwrap(), 20);
    assert_eq!(*stats.reps_by_category.get("Steps").unwrap(), 1500);
}

fn newcat(name: &str, weight: f64) -> Category {
    Category::new(name, weight, Vec::<String>::new())
}

fn new_lookup() -> CategoryLookup {
    let mut lookup = CategoryLookup::new();

    lookup.add(newcat("Pushups", 1.0)).unwrap();
    lookup.add(newcat("Burpees", 1.5)).unwrap();
    lookup.add(newcat("Steps", 0.01)).unwrap();

    lookup
}
