use std::{
    error::Error,
    fmt::Display,
    io::ErrorKind,
    num::{IntErrorKind, ParseIntError},
};

use crate::utils::constants::HELP_MESSAGE;

#[derive(Debug)]
pub enum Errors {
    FileNotFound(String),
    UnknownAction,
    TicketNotFound,
    MissingParameter,
    BadParameter(String),
    HelpNeeded,
    IOError(String),
    InvalidFormat,
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::UnknownAction => {
                write!(f, "Error: unknown action name")
            }
            Errors::TicketNotFound => {
                write!(f, "Ticket id not found")
            }
            Errors::MissingParameter => {
                write!(f, "Error: missing parameter, use -h for help")
            }
            Errors::FileNotFound(path) => {
                write!(f, "The file at '{}' could not be found", path)
            }
            Errors::HelpNeeded => {
                write!(f, "{}", HELP_MESSAGE)
            }
            Errors::IOError(e) => {
                log::debug!("Error while accessing file IO: {}", e);
                write!(f, "Something went wrong while accessing db")
            }
            Errors::BadParameter(s) => {
                write!(f, "Bad parameter: {}", s)
            }
            Errors::InvalidFormat => {
                write!(f, "Invalid formatted data received from database")
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
