use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read},
};

use regex::Regex;
use sqlx::sqlite::SqlitePoolOptions;
use std::io::Write;

use crate::{errors::Errors, models::ticket::Ticket};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Add,
    Delete,
    List,
    SetStatus,
    GetDetail,
}

impl Action {
    pub fn is_valid(s: &String) -> Result<Action, Errors> {
        match s.to_lowercase().as_str() {
            "add" | "a" => Ok(Action::Add),
            "delete" | "d" => Ok(Action::Delete),
            "list" | "l" => Ok(Action::List),
            "setstatus" | "set" => Ok(Action::SetStatus),
            "getdetail" | "get" => Ok(Action::GetDetail),
            _ => Err(Errors::UnknownAction),
        }
    }
}

pub struct State {
    pub action: Option<Action>,
    pub default_filepath: String,
    pub custom_filepath: Option<String>,
    pub verbose_mode: bool,
}

impl State {
    pub fn default() -> Self {
        State {
            action: None,
            default_filepath: String::from("tickets.db"),
            custom_filepath: None,
            verbose_mode: false,
        }
    }
    pub async fn execute(&self, args: &Vec<String>) -> Result<(), Errors> {
        log::debug!("executing action: {:?}", &self.action.unwrap());

        let path = self
            .custom_filepath
            .as_ref()
            .unwrap_or(&self.default_filepath);

        let db_url = format!("sqlite:./{}?mode=rwc", path);

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(db_url.as_str())
            .await
            .map_err(|e| {
                log::error!("error accessing db {db_url}: {e}");
                Errors::FileNotFound(path.to_owned())
            })?;

        Ticket::create_table(&pool).await?;

        match self.action.unwrap() {
            Action::Add => {
                let title = args.get(2).ok_or(Errors::MissingParameter)?;
                let description = args.get(3).ok_or(Errors::MissingParameter)?;
                log::debug!("adding ticket <title: {}, desc: {}>", &title, &description);

                let ticket = Ticket::new(title, description);
                ticket.save(&pool).await?;
                log::info!("Ticket {:x} created", ticket.id);
                Ok(())
            }
            Action::SetStatus => {
                // let re = Regex::new(r"[0-9a-fA-F]+").unwrap();
                //
                // let ticket_id = args
                //     .get(2)
                //     .filter(|id| id.len() == 8)
                //     .filter(|id| re.is_match(id))
                //     .ok_or(Errors::BadParameter(
                //         "[ticket_id] must be a hexadecimal 8 chars".to_string(),
                //     ))?
                //     .to_string();
                //
                // let ticket_id_u32 = u32::from_str_radix(ticket_id.as_str(), 16)?;
                // log::debug!("HEX: {} --- DEC U32: {}", ticket_id, ticket_id_u32);
                //
                // let new_status = args
                //     .get(3)
                //     .ok_or(Errors::MissingParameter)
                //     .and_then(TicketStatus::is_valid)?;
                //
                // let reader = BufReader::new(file);
                // let temp_path = format!("{}.tmp", path);
                // let mut temp = fs::File::create(&temp_path)?;
                // let mut any_changes = false;
                //
                // for (i, line) in reader.lines().enumerate() {
                //     let line = line?;
                //     let mut content = line.clone();
                //
                //     if line.contains(ticket_id_u32.to_string().as_str()) {
                //         log::info!("Found '{}' on line {}: {}", ticket_id, i + 1, line.trim());
                //         let mut ticket = Ticket::new_from_string(line.as_str())?;
                //         ticket.status = new_status;
                //         content = ticket.to_string();
                //         any_changes = true
                //     }
                //     writeln!(temp, "{}", content).map_err(|e| Errors::IOError(e.to_string()))?;
                // }
                //
                // if !any_changes {
                //     fs::remove_file(temp_path)?;
                //     return Err(Errors::TicketNotFound);
                // }
                //
                // fs::rename(temp_path, path)?;
                //
                Ok(())
            }
            Action::GetDetail => {
                // let re = Regex::new(r"[0-9a-fA-F]+").unwrap();
                //
                // let ticket_id = args
                //     .get(2)
                //     .filter(|id| id.len() == 8)
                //     .filter(|id| re.is_match(id))
                //     .ok_or(Errors::BadParameter(
                //         "[ticket_id] must be a hexadecimal 8 chars".to_string(),
                //     ))?
                //     .to_string();
                //
                // let ticket_id_str = &u32::from_str_radix(ticket_id.as_str(), 16)?.to_string();
                // log::debug!("HEX: {} --- DEC U32: {}", ticket_id, ticket_id_str);
                //
                // let reader = BufReader::new(file);
                // let mut ticket: Option<Ticket> = None;
                //
                // for (i, line) in reader.lines().enumerate() {
                //     let line = line?;
                //
                //     if line.contains(ticket_id_str) {
                //         log::info!("Found '{}' on line {}: {}", ticket_id, i + 1, line.trim());
                //         ticket = Ticket::new_from_string(line.as_str()).ok();
                //     }
                // }
                //
                // if ticket.is_none() {
                //     return Err(Errors::TicketNotFound);
                // } else if let Some(data) = ticket {
                //     println!("{}", data.show_detailed());
                // }
                //
                Ok(())
            }
            Action::Delete => {
                // let re = Regex::new(r"[0-9a-fA-F]+").unwrap();
                //
                // let ticket_id = args
                //     .get(2)
                //     .filter(|id| id.len() == 8)
                //     .filter(|id| re.is_match(id))
                //     .ok_or(Errors::BadParameter(
                //         "[ticket_id] must be a hexadecimal 8 chars".to_string(),
                //     ))?
                //     .to_string();
                // let ticket_id_str = &u32::from_str_radix(ticket_id.as_str(), 16)?.to_string();
                // log::debug!("HEX: {} --- DEC U32: {}", ticket_id, ticket_id_str);
                //
                // let reader = BufReader::new(file);
                // let temp_path = format!("{}.tmp", path);
                // let mut temp = fs::File::create(&temp_path)?;
                // let mut any_changes = false;
                //
                // for (i, line) in reader.lines().enumerate() {
                //     let line = line?;
                //
                //     if line.contains(ticket_id_str) {
                //         log::info!("Found '{}' on line {}: {}", ticket_id, i + 1, line.trim());
                //         any_changes = true
                //     } else {
                //         writeln!(temp, "{}", line).map_err(|e| Errors::IOError(e.to_string()))?;
                //     }
                // }
                //
                // if !any_changes {
                //     fs::remove_file(temp_path)?;
                //     return Err(Errors::TicketNotFound);
                // }
                //
                // fs::rename(temp_path, path)?;
                //
                Ok(())
            }
            Action::List => {
                log::debug!("listing all tickets <>");
                let ticket_list = Ticket::find_all(&pool).await?;
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
