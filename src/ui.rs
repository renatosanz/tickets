use crate::models::ticket::Ticket;

fn term_w() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}

pub fn render_list(tickets: &[Ticket]) {
    let w = term_w();
    println!("┌{}┐", "─".repeat(w.saturating_sub(2)));
    println!("│ {:<width$} │", "TICKETS LIST", width = w.saturating_sub(4));
    println!("├{}┤", "─".repeat(w.saturating_sub(2)));
    println!("│ {:08} │ {:20} │ {:12} │ {:20} │ {:10} │", "ID", "TITLE", "STATUS", "DESC", "DATE");
    println!("├{}┤", "─".repeat(w.saturating_sub(2)));
    for t in tickets {
        println!("│ {:08x} │ {:20} │ {:12} │ {:20} │ {:10} │",
            t.id, truncate(&t.title, 20), format!("{:?}", t.status), truncate(&t.description, 20), t.date.format("%Y-%m-%d"));
    }
    println!("└{}┘", "─".repeat(w.saturating_sub(2)));
}

pub fn render_detail(t: &Ticket) {
    let w = term_w();
    println!("╔{}╗", "═".repeat(w.saturating_sub(2)));
    println!("║ Ticket #{:08x} {:>width$} ║", t.id, "", width = w.saturating_sub(14));
    println!("╠{}╣", "═".repeat(w.saturating_sub(2)));
    println!("║ Title:    {:<width$} ║", t.title, width = w.saturating_sub(10));
    println!("║ Status:   {:>width$} ║", format!("{:?}", t.status), width = w.saturating_sub(10));
    println!("║ Date:     {:<width$} ║", t.date.format("%-d %B %Y %I:%M %p"), width = w.saturating_sub(10));
    println!("╠{}╣", "═".repeat(w.saturating_sub(2)));
    println!("║ Description:{:<width$} ║", "", width = w.saturating_sub(14));
    for line in t.description.lines().take(10) {
        println!("║   {:<width$} ║", truncate(line, w.saturating_sub(7)), width = w.saturating_sub(7));
    }
    println!("╚{}╝", "═".repeat(w.saturating_sub(2)));
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max { s[..max].to_string() + "…" } else { s.to_string() }
}
