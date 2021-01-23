use crate::model::{Activity, CategoryLookup, Config};

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

/// Read all categories and return a populated lookup structure
pub fn read_categories(cfg: &Config) -> Result<CategoryLookup, String> {
    match fs::read_categories(cfg) {
        Ok(lookup) => Ok(lookup),
        Err(error) => Err(format!("{:?}", error)),
    }
}
