use crate::model::{Activity, CategoryLookup, Config};
use chrono::{Date, Local};

mod fs;

//
// Main Interface -----------------------
//
pub fn store(activity: &Activity, config: &Config) -> Result<(), String> {
    match fs::store(activity, config) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("{:?}", error)),
    }
}

pub fn read_today(config: &Config) -> Result<Vec<Activity>, String> {
    match fs::read_today(config) {
        Ok(activities) => Ok(activities),
        Err(error) => Err(format!("{:?}", error)),
    }
}

pub fn read_days(
    start: &Date<Local>,
    end: &Date<Local>,
    config: &Config,
) -> Result<Vec<Activity>, String> {
    match fs::read_days(start, end, config) {
        Ok(activities) => Ok(activities),
        Err(error) => Err(format!("{:?}", error)),
    }
}

/// Read all categories and return a populated lookup structure
pub fn read_categories(cfg: &Config) -> Result<CategoryLookup, String> {
    match fs::read_categories(cfg) {
        Ok(lookup) => Ok(lookup),
        Err(error) => Err(format!("{:?}", error)),
    }
}
