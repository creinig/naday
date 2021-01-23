use crate::model::{Activity, Category, CategoryLookup, Config};

use chrono::prelude::*;
use std::error::Error;

mod activity;
mod category;

/// Store the given activity on the filesystem
pub fn store(activity: &Activity, config: &Config) -> Result<(), Box<dyn Error>> {
    activity::store(activity, config)
}

/// Read all activities for a given day
pub fn read_day(date: &Date<Local>, config: &Config) -> Result<Vec<Activity>, Box<dyn Error>> {
    activity::read_day(date, config)
}

/// Read all activities for today
pub fn read_today(config: &Config) -> Result<Vec<Activity>, Box<dyn Error>> {
    let now = Local::today();
    read_day(&now, config)
}

/// Read all categories and return a populated lookup structure
pub fn read_categories(cfg: &Config) -> Result<CategoryLookup, Box<dyn Error>> {
    let categories = category::read_categories(cfg)?;
    let mut lookup = CategoryLookup::new();

    for category in categories {
        lookup.add(category);
    }

    Ok(lookup)
}
