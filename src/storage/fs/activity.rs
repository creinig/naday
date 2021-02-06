use crate::model::{Activity, Config};

use crate::error::ParseError;
use chrono::prelude::*;
use chrono::Duration;
use itertools::Itertools;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Store the given activity on the filesystem
pub fn store(activity: &Activity, config: &Config) -> Result<(), Box<dyn Error>> {
    let dir_path = Path::new(&config.data_dir);
    std::fs::create_dir_all(&dir_path)?;

    let file_path = path_for_date(&activity.timestamp.date(), config);

    let mut file: File = init_activity_file(&file_path)?;

    writeln!(
        &mut file,
        "{};{};{}",
        ts2str(activity.timestamp),
        activity.reps,
        activity.category
    )?;

    Ok(())
}

/// Read all activities for a given day
pub fn read_day(date: &Date<Local>, config: &Config) -> Result<Vec<Activity>, Box<dyn Error>> {
    read_days(date, date, config)
}

/// Read all activities for the days from "start" up to "end" (inclusive)
pub fn read_days(
    start: &Date<Local>,
    end: &Date<Local>,
    config: &Config,
) -> Result<Vec<Activity>, Box<dyn Error>> {
    if end < start {
        panic!("end is before start");
    }

    let mut paths = Vec::new();
    let mut day = *start;
    while day <= *end {
        println!("Adding day {}", &day);
        paths.push(path_for_date(&day, config));
        day = day + Duration::days(1);
    }

    let paths = paths.into_iter().unique();
    let mut activities = Vec::new();

    for path in paths {
        println!("Reading path {:?}", &path);
        let mut for_path = read_activities(&path)?;
        activities.append(&mut for_path);
    }

    let activities = activities
        .into_iter()
        .filter(|a| &a.timestamp.date() >= start && &a.timestamp.date() <= end)
        .collect();

    Ok(activities)
}

/// Read all activities for the past 'ndays' days (including today)
pub fn read_past_days(ndays: u32, config: &Config) -> Result<Vec<Activity>, Box<dyn Error>> {
    let today = Local::now().date();
    let start = today - Duration::days(ndays.into());
    read_days(&start, &today, config)
}

//
// Internals ------------------------------
//

const ACTIVITY_FILE_FORMAT: &str = "%Y-%m.txt";
const ACTIVITY_TS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const PREAMBLE_ACTIVITIES_V1: &str = "naday activities v1";

/// Get the path (fully qualified filename) of the file containing the activities of the given
/// date.
/// This does not check whether the file or its parent directories exist.
fn path_for_date(date: &Date<Local>, config: &Config) -> PathBuf {
    let filename = date.format(ACTIVITY_FILE_FORMAT).to_string();
    let file_path: PathBuf = [&config.data_dir, &filename].iter().collect();
    file_path
}

/// Open the activity file for the given timestamp.
/// If it doesn't exist, initialize it
fn init_activity_file(path: &Path) -> Result<File, Box<dyn Error>> {
    if path.exists() {
        return Ok(OpenOptions::new().append(true).open(path)?);
    }

    let mut file: File = OpenOptions::new().create(true).write(true).open(path)?;

    writeln!(
        &mut file,
        "\
{}
# List of recorded activities for the 'naday' tool (https://github.com/creinig/naday)
# Lines beginning with '#' are comments and are ignored by the tool
# The remaining lines are plain CSV, with one recorded activity per line.
# Separator character is ';', encoding is UTF-8.
# Columns: timestamp (local time zone) ; number of repetitions ; category (excercise)",
        PREAMBLE_ACTIVITIES_V1
    )?;

    Ok(file)
}

/// Read all activities in the given file
fn read_activities(file_path: &Path) -> Result<Vec<Activity>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut activities = Vec::new();

    let mut lines = contents.lines();
    if let Some(preamble) = lines.next() {
        if preamble.trim() != PREAMBLE_ACTIVITIES_V1 {
            return Err(Box::new(ParseError::new(
                "No valid preamble found - unable to determine file format",
            )));
        }
    } else {
        return Err(Box::new(ParseError::new("File seems to be empty")));
    }

    for line in lines {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        match parse_activity(line) {
            Ok(activity) => activities.push(activity),
            Err(msg) => eprintln!(
                "Skipping unreadable activity <{}> in {}: {}",
                line,
                file_path.to_str().unwrap(),
                msg
            ),
        }
    }

    Ok(activities)
}

/// parse a single line from an activity file into an Activity struct
fn parse_activity(line: &str) -> Result<Activity, String> {
    let mut parts = line.split(';');

    //let mut timestamp: DateTime<Local> = Local::now();
    let mut category: String = String::new();

    let timestamp = match parts.next() {
        Some(ts) => match str2ts(ts) {
            Ok(parsed) => parsed,
            Err(msg) => {
                let message = format!("timestamp <{}> could not be parsed: {}", ts, msg);
                return Err(message);
            }
        },
        None => return Err("No activity timestamp found".to_string()),
    };

    let reps = match parts.next() {
        Some(rep_str) => {
            if let Ok(parsed) = rep_str.trim().parse() {
                parsed
            } else {
                return Err("Repetitions can not be parsed as whole number".to_string());
            }
        }
        None => return Err("No repetitions found".to_string()),
    };

    if let Some(cat) = parts.next() {
        category = cat.trim().to_string();
    }

    Ok(Activity {
        timestamp,
        reps,
        category,
    })
}

