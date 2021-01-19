use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Config {
    pub data_dir: String,
}

#[derive(Debug)]
pub struct Activity {
    pub timestamp: u64,
    pub category: String,
    pub reps: u32,
}

impl Activity {
    pub fn new(repetitions: u32, category: String) -> Activity {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Activity {
            timestamp: now,
            reps: repetitions,
            category: category,
        }
    }
}
