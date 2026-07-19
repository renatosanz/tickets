use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

use chrono::{DateTime, Utc};

use crate::Errors;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TicketStatus {
    Open,
    Blocked,
    Closed,
    InProgress,
}

impl TicketStatus {
    pub fn is_valid(s: &String) -> Result<TicketStatus, Errors> {
        log::debug!("validating TicketStatus: {}", s);
        match s.to_lowercase().as_str() {
            "open" => Ok(TicketStatus::Open),
            "blocked" => Ok(TicketStatus::Blocked),
            "closed" => Ok(TicketStatus::Closed),
            "inprogress" => Ok(TicketStatus::InProgress),
            _ => Err(Errors::BadParammeter(format!(
                "[ticket_status] invalid value '{s}' for ticket status\npls see -h / --help"
            ))),
        }
    }
}

pub struct Ticket {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub status: TicketStatus,
}

impl Ticket {
    // static method (dont take self as first parm)
    pub fn new<S: Into<String> + Copy>(title: S, description: S) -> Ticket {
        let mut hasher = DefaultHasher::new();
        let current_date = Utc::now();
        let unique_str = format!(
            "{}-{}-{}",
            title.into(),
            description.into(),
            current_date.to_string()
        );
        unique_str.hash(&mut hasher);
        let hash_u32 = (hasher.finish() & u32::MAX as u64) as u32;
        Ticket {
            id: hash_u32,
            title: title.into(),
            description: description.into(),
            status: TicketStatus::Open,
            date: current_date,
        }
    }

    pub fn new_from_string<S: Into<String> + Copy>(raw_data: S) -> Result<Ticket, Errors> {
        log::debug!("Received raw_data:  {}", &raw_data.into());
        let binding = raw_data.into();
        let parts: Vec<&str> = binding.split(',').collect();
        log::debug!("Received: {} parts - {:?}", parts.len(), parts);

        let [id, title, status, description, date] =
            parts.try_into().map_err(|_| Errors::InvalidFormat)?;

        Ok(Ticket {
            id: id.parse().map_err(|_| Errors::InvalidFormat)?,
            title: title.to_string(),
            status: TicketStatus::is_valid(&status.to_string())?,
            description: description.to_string(),
            date: date.parse().map_err(|_| Errors::InvalidFormat)?,
        })
    }

    pub fn status(mut self, status: TicketStatus) -> Self {
        self.status = status;
        self
    }

    pub fn default() -> Ticket {
        Ticket::new("blank", "no description")
    }
    //struct method
    pub fn is_open(self) -> bool {
        self.status == TicketStatus::Open
    }

    pub fn list_view(&self) -> String {
        format!(
            "{:x}\t{}\t{:?}\t{}\t{}",
            self.id,
            self.title,
            self.status,
            self.description,
            self.date.to_rfc3339()
        )
    }
}

impl Display for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{:?},{},{}",
            self.id,
            self.title,
            self.status,
            self.description,
            self.date.to_rfc3339()
        )
    }
}
