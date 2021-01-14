use lazy_static::lazy_static;
use regex::Regex;
use std::process;
use std::error::Error;

lazy_static! {
    static ref ACTIVITY_PATTERN: Regex = Regex::new(r"^(\d+)([a-zA-Z]+)$").unwrap();
}

#[derive(Debug)]
pub struct Config {
    pub data_dir: String,
}

#[derive(Debug)]
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
    pub fn new(args: &[String]) -> RunContext {
        let mut ctx = RunContext {
            config: Config {
                data_dir: "~/.naday".to_string(),
            },
            action: CliAction::System,
        };

        if args.len() < 2 {
            show_help(None);
            process::exit(1);
        }

        let arg = args[1].to_lowercase().trim().to_string();

        if arg == "report" {
            ctx.action = CliAction::Report;
        } else if arg == "system" {
            ctx.action = CliAction::System;
        } else if ACTIVITY_PATTERN.is_match(&arg) {
            match parse_activity(&arg) {
                Ok(action) => ctx.action = action,
                Err(msg) => {
                    show_help(Some(format!("Unable to parse activity '{}': {}", &arg, msg)));
                    process::exit(1)
                }
            }
        } else {
            show_help(Some(format!("Cannot understand argument '{}'", arg)));
            process::exit(1);
        }

        ctx
    }
}

fn show_help(error_msg: Option<String>) -> () {
    if let Some(msg) = error_msg {
        println!("Argument error: {}", msg);
    }

    // TODO: implement
}

fn parse_activity(spec: &String) -> Result<CliAction, Box<dyn Error>> {
    let groups = ACTIVITY_PATTERN.captures(spec).unwrap(); // not perfect error handling, but we can only enter here if the regex matched
    let repetitions: u32 = groups.get(1).unwrap().as_str().parse()?;
    let category: String = groups.get(2).unwrap().as_str().to_string();

    Ok(CliAction::AddActivity {
        repetitions: repetitions,
        category: category
    }) 
}
