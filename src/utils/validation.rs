use regex::Regex;

use crate::{
    errors::Errors,
    models::ticket::TicketStatus,
};

pub fn ticket_status(raw: &str) -> Result<TicketStatus, Errors> {
    log::trace!("validating ticket status input: [{}]", raw);
    match raw.to_lowercase().as_str() {
        "open" => Ok(TicketStatus::Open),
        "blocked" => Ok(TicketStatus::Blocked),
        "closed" => Ok(TicketStatus::Closed),
        "inprogress" => Ok(TicketStatus::InProgress),
        _ => Err(Errors::BadParameter(format!(
            "[ticket_status] invalid value '{raw}' for ticket status, use -h for help"
        ))),
    }
}

pub fn ticket_id(raw: &str) -> Result<u32, Errors> {
    log::trace!("validating ticket id input: [{}]", raw);

    let re = Regex::new(r"^[0-9a-fA-F]{8}$").unwrap();
    if !re.is_match(raw) {
        return Err(Errors::BadParameter(
            "[ticket_id] must be a hexadecimal 8 chars".to_string(),
        ));
    }

    let ticket_id = u32::from_str_radix(raw, 16)?;
    log::trace!("valid ticket id - HEX: {} DEC U32: {}", raw, ticket_id);
    Ok(ticket_id)
}