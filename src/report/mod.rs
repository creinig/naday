mod common;
mod sliding;
mod today;

#[cfg(test)]
mod test_common;

use crate::model::Config;
use chrono::Local;

pub fn today(config: &Config) -> Result<(), String> {
    today::run(config)
}

pub fn sliding_month(category: Option<String>, config: &Config) -> Result<(), String> {
    sliding::sliding_days(Local::now().date(), 31, category, config)
}

pub fn sliding_week(category: Option<String>, config: &Config) -> Result<(), String> {
    sliding::sliding_days(Local::now().date(), 7, category, config)
}
