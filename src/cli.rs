use crate::error::ParseError;
use crate::model::Config;
use clap::{arg_enum, crate_authors, crate_version, App, Arg};
use directories::BaseDirs;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::ffi::OsString;

lazy_static! {
    static ref ACTIVITY_PATTERN: Regex = Regex::new(r"^(\d+)([a-zA-Z_]\w*)$").unwrap();
    static ref REPORT_PATTERN: Regex = Regex::new(r"^[rR](-(\d+))?([dmwDMW])$").unwrap();
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
    pub fn new<T>(args: T) -> Result<RunContext, String>
    where
        T: Iterator<Item = String>,
    {
        let mut ctx = RunContext {
            config: Config {
                data_dir: default_data_dir(),
            },
            action: CliAction::System,
        };

        match parse_cmdline(args) {
            Ok(action) => {
                ctx.action = action;
                Ok(ctx)
            }
            Err(msg) => Err(format!("{:?}", msg).to_string()),
        }
    }
}

fn parse_activity(spec: &str) -> Result<CliAction, Box<dyn Error>> {
    let groups = match ACTIVITY_PATTERN.captures(spec) {
        Some(groups) => groups,
        None => {
            return Err(Box::new(ParseError::new("Unable to parse activity")));
        }
    };

    let repetitions: u32 = groups.get(1).unwrap().as_str().parse()?;
    let category: String = groups.get(2).unwrap().as_str().to_string();

    Ok(CliAction::AddActivity {
        repetitions,
        category,
    })
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum ReportType {
        Day,
        Week,
        Month
    }
}

fn setup_clap_app() -> App<'static, 'static> {
    App::new("naday")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A little tool for tracking (physical) excercise of the 'N repetitions a day' variant - 100 pushups per day, 10'000 steps per day etc.")
        .arg(Arg::from_usage("[SHORTHAND] 'Shorthand notation for the most common use cases'")
            .long_help(
"'18pu' is short for 'log 18pu'
'rd' is short for 'report --type=Day'
'r-2w' is short for 'report --type=Week --go-back=2")
            .conflicts_with_all(&["log", "system", "report"]))
        .subcommand(
            App::new("log").about("Log an activity")
                .arg(Arg::from_usage("[SPEC] 'Shorthand notation of the activity to log'"))
        )
        .subcommand(
            App::new("system").about("Get information on the tool's environment and settings")
        )
        .subcommand(
            App::new("report").about("Generate a report on logged activities")
                .arg(Arg::from_usage("-t, --type=[TYPE] 'The kind of report to generate'")
                    .possible_values(&ReportType::variants())
                    .case_insensitive(true)
                    .default_value("Day"))
                .arg(Arg::from_usage("-b, --go-back=[UNITS] 'report on the Nth previous day/week/month'"))

        )
}

fn parse_cmdline<I, T>(args: I) -> Result<CliAction, ()>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let app = setup_clap_app();

    let matches = match app.get_matches_from_safe(args) {
        Ok(m) => m,
        Err(error) => {
            eprintln!("{}", error);
            return Err(());
        }
    };

    if let Some(ref _report) = matches.subcommand_matches("report") {
        return Ok(CliAction::Report);
    } else if let Some(_system) = matches.subcommand_matches("system") {
        return Ok(CliAction::System);
    } else if let Some(log) = matches.subcommand_matches("log") {
        let spec = log.value_of("SPEC").unwrap(); // required parameter
        if let Ok(activity) = parse_activity(&spec) {
            return Ok(activity);
        } else {
            eprintln!("{}", log.usage());
            return Err(());
        }
    } else if let Some(shorthand) = matches.value_of("SHORTHAND") {
        return parse_shorthand(&shorthand);
    }

    Ok(CliAction::System)
}

fn parse_shorthand(spec: &str) -> Result<CliAction, ()> {
    if let Ok(activity) = parse_activity(spec) {
        Ok(activity)
    } else if let Ok(report) = parse_report(spec) {
        Ok(report)
    } else {
        eprintln!("Could not parse shorthand spec '{}'", spec);
        Err(())
    }
}

fn parse_report(spec: &str) -> Result<CliAction, Box<dyn Error>> {
    let _groups = match REPORT_PATTERN.captures(spec) {
        Some(groups) => groups,
        None => {
            return Err(Box::new(ParseError::new(
                "Unable to parse report shorthand",
            )))
        }
    };

    // TODO: support the other parts of the spec

    Ok(CliAction::Report)
}

fn default_data_dir() -> String {
    let homedir = BaseDirs::new().unwrap();
    let homedir = homedir.home_dir();

    homedir.join(".naday").to_str().unwrap().to_string()
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
        assert_eq!(CliAction::System, ctx.unwrap().action);

        let ctx = RunContext::new(build_args(vec!["report"]).into_iter());
        assert_eq!(CliAction::Report, ctx.unwrap().action);
    }

    #[test]
    fn activities() {
        let ctx = RunContext::new(build_args(vec!["16pu"]).into_iter());
        assert_eq!(build_activity(16, "pu"), ctx.unwrap().action);

        let ctx = RunContext::new(build_args(vec!["23h2"]).into_iter());
        assert_eq!(build_activity(23, "h2"), ctx.unwrap().action);
    }

    #[test]
    fn shorthand() {
        let ctx = RunContext::new(build_args(vec!["r-1d"]).into_iter());
        assert_eq!(CliAction::Report, ctx.unwrap().action);

        let ctx = RunContext::new(build_args(vec!["rd"]).into_iter());
        assert_eq!(CliAction::Report, ctx.unwrap().action);
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
