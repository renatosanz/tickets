use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{Errors, utils::validation};
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool, Type, prelude::FromRow};

#[derive(Debug, PartialEq, Clone, Copy, Type)]
#[sqlx(rename_all = "lowercase")]
pub enum TicketStatus {
    Open,
    Blocked,
    Closed,
    InProgress,
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
        log::trace!(
            "Ticket::new - title: {}, description: {}",
            title.into(),
            description.into()
        );
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
        log::trace!("Received raw_data:  {}", &raw_data.into());
        let binding = raw_data.into();
        let parts: Vec<&str> = binding.split(',').collect();
        log::trace!("Received: {} parts - {:?}", parts.len(), parts);

        let [id, title, status, description, date] =
            parts.try_into().map_err(|_| Errors::InvalidFormat)?;

        Ok(Ticket {
            id: id.parse().map_err(|_| Errors::InvalidFormat)?,
            title: title.to_string(),
            status: validation::ticket_status(&status.to_string())?,
            description: description.to_string(),
            date: date.parse().map_err(|_| Errors::InvalidFormat)?,
        })
    }

    pub async fn create_table(pool: &SqlitePool) -> Result<(), Errors> {
        log::trace!("creating tickets table schema");
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

    pub fn status(&mut self, status: TicketStatus) -> &Self {
        log::trace!("setting ticket {:x} status to {:?}", self.id, status);
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
        log::trace!("building detailed view for ticket {:x}", self.id);
        let w: usize = 56;
        let border = "─".repeat(w);

        let date_str = self.date.format("%-d of %B of %Y at %-I:%M %p").to_string();
        let status_str = format!("{:?}", self.status);
        let id_str = format!("{:08x}", self.id);

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
        log::trace!("building list view for ticket {:x}", self.id);
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
        log::trace!(
            "saving ticket - id: {:x}, title: {}, status: {:?}",
            self.id,
            self.title,
            self.status
        );
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
            let ticket_id = &self.id;
            log::error!("Error saving ticket with id: {ticket_id} - {e}");
            Errors::IOError(format!("Error saving ticket with id: {ticket_id}"))
        })?;

        Ok(())
    }

    pub async fn update(&self, pool: &SqlitePool) -> Result<(), Errors> {
        log::trace!(
            "updating ticket - id: {:x}, title: {}, status: {:?}",
            self.id,
            self.title,
            self.status
        );
        sqlx::query("UPDATE tickets SET title=?, description=?, date=?, status=? WHERE id=?")
            .bind(&self.title)
            .bind(&self.description)
            .bind(&self.date)
            .bind(&self.status)
            .bind(&self.id)
            .execute(pool)
            .await
            .map_err(|e| {
                let ticket_id = &self.id;
                log::error!("Error updating ticket with id: {ticket_id} - {e}");
                Errors::IOError(format!("Error updating ticket with id: {ticket_id}"))
            })?;

        Ok(())
    }

    pub async fn delete(&self, pool: &SqlitePool) -> Result<(), Errors> {
        log::trace!("deleting ticket {:x}", self.id);
        sqlx::query("DELETE FROM tickets WHERE id=?")
            .bind(&self.id)
            .execute(pool)
            .await
            .map_err(|e| {
                let ticket_id = &self.id;
                log::error!("Error deleting ticket with id: {ticket_id} - {e}");
                Errors::IOError(format!("Error deleting ticket with id: {ticket_id}"))
            })?;

        Ok(())
    }

    pub async fn find_one_by_id(pool: &SqlitePool, ticket_id: u32) -> Result<Ticket, Errors> {
        log::trace!("finding ticket by id: {:x}", ticket_id);
        let row =
            sqlx::query("SELECT id, title, description, date, status FROM tickets WHERE id=?")
                .bind(&ticket_id)
                .fetch_one(pool)
                .await
                .map_err(|e| {
                    log::error!("Error finding ticket with id: {ticket_id} - {e}");
                    Errors::IOError(format!("Error finding ticket with id: {ticket_id}"))
                })?;

        log::info!("Ticket found with id: {ticket_id} - {row:?}");

        let mut ticket = Ticket::default();
        ticket.id = row.get("id");
        ticket.title = row.get("title");
        ticket.description = row.get("description");
        ticket.date = row.get("date");
        ticket.status = row.get("status");

        Ok(ticket)
    }

    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Self>, Errors> {
        log::trace!("finding all tickets");
        let tickets: Vec<Self> = sqlx::query_as("SELECT * FROM tickets ORDER BY id DESC")
            .fetch_all(pool)
            .await
            .map_err(|e| {
                log::error!("Error fetching tickets from db: {e}");
                Errors::IOError("Error fetching tickets from db".to_string())
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
