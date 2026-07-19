use log;
use regex::Regex;
use std::{
    env,
    error::Error,
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, ErrorKind, Read, Write},
    num::{IntErrorKind, ParseIntError},
};

use crate::models::ticket::{Ticket, TicketStatus};

pub mod models;

const HELP_MESSAGE: &str = "\
tickets 0.1.0

A simple CLI ticket manager.

USAGE:
    tickets <ACTION> [<title> <description>] [OPTIONS]

ACTIONS:
    add, a          Create a new ticket (requires <title> and <description>)
    list, l         List all tickets
    delete, d       Delete a ticket (not yet implemented)
    setstatus, set  Change a ticket's status (not yet implemented)

OPTIONS:
    -f, --file <PATH>  Use a custom database file instead of the default \"tickets.db\"
    -v, --verbose      Enable verbose/debug logging
    -h, --help         Print this help message

EXAMPLES:
    tickets add \"Fix login bug\" \"The login button doesn't respond\"
    tickets list
    tickets -f my_tickets.db list";

#[derive(Debug, Clone, Copy)]
enum Action {
    Add,
    Delete,
    List,
    SetStatus,
    GetDetail,
}

impl Action {
    fn is_valid(s: &String) -> Result<Action, Errors> {
        match s.to_lowercase().as_str() {
            "add" | "a" => Ok(Action::Add),
            "delete" | "d" => Ok(Action::Delete),
            "list" | "l" => Ok(Action::List),
            "setstatus" | "set" => Ok(Action::SetStatus),
            "getdetail" | "get" => Ok(Action::GetDetail),
            _ => Err(Errors::UnknowAction),
        }
    }
}

#[derive(Debug)]
pub enum Errors {
    FileNotFound(String),
    UnknowAction,
    TicketNotFound,
    MissingParameter,
    BadParammeter(String),
    HelpNeeded,
    NotYetImplemented(String),
    IOError(String),
    InvalidFormat,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::UnknowAction => {
                write!(f, "Error: Unknow action name")
            }
            Errors::TicketNotFound => {
                write!(f, "Ticket id not found")
            }
            Errors::MissingParameter => {
                write!(f, "Error: Missing parameter, pls see -help")
            }
            Errors::FileNotFound(path) => {
                write!(f, "The file at '{}' could not be found", path)
            }
            Errors::HelpNeeded => {
                write!(f, "{}", HELP_MESSAGE)
            }
            Errors::NotYetImplemented(action) => {
                write!(f, "Error: '{}' action is not yet implemented", action)
            }
            Errors::IOError(e) => {
                log::debug!("Error while accessing file IO: {}", e);
                write!(f, "Something went wrong while accessing db")
            }
            Errors::BadParammeter(s) => {
                write!(f, "Bad parameter: {}", s)
            }
            Errors::InvalidFormat => {
                write!(f, "Invalid formated data recived from db!")
            }
        }
    }
}

impl Error for Errors {}

impl From<std::io::Error> for Errors {
    fn from(value: std::io::Error) -> Self {
        log::error!("IO Error: {}", value.to_string());
        match value.kind() {
            ErrorKind::NotFound => {
                Errors::FileNotFound("The requested file was not found".to_string())
            }
            _ => Errors::IOError(value.to_string()),
        }
    }
}

impl From<ParseIntError> for Errors {
    fn from(value: ParseIntError) -> Self {
        log::error!("Parse Error: {}", value.to_string());
        match value.kind() {
            IntErrorKind::InvalidDigit => Errors::InvalidFormat,
            _ => Errors::IOError(value.to_string()),
        }
    }
}

struct State {
    action: Option<Action>,
    default_filepath: String,
    custom_filepath: Option<String>,
    verbose_mode: bool,
}

impl State {
    fn default() -> Self {
        State {
            action: None,
            default_filepath: String::from("tickets.db"),
            custom_filepath: None,
            verbose_mode: false,
        }
    }
    fn execute(&self, args: &Vec<String>) -> Result<(), Errors> {
        log::debug!("executing action: {:?}", &self.action.unwrap());

        let path = self
            .custom_filepath
            .as_ref()
            .unwrap_or(&self.default_filepath);

        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .write(true)
            .open(path)
            .or_else(|_| {
                log::debug!("Creating db file {}", path);
                File::create_new(path)
            })
            .map_err(|_| Errors::FileNotFound(path.clone()))?;

        match self.action.unwrap() {
            Action::Add => {
                let title = args.get(2).ok_or(Errors::MissingParameter)?;
                let description = args.get(3).ok_or(Errors::MissingParameter)?;
                log::debug!("adding ticket <title: {}, desc: {}>", &title, &description);

                let ticket = Ticket::new(title, description);
                writeln!(file, "{}", ticket).map_err(|e| Errors::IOError(e.to_string()))?;
                log::info!("Ticket {:x} created", ticket.id);
                Ok(())
            }
            Action::SetStatus => {
                let re = Regex::new(r"[0-9a-fA-F]+").unwrap();

                let ticket_id = args
                    .get(2)
                    .filter(|id| id.len() == 8)
                    .filter(|id| re.is_match(id))
                    .ok_or(Errors::BadParammeter(
                        "[ticket_id] must be a hexadecimal 8 chars".to_string(),
                    ))?
                    .to_string();

                let ticket_id_u32 = u32::from_str_radix(ticket_id.as_str(), 16)?;
                log::debug!("HEX: {} --- DEC U32: {}", ticket_id, ticket_id_u32);

                let new_status = args
                    .get(3)
                    .ok_or(Errors::MissingParameter)
                    .and_then(TicketStatus::is_valid)?;

                let reader = BufReader::new(file);
                let temp_path = format!("{}.tmp", path);
                let mut temp = fs::File::create(&temp_path)?;
                let mut any_changes = false;

                for (i, line) in reader.lines().enumerate() {
                    let line = line?;
                    let mut content = line.clone();

                    if line.contains(ticket_id_u32.to_string().as_str()) {
                        log::info!("Found '{}' on line {}: {}", ticket_id, i + 1, line.trim());
                        let mut ticket = Ticket::new_from_string(line.as_str())?;
                        ticket.status = new_status;
                        content = ticket.to_string();
                        any_changes = true
                    }
                    writeln!(temp, "{}", content).map_err(|e| Errors::IOError(e.to_string()))?;
                }

                if !any_changes {
                    return Err(Errors::TicketNotFound);
                }

                fs::rename(temp_path, path)?;

                Ok(())
            }
            Action::GetDetail => Err(Errors::NotYetImplemented("getdetail".to_string())),
            Action::Delete => Err(Errors::NotYetImplemented("delete".to_string())),
            Action::List => {
                log::debug!("listing all tickets <>");
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .map_err(|e| Errors::IOError(e.to_string()))?;

                let ticket_list: Vec<Ticket> = contents
                    .split("\n")
                    .filter(|s| !s.is_empty())
                    .map(Ticket::new_from_string)
                    .collect::<Result<Vec<Ticket>, Errors>>()?;

                println!("  ALL TICKETS [{}]", ticket_list.len());
                println!("  id\ttitle\tstatus\tdescription\tdate");
                for ticket in ticket_list {
                    println!("  {}", ticket.list_view());
                }
                Ok(())
            }
        }
    }
}

fn main() {
    if let Err(error) = run_application() {
        eprintln!("[TICKETS] > {}", error);
        std::process::exit(1);
    }
}

fn run_application() -> Result<(), Errors> {
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

    state.execute(&args)?;

    Ok(())
}
