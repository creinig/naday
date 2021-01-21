use crate::model::{Activity, Config};

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
