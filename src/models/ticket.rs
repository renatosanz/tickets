use std::{
    fmt::{Display, format},
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::Errors;
use chrono::{DateTime, Utc};
use sqlx::{SqlitePool, Type, prelude::FromRow};

#[derive(Debug, PartialEq, Clone, Copy, Type)]
#[sqlx(rename_all = "lowercase")]
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
            _ => Err(Errors::BadParameter(format!(
                "[ticket_status] invalid value '{s}' for ticket status, use -h for help"
            ))),
        }
    }
}

#[derive(FromRow)]
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

    pub async fn create_table(pool: &SqlitePool) -> Result<(), Errors> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS tickets  (
            id SERIAL PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            date DATETIME NOT NULL,
            status TEXT NOT NULL
        );",
        )
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Error creating tickets db scheme: {e}");
            Errors::IOError("Error creating tickets db scheme".to_string())
        })?;

        Ok(())
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

    pub fn show_detailed(self) -> String {
        let w: usize = 56;
        let border = "─".repeat(w);

        let date_str = self.date.format("%-d of %B of %Y at %-I:%M %p").to_string();
        let status_str = format!("{:?}", self.status);
        let id_str = format!("{:x}", self.id);

        let mut s = String::new();

        s.push_str(&format!(" ┌{}┐\n", border));
        s.push_str(&format!(" │ Ticket #{:<width$}│\n", id_str, width = w - 9));
        s.push_str(&format!(" ├{}┤\n", border));
        s.push_str(&format!(
            " │ Title       {:<width$}│\n",
            self.title,
            width = w - 14
        ));
        s.push_str(&format!(
            " │ Status      {:<width$}│\n",
            status_str,
            width = w - 14
        ));
        s.push_str(&format!(
            " │ Date        {:<width$}│\n",
            date_str,
            width = w - 14
        ));
        s.push_str(&format!(" ├{}┤\n", border));
        s.push_str(&format!(" │ Description{:<width$}│\n", "", width = w - 13));
        for line in self.description.lines() {
            s.push_str(&format!(" │   {:<width$}│\n", line, width = w - 3));
        }
        s.push_str(&format!(" └{}┘", border));

        s
    }

    pub fn list_view(&self) -> String {
        format!(
            "{:08x}\t{}\t{:?}\t{}\t{}",
            self.id,
            self.title,
            self.status,
            self.description,
            self.date.to_rfc3339()
        )
    }

    pub async fn save(&self, pool: &SqlitePool) -> Result<(), Errors> {
        sqlx::query(
            "INSERT INTO tickets (id,title, description, date, status) VALUES (?,?, ?, ?, ?)",
        )
        .bind(&self.id)
        .bind(&self.title)
        .bind(&self.description)
        .bind(&self.date)
        .bind(&self.status)
        .execute(pool)
        .await
        .map_err(|e| {
            log::error!("Error creating tickets db scheme: {e}");
            Errors::IOError("Error creating tickets db scheme".to_string())
        })?;

        Ok(())
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Self>, Errors> {
        let tickets: Vec<Self> = sqlx::query_as("SELECT * FROM tickets ORDER BY id DESC")
            .fetch_all(pool)
            .await
            .map_err(|e| {
                log::error!("Error creating tickets db scheme: {e}");
                Errors::IOError("Error creating tickets db scheme".to_string())
            })?;

        Ok(tickets)
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
