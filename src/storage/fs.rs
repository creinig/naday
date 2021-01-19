use crate::model::{Activity, Config};

use chrono::prelude::*;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn store(activity: &Activity, config: &Config) -> Result<(), Box<dyn Error>> {
    let dir_path = Path::new(&config.data_dir);
    std::fs::create_dir_all(&dir_path)?;

    let filename = Local::now().format("%Y-%m").to_string() + ".csv";
    let file_path: PathBuf = [&config.data_dir, &filename].iter().collect();
    let mut file: File = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    writeln!(
        &mut file,
        "{};{};{}",
        activity.timestamp, activity.reps, activity.category
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Activity, Config};
    use chrono::prelude::{DateTime, Local};
    use std::error::Error;
    use std::path::Path;
    use tempfile::TempDir;

    #[test]
    fn store_dirinit() -> Result<(), Box<dyn Error>> {
        let tmp_dir = TempDir::new()?;

        let cfg = Config {
            data_dir: tmp_dir.path().to_str().unwrap().to_string(),
        };

        let timestamp: DateTime<Local> = Local::now();

        let activity = Activity {
            timestamp: timestamp.timestamp() as u64,
            reps: 34,
            category: "Pushups".to_string(),
        };

        store(&activity, &cfg)?;

        let filename = timestamp.format("%Y-%m").to_string() + ".csv";
        let filepath = cfg.data_dir + &(std::path::MAIN_SEPARATOR.to_string()) + &filename;

        assert!(Path::new(&filepath).exists());

        Ok(())
    }
}
