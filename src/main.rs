use log;
use std::env;

use crate::{
    errors::Errors,
    state::{Action, State},
};

pub mod errors;
pub mod models;
pub mod state;
pub mod utils;

#[tokio::main]
async fn main() {
    if let Err(error) = run_application().await {
        eprintln!("[TICKETS] > {}", error);
        std::process::exit(1);
    }
}

async fn run_application() -> Result<(), Errors> {
    let mut state = State::default();

    // ARGS
    // tickets [action] [title] [description] [--args]

    let args: Vec<String> = env::args().collect();

    let verbose = args
        .iter()
        .any(|a| matches!(a.to_lowercase().as_str(), "-v" | "--verbose"));
    if verbose {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    }

    log::debug!("Received {} arguments: {}", args.len(), args.join(","));

    if args.len() <= 1 {
        return Err(Errors::MissingParameter);
    }

    //validate if help or any action is recieved
    if let Some(action) = args.get(1) {
        log::debug!("Validating first arg: {}", action);
        if matches!(action.to_lowercase().as_str(), "--help" | "-h") {
            log::debug!("Help needed: {}", action);
            return Err(Errors::HelpNeeded);
        }
        state.action = Some(Action::is_valid(action)?);
    }

    for (idx, value) in args.iter().enumerate() {
        match value.to_lowercase().as_str() {
            "-f" | "--file" => {
                if let Some(next) = args.get(idx + 1) {
                    state.custom_filepath = Some(next.clone());
                }
            }
            "-v" | "--verbose" => state.verbose_mode = true,
            &_ => {}
        }
    }

    state.execute(&args).await?;

    Ok(())
}
