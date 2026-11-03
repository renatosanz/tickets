use crate::models::ticket::Ticket;

fn term_w() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80)
}

pub fn render_list(tickets: &[Ticket]) {
    println!("{:>8} │ {:20} │ {:12} │ {:20} │ {:10}", "ID", "TITLE", "STATUS", "DESC", "DATE");
    println!("{}", "─".repeat(term_w()));
    for t in tickets {
        println!("{:08x} │ {:20} │ {:12} │ {:20} │ {:10}",
            t.id, truncate(&t.title, 20), format!("{:?}", t.status), truncate(&t.description, 20), t.date.format("%Y-%m-%d"));
    }
}

pub fn render_detail(t: &Ticket) {
    println!("Ticket #{}  │  Title: {}  │  Status: {}  │  Date: {}",
        format!("{:08x}", t.id),
        t.title,
        format!("{:?}", t.status),
        t.date.format("%-d %B %Y %I:%M %p"));
    println!("{}", "─".repeat(term_w()));
    println!("Description:");
    for line in t.description.lines().take(10) {
        println!("  {}", line);
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max { s[..max].to_string() + "…" } else { s.to_string() }
}
