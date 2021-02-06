mod common;
mod sliding;
mod today;

use crate::model::Config;
use chrono::Local;

pub fn today(config: &Config) -> Result<(), String> {
    today::run(config)
}

pub fn sliding_month(config: &Config) -> Result<(), String> {
    sliding::sliding_days(Local::now().date(), 31, config)
}
