mod cli;
mod error;
mod model;
mod report;
mod storage;

use cli::CliAction;
use cli::RunContext;
use model::{Activity, Config};
use std::env;
use std::process;

pub fn cli_parse(args: env::Args) -> RunContext {
    RunContext::new(args).unwrap()
}

pub fn run(ctx: RunContext) -> Result<(), String> {
    match ctx.action {
        CliAction::Report => report::today(&ctx.config),
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

fn run_system(config: &Config) -> Result<(), String> {
    let categories = storage::read_categories(config)?;

    println!("Storage directory: {}", &config.data_dir);
    println!("Known Categories:");
    for category in categories.iter() {
        println!(
            "  {} (weight {}), aliases {:?}",
            &category.name, &category.weight, &category.aliases
        );
    }

    Ok(())
}

fn run_add_activity(repetitions: u32, category: String, config: &Config) -> Result<(), String> {
    let categories = storage::read_categories(config)?;

    let category = match categories.find(&category) {
        Some(cat) => cat.name.to_string(),
        None => {
            eprintln!("Activity category '{}' is not known", category);
            process::exit(1);
        }
    };

    let activity = Activity::new(repetitions, &category);
    storage::store(&activity, config)?;

    println!("Added {} {}", repetitions, &category);
    report::today(config)?;
    Ok(())
}

//
// Utility Functions ---------------------------------
//
