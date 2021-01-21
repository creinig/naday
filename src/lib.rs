mod cli;
mod error;
mod model;
mod storage;

use cli::CliAction;
use cli::RunContext;
use model::{Activity, Config};
use std::env;

pub fn cli_parse(args: env::Args) -> RunContext {
    RunContext::new(args).unwrap()
}

pub fn run(ctx: RunContext) -> Result<(), String> {
    match ctx.action {
        CliAction::Report => run_report(&ctx.config),
        CliAction::System => run_system(&ctx.config),
        CliAction::AddActivity {
            repetitions,
            category,
        } => run_add_activity(repetitions, category, &ctx.config),
    }
}

//
// Main Command handlers ----------------------------
//

fn run_report(_config: &Config) -> Result<(), String> {
    Err("'report' command not implemented yet".to_string())
}

fn run_system(_config: &Config) -> Result<(), String> {
    Err("'system' command not implemented yet".to_string())
}

fn run_add_activity(repetitions: u32, category: String, config: &Config) -> Result<(), String> {
    let activity = Activity::new(repetitions, &category);
    storage::store(&activity, config)
}

//
// Utility Functions ---------------------------------
//
