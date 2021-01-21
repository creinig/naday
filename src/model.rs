use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Config {
    pub data_dir: String,
}

#[derive(Debug)]
pub struct Activity {
    pub timestamp: DateTime<Local>,
    pub category: String,
    pub reps: u32,
}

impl Activity {
    pub fn new(repetitions: u32, category: &str) -> Activity {
        let now = Local::now();

        Activity {
            timestamp: now,
            reps: repetitions,
            category: category.to_string(),
        }
    }
}

impl PartialEq for Activity {
    fn eq(&self, other: &Self) -> bool {
        (self.timestamp == other.timestamp)
            && (self.category == other.category)
            && (self.reps == other.reps)
    }
}
