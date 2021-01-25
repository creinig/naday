use crate::model::{Category, Config};

use crate::error::ParseError;
use std::error::Error;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub fn read_categories(cfg: &Config) -> Result<Vec<Category>, Box<dyn Error>> {
    let path = &(init_category_file(cfg)?);
    let contents = fs::read_to_string(path)?;

    let mut categories: Vec<Category> = Vec::new();
    let mut lines = contents.lines();

    if let Some(preamble) = lines.next() {
        if preamble.trim() != PREAMBLE_CATEGORIES_V1 {
            return Err(Box::new(ParseError::new(
                "No valid preamble found - unable to determine category file format",
            )));
        }
    } else {
        return Err(Box::new(ParseError::new("Category file seems to be empty")));
    }

    for line in lines {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        match parse_category(line) {
            Ok(category) => categories.push(category),
            Err(msg) => eprintln!(
                "Skipping unreadable category <{}> in {}: {}",
                line,
                path.to_str().unwrap(),
                msg
            ),
        }
    }

    Ok(categories)
}

//
// Internals ---------------------------
//

const PREAMBLE_CATEGORIES_V1: &str = "naday categories v1";

fn init_category_file(cfg: &Config) -> Result<PathBuf, Box<dyn Error>> {
    let mut path = PathBuf::from(&cfg.data_dir);
    std::fs::create_dir_all(&path)?;

    path.push("categories.txt");

    if !path.exists() {
        let mut file: File = OpenOptions::new().create(true).write(true).open(&path)?;

        writeln!(&mut file, "\
{}
# List of activity categories and their attributes for the 'naday' tool (https://github.com/creinig/naday).
# Lines beginning with '#' are comments and are ignored by the tool.
# The remaining lines are basically plain CSV, with one category per line.
# Separator character is ';', encoding is UTF-8.
# Columns: display name ; 'weight' of repetitions in relation to other activities [; alias]*
Pushups;1;pu;push
Situps;1;si
Burpees;1.5;bu
PlankSeconds;0.33;pl
WalkingSteps;0.01;wa
# General category for unplanned / one-off strenuous activity
Extra;1;x
", PREAMBLE_CATEGORIES_V1)?;
    }

    // return a readonly handle
    Ok(path)
}

fn parse_category(line: &str) -> Result<Category, String> {
    let mut parts = line.split(';');

    let name = match parts.next() {
        Some(name) => name.trim(),
        None => return Err("No category name found".to_string()),
    };

    let weight: f64 = match parts.next() {
        Some(weight) => match weight.parse() {
            Ok(val) => val,
            Err(error) => return Err(error.to_string()),
        },
        None => 1.0,
    };

    let mut aliases = Vec::new();
    for alias in parts {
        aliases.push(alias.trim());
    }

    Ok(Category::new(name, weight, aliases))
}

//
// Tests --------------------------------
//
#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Config;
    use tempfile::TempDir;

    #[test]
    fn file_init() {
        let tmp_dir = TempDir::new().unwrap();
        let cfg = cfg(&tmp_dir);

        let categories = read_categories(&cfg).unwrap();
        assert_eq!(6, categories.len());

        assert_eq!("Pushups", &(categories.get(0).unwrap().name));
        assert_eq!(1.0, categories.get(0).unwrap().weight);
        assert_eq!(
            "Category (Pushups, 1, [\"pu\", \"push\"])",
            categories.get(0).unwrap().to_string()
        );
        assert_eq!(
            "Category (Situps, 1, [\"si\"])",
            categories.get(1).unwrap().to_string()
        );
        assert_eq!(
            "Category (Burpees, 1.5, [\"bu\"])",
            categories.get(2).unwrap().to_string()
        );
        assert_eq!(
            "Category (PlankSeconds, 0.33, [\"pl\"])",
            categories.get(3).unwrap().to_string()
        );
    }

    fn cfg(tmp: &TempDir) -> Config {
        Config {
            data_dir: tmp.path().to_str().unwrap().to_string(),
        }
    }
}
