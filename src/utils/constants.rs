pub const HELP_MESSAGE: &str = "\
tickets 0.1.0

A simple CLI ticket manager.

USAGE:
    tickets <ACTION> [<args>] [OPTIONS]

ACTIONS:
    add, a          Create a new ticket (requires <title> and <description>)
    list, l         List all tickets
    delete, d       Delete a ticket (requires <ticket_id>)
    setstatus, set  Change a ticket's status (requires <ticket_id> <status>)
    getdetail, get  Show detailed info for a ticket (requires <ticket_id>)

STATUS VALUES:
    open, closed, blocked, inprogress

OPTIONS:
    -f, --file <PATH>  Use a custom database file instead of the default \"tickets.db\"
    -v, --verbose      Enable verbose/debug logging
    -h, --help         Print this help message

EXAMPLES:
    tickets add \"Fix login bug\" \"The login button doesn't respond\"
    tickets list
    tickets getdetail a1b2c3d4
    tickets setstatus a1b2c3d4 closed
    tickets delete a1b2c3d4
    tickets list -f my_tickets.db";
