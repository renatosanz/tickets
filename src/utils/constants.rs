pub const HELP_MESSAGE: &str = "\
tickets 0.1.0

A simple CLI ticket manager.

USAGE:
    tickets <ACTION> [<title> <description>] [OPTIONS]

ACTIONS:
    add, a          Create a new ticket (requires <title> and <description>)
    list, l         List all tickets
    delete, d       Delete a ticket (not yet implemented)
    setstatus, set  Change a ticket's status (not yet implemented)

OPTIONS:
    -f, --file <PATH>  Use a custom database file instead of the default \"tickets.db\"
    -v, --verbose      Enable verbose/debug logging
    -h, --help         Print this help message

EXAMPLES:
    tickets add \"Fix login bug\" \"The login button doesn't respond\"
    tickets list
    tickets -f my_tickets.db list";
