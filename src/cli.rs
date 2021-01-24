use crate::model::Config;
use directories::BaseDirs;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

lazy_static! {
    static ref ACTIVITY_PATTERN: Regex = Regex::new(r"^(\d+)([a-zA-Z_]\w*)$").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum CliAction {
    AddActivity { repetitions: u32, category: String },
    Report,
    System,
}

#[derive(Debug)]
pub struct RunContext {
    pub config: Config,
    pub action: CliAction,
}

impl RunContext {
    pub fn new<T>(mut args: T) -> Result<RunContext, String>
    where
        T: Iterator<Item = String>,
    {
        let homedir = BaseDirs::new().unwrap();
        let homedir = homedir.home_dir();

        let mut ctx = RunContext {
            config: Config {
                data_dir: homedir.join(".naday").to_str().unwrap().to_string(),
            },
            action: CliAction::System,
        };

        args.next();
        let arg = args.next();
        if arg.is_none() {
            show_help(Some(format!("Not enough arguments. Expected at least 2")))?;
        }

        let arg = arg.unwrap().to_lowercase().trim().to_string();

        if arg == "report" {
            ctx.action = CliAction::Report;
        } else if arg == "system" {
            ctx.action = CliAction::System;
        } else if ACTIVITY_PATTERN.is_match(&arg) {
            match parse_activity(&arg) {
                Ok(action) => ctx.action = action,
                Err(msg) => {
                    show_help(Some(format!(
                        "Unable to parse activity '{}': {}",
                        &arg, msg
                    )))?;
                }
            }
        } else {
            show_help(Some(format!("Cannot understand argument '{}'", arg)))?;
        }

        Ok(ctx)
    }
}

fn show_help(error_msg: Option<String>) -> Result<(), String> {
    if let Some(msg) = error_msg {
        eprintln!("Argument error: {}", msg);
        return Err(msg);
    }

    Err("Invocation failure".to_string())
    // TODO: implement
}

fn parse_activity(spec: &str) -> Result<CliAction, Box<dyn Error>> {
    let groups = ACTIVITY_PATTERN.captures(spec).unwrap(); // not perfect error handling, but we can only enter here if the regex matched
    let repetitions: u32 = groups.get(1).unwrap().as_str().parse()?;
    let category: String = groups.get(2).unwrap().as_str().to_string();

    Ok(CliAction::AddActivity {
        repetitions,
        category,
    })
}

//
// Tests ---------------------------------------------------------
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_commands() {
        let ctx = RunContext::new(build_args(vec!["system"]).into_iter());
        assert_eq!(CliAction::System, ctx.unwrap().action, "Falsch");

        let ctx = RunContext::new(build_args(vec!["Report"]).into_iter());
        assert_eq!(CliAction::Report, ctx.unwrap().action, "Falsch");
    }

    #[test]
    fn activities() {
        let ctx = RunContext::new(build_args(vec!["16pu"]).into_iter());
        assert_eq!(build_activity(16, "pu"), ctx.unwrap().action, "Falsch");

        let ctx = RunContext::new(build_args(vec!["23h2"]).into_iter());
        assert_eq!(build_activity(23, "h2"), ctx.unwrap().action, "Falsch");
    }

    fn build_args(raw: Vec<&str>) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        args.push("naday".to_string());

        for item in raw {
            args.push(item.to_string());
        }

        args
    }

    fn build_activity(repetitions: u32, category: &str) -> CliAction {
        CliAction::AddActivity {
            repetitions: repetitions,
            category: category.to_string(),
        }
    }
}
