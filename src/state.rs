use sqlx::sqlite::SqlitePoolOptions;

use crate::{errors::Errors, models::ticket::Ticket, utils::validation};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Add,
    Delete,
    List,
    SetStatus,
    GetDetail,
}

#[derive(Debug)]
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
        log::trace!("state to execute - {:?}", &self);

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
                log::trace!(
                    "add input data - title: {}, description: {}",
                    title,
                    description
                );

                let ticket = Ticket::new(title, description);
                ticket.save(&pool).await?;
                log::info!("Ticket {:x} created", ticket.id);
                Ok(())
            }
            Action::SetStatus => {
                let ticket_id_u32 =
                    validation::ticket_id(args.get(2).ok_or(Errors::MissingParameter)?)?;

                let new_status =
                    validation::ticket_status(args.get(3).ok_or(Errors::MissingParameter)?)?;

                let mut ticket = Ticket::find_one_by_id(&pool, ticket_id_u32).await?;
                ticket.status(new_status);
                ticket.update(&pool).await?;

                Ok(())
            }
            Action::GetDetail => {
                let ticket_id_u32 =
                    validation::ticket_id(args.get(2).ok_or(Errors::MissingParameter)?)?;

                let ticket = Ticket::find_one_by_id(&pool, ticket_id_u32).await?;
                println!("{}", ticket.show_detailed());
                Ok(())
            }
            Action::Delete => {
                let ticket_id_u32 =
                    validation::ticket_id(args.get(2).ok_or(Errors::MissingParameter)?)?;

                let ticket = Ticket::find_one_by_id(&pool, ticket_id_u32).await?;
                ticket.delete(&pool).await?;

                Ok(())
            }
            Action::List => {
                log::debug!("listing all tickets <>");
                log::trace!("fetching ticket list for display");
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