/// convert activity timestamp to string
fn ts2str(timestamp: DateTime<Local>) -> String {
    timestamp.format(ACTIVITY_TS_FORMAT).to_string()
}

/// parse string as activity timestamp
fn str2ts(raw: &str) -> Result<DateTime<Local>, Box<dyn Error>> {
    Ok(Local.datetime_from_str(raw.trim(), ACTIVITY_TS_FORMAT)?)
}

//
// Tests ---------------------------------
//

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
        let cfg = cfg(&tmp_dir);
        let timestamp: DateTime<Local> = Local::now();

        let activity = Activity::new(34, "Pushups");

        store(&activity, &cfg)?;

        let filename = timestamp.format(ACTIVITY_FILE_FORMAT).to_string();
        let filepath = cfg.data_dir + &(std::path::MAIN_SEPARATOR.to_string()) + &filename;

        assert!(Path::new(&filepath).exists());

        Ok(())
    }

    #[test]
    fn parse_activity() {
        let activity = super::parse_activity("2020-12-13 05:43:25;12;Pushups").unwrap();
        assert_eq!(2020, activity.timestamp.year());
        assert_eq!(12, activity.timestamp.month());
        assert_eq!(13, activity.timestamp.day());
        assert_eq!(5, activity.timestamp.hour());
        assert_eq!(43, activity.timestamp.minute());
        assert_eq!(25, activity.timestamp.second());
        assert_eq!(12, activity.reps);
        assert_eq!("Pushups", activity.category);

        // Just testing that some cases are parsed at all for now
        super::parse_activity("2003-01-01 00:00:00 ; 1 ; Burpees").unwrap();
        super::parse_activity("2021-12-31 23:59:59 ; 435 ; Plank_Minutes").unwrap();
    }

    #[test]
    fn init_activity_file() {
        let tmp_dir = TempDir::new().unwrap();
        let cfg = cfg(&tmp_dir);
        let timestamp: DateTime<Local> = Local::now();
        let activity = Activity::new(13, "Burpees");

        let path = path_for_date(&timestamp.date(), &cfg);
        assert!(path.exists() == false);

        store(&activity, &cfg).unwrap();
        assert!(path.exists());

        let contents = fs::read_to_string(path).unwrap();
        assert_eq!(PREAMBLE_ACTIVITIES_V1, contents.lines().next().unwrap());
    }

    #[test]
    fn activity_roundtrip() -> Result<(), Box<dyn Error>> {
        let tmp_dir = TempDir::new()?;
        let cfg = cfg(&tmp_dir);
        let timestamp1 = str2ts("2020-12-13 14:34:53")?;
        let path = path_for_date(&timestamp1.date(), &cfg);

        println!("Target path = <{:?}>", &path);

        // initial state: nothing in the file
        read_activities(&path).expect_err("File should not exist yet");

        // store 1 activity and read it again
        let activity = Activity {
            timestamp: timestamp1,
            reps: 13,
            category: "Burpees".to_string(),
        };
        store(&activity, &cfg)?;

        let activities = read_activities(&path)?;
        assert_eq!(1, activities.len());
        assert_eq!(activity, activities[0]);

        // store another activity and read it again
        let timestamp2 = str2ts("2020-12-13 16:34:53")?;
        let activity = Activity {
            timestamp: timestamp2,
            reps: 20,
            category: "Situps".to_string(),
        };
        store(&activity, &cfg)?;

        let activities = read_activities(&path)?;
        assert_eq!(2, activities.len());

        assert_eq!(activity, activities[1]);

        Ok(())
    }

    #[test]
    fn multiple_months() {
        let tmp_dir = TempDir::new().unwrap();
        let cfg = cfg(&tmp_dir);
        let start_date = str2ts("2020-12-13 14:34:53").unwrap();
        let ndays = 200;

        for dayidx in 0..200 {
            let day = start_date + Duration::days(dayidx);
            let activity = Activity {
                timestamp: day,
                reps: dayidx as u32,
                category: "Pushups".to_string(),
            };
            store(&activity, &cfg).unwrap();
        }

        let windowsize = 10;
        for dayidx in 0..200 {
            let start = (start_date + Duration::days(dayidx)).date();
            let days_in_window = std::cmp::min(windowsize, ndays - dayidx);
            let end = start + Duration::days(days_in_window - 1);
            println!(
                "Test run with {} .. {}  ({} days)",
                &start, &end, days_in_window
            );

            let activities = read_days(&start, &end, &cfg).unwrap();
            assert_eq!(
                days_in_window as usize,
                activities.len(),
                "activities should contain exactly  one entry per day"
            );

            for idx in 0..days_in_window {
                assert_eq!(
                    (idx + dayidx) as u32,
                    activities.get(idx as usize).unwrap().reps
                );
            }
        }
    }

    fn cfg(tmp: &TempDir) -> Config {
        Config {
            data_dir: tmp.path().to_str().unwrap().to_string(),
        }
    }
}
