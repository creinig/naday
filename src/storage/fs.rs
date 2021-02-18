use crate::model::{Activity, CategoryLookup, Config};

use anyhow::{Context, Result};
use chrono::prelude::*;
use std::path::PathBuf;

mod activity;
mod category;

/// Store the given activity on the filesystem
pub fn store(activity: &Activity, config: &Config) -> Result<()> {
    activity::store(activity, config)
}

/// Read all activities for a given day
pub fn read_day(date: &Date<Local>, config: &Config) -> Result<Vec<Activity>> {
    activity::read_day(date, config)
}

/// Read all activities for today
pub fn read_today(config: &Config) -> Result<Vec<Activity>> {
    let now = Local::today();
    read_day(&now, config)
}

/// Read all activities for the given range of days (both ends inclusive)
pub fn read_days(start: &Date<Local>, end: &Date<Local>, config: &Config) -> Result<Vec<Activity>> {
    activity::read_days(start, end, config)
}

/// Read all categories and return a populated lookup structure
pub fn read_categories(cfg: &Config) -> Result<CategoryLookup> {
    let categories = category::read_categories(cfg)?;
    let mut lookup = CategoryLookup::new();

    for category in categories {
        lookup.add(category)?;
    }

    Ok(lookup)
}

//
// Internals --------------------------------------
//

fn init_data_dir(cfg: &Config) -> Result<PathBuf> {
    let path = PathBuf::from(&cfg.data_dir);
    std::fs::create_dir_all(&path)
        .with_context(|| format!("Could not create base directory {:?}", &path))?;

    Ok(path)
}
