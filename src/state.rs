use sqlx::sqlite::SqlitePoolOptions;

use crate::{errors::Errors, models::ticket::Ticket};

pub async fn connect(path: &str) -> Result<sqlx::SqlitePool, Errors> {
    let db_url = format!("sqlite:./{}?mode=rwc", path);
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(db_url.as_str())
        .await
        .map_err(|e| {
            log::error!("error accessing db {db_url}: {e}");
            Errors::FileNotFound(format!("the database file '{}' could not be found", path))
        })
}

pub async fn run_add(path: &str, title: &str, description: &str) -> Result<(), Errors> {
    let pool = connect(path).await?;
    Ticket::create_table(&pool).await?;
    let ticket = Ticket::new(title, description);
    ticket.save(&pool).await?;
    log::info!("Ticket {:x} created", ticket.id);
    Ok(())
}

pub async fn run_list(path: &str) -> Result<(), Errors> {
    let pool = connect(path).await?;
    Ticket::create_table(&pool).await?;
    let tickets = Ticket::find_all(&pool).await?;
    crate::ui::render_list(&tickets);
    Ok(())
}

pub async fn run_delete(path: &str, id: u32) -> Result<(), Errors> {
    let pool = connect(path).await?;
    Ticket::create_table(&pool).await?;
    let ticket = Ticket::find_one_by_id(&pool, id).await?;
    ticket.delete(&pool).await?;
    Ok(())
}

pub async fn run_getdetail(path: &str, id: u32) -> Result<(), Errors> {
    let pool = connect(path).await?;
    Ticket::create_table(&pool).await?;
    let ticket = Ticket::find_one_by_id(&pool, id).await?;
    crate::ui::render_detail(&ticket);
    Ok(())
}

pub async fn run_setstatus(
    path: &str,
    id: u32,
    new_status: crate::models::ticket::TicketStatus,
) -> Result<(), Errors> {
    let pool = connect(path).await?;
    Ticket::create_table(&pool).await?;
    let mut ticket = Ticket::find_one_by_id(&pool, id).await?;
    ticket.status(new_status);
    ticket.update(&pool).await?;
    Ok(())
}