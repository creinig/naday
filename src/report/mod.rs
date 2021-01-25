mod common;
mod today;

use crate::model::Config;

pub fn today(config: &Config) -> Result<(), String> {
    today::run(config)
}
