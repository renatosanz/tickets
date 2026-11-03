use clap::{Parser, Subcommand};
use log;

use crate::{
    errors::Errors,
    models::ticket::TicketStatus,
    utils::{constants::HELP_MESSAGE, validation},
};

pub mod errors;
pub mod models;
pub mod state;
pub mod ui;
pub mod utils;

#[derive(Parser)]
#[command(name = "tickets", after_help = HELP_MESSAGE, version)]
struct Cli {
    #[command(subcommand)]
    action: ActionSpec,

    #[arg(short = 'f', long = "file", default_value = "tickets.db")]
    filepath: String,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Subcommand)]
enum ActionSpec {
    #[command(name = "add", alias = "a")]
    Add { title: String, description: String },
    #[command(name = "list", alias = "l")]
    List,
    #[command(name = "delete", alias = "d")]
    Delete { id: String },
    #[command(name = "setstatus", alias = "set")]
    SetStatus { id: String, status: TicketStatus },
    #[command(name = "getdetail", alias = "get")]
    GetDetail { id: String },
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => {}
        Err(error) => {
            eprintln!("[TICKETS] > {}", error);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), Errors> {
    let cli = Cli::parse();

    if cli.verbose {
        simple_logger::init_with_level(log::Level::Trace).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Error).unwrap();
    }

    let path = cli.filepath.clone();

    match cli.action {
        ActionSpec::Add { title, description } => {
            log::debug!("adding ticket <title: {}, desc: {}>", &title, &description);
            state::run_add(&path, &title, &description).await?;
        }
        ActionSpec::List => {
            state::run_list(&path).await?;
        }
        ActionSpec::Delete { id } => {
            let ticket_id = validation::ticket_id(&id)?;
            state::run_delete(&path, ticket_id).await?;
        }
        ActionSpec::SetStatus { id, status } => {
            let ticket_id = validation::ticket_id(&id)?;
            state::run_setstatus(&path, ticket_id, status).await?;
        }
        ActionSpec::GetDetail { id } => {
            let ticket_id = validation::ticket_id(&id)?;
            state::run_getdetail(&path, ticket_id).await?;
        }
    }

    Ok(())
}