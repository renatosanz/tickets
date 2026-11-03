use crate::models::ticket::Ticket;

pub fn render_list(tickets: &[Ticket]) {
    println!("┌{}┐", "─".repeat(70));
    println!("│ {:<68} │", "TICKETS LIST (textual UI)".to_string());
    println!("├┬{}┬{}┬{}┬{}┬{}┤", "─".repeat(8), "─".repeat(20), "─".repeat(12), "─".repeat(20), "─".repeat(10));
    println!("│ {:8} │ {:20} │ {:12} │ {:20} │ {:10} │", "ID", "TITLE", "STATUS", "DESC", "DATE");
    println!("├{}┤", "─".repeat(70));
    for t in tickets {
        println!("│ {:08x} │ {:20} │ {:12} │ {:20} │ {:10} │",
            t.id, truncate(&t.title, 20), format!("{:?}", t.status), truncate(&t.description, 20), t.date.format("%Y-%m-%d"));
    }
    println!("└{}┘", "─".repeat(70));
}

pub fn render_detail(t: &Ticket) {
    println!("╔{}╗", "═".repeat(56));
    println!("║ Ticket #{:08x} {:>35} ║", t.id, "");
    println!("╠{}╣", "═".repeat(56));
    println!("║ Title:    {:<45} ║", t.title);
    println!("║ Status:   {:>42} ║", format!("{:?}", t.status));
    println!("║ Date:     {:<45} ║", t.date.format("%-d %B %Y %I:%M %p"));
    println!("╠{}╣", "═".repeat(56));
    println!("║ Description:{:<38} ║", "");
    for line in t.description.lines().take(10) {
        println!("║   {:<52} ║", truncate(line, 52));
    }
    println!("╚{}╝", "═".repeat(56));
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max { s[..max].to_string() + "…" } else { s.to_string() }
}
