mod cli;
mod error;
mod model;
mod storage;

use cli::CliAction;
use cli::RunContext;
use model::{Activity, Category, CategoryLookup, Config};
use std::collections::HashMap;
use std::env;
use std::process;

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

fn run_report(config: &Config) -> Result<(), String> {
    let activities = storage::read_today(config)?;
    let mut by_category = HashMap::new();

    for activity in activities {
        let cat = (&activity.category).to_string();

        let reps = if by_category.contains_key(&cat) {
            by_category.get(&cat).unwrap() + activity.reps
        } else {
            activity.reps
        };

        by_category.insert(cat, reps);
    }

    println!("\nStats for today:");
    for (category, reps) in by_category {
        println!("  {}: {} repetitions", category, reps);
    }

    Ok(())
}

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
    run_report(config)?;
    Ok(())
}

//
// Utility Functions ---------------------------------
//
