use chrono::{DateTime, Local};
use std::fmt;
use std::fmt::Display;

mod category_lookup;

pub use category_lookup::CategoryLookup;

//
// Config -------------------------
//

#[derive(Debug)]
pub struct Config {
    pub data_dir: String,
}

//
// Activity -----------------------
//

#[derive(Debug)]
pub struct Activity {
    pub timestamp: DateTime<Local>,
    pub category: String,
    pub reps: u32,
}

impl Activity {
    pub fn new<S: AsRef<str>>(repetitions: u32, category: S) -> Activity {
        let now = Local::now();

        Activity {
            timestamp: now,
            reps: repetitions,
            category: category.as_ref().to_string(),
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

//
// Category ----------------------------
//

#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub aliases: Vec<String>,
    pub weight: f64,
}

impl Category {
    pub fn new<T: Display>(name: &str, weight: f64, aliases: Vec<T>) -> Category {
        Category {
            name: name.to_string(),
            weight,
            aliases: aliases.iter().map(|a| a.to_string()).collect(),
        }
    }

    fn all_names(&self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        result.push(&self.name);
        for alias in &self.aliases {
            result.push(&alias);
        }

        result
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Category ({}, {}, {:?})",
            self.name, self.weight, self.aliases
        )
    }
}
