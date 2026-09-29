# tickets

A simple command-line ticket manager written in Rust.

## What is it for?

`tickets` is a small CLI tool for creating and managing tickets. A ticket is just a
record with a title, a description, a status, and a creation date. You can add new
tickets, list all of them, view a single ticket in detail, change a ticket's status,
and delete a ticket.

It is built for fun and to learn the Rust programming language. There is no grand
purpose, no real users, and no production constraints. It is a playground for
practicing Rust idioms, async runtime usage, SQL, error handling, and structured
programming.

## How it works

`tickets` is an async CLI application built on `tokio`. Every invocation performs
these steps:

1. Read the command-line arguments.
2. Validate the action name (for example `add`, `list`, `setstatus`) and any extra
   parameters (title, description, ticket id, status).
3. Optionally set up a custom database path or verbose logging.
4. Open a connection to a SQLite database file (defaults to `tickets.db` in the
   current directory). The file is created if it does not exist.
5. Make sure the `tickets` table exists.
6. Run the requested action against the database and print the result.

### Actions

| Action    | Alias | Description                                        |
|-----------|-------|----------------------------------------------------|
| `add`     | `a`   | Create a new ticket from a title and description   |
| `list`    | `l`   | List all tickets, newest first                     |
| `delete`  | `d`   | Delete a ticket by id                              |
| `setstatus` | `set` | Change a ticket's status                          |
| `getdetail` | `get` | Show full details for one ticket                 |

### Options

| Option          | Description                                              |
|-----------------|----------------------------------------------------------|
| `-f, --file`    | Use a custom database file instead of `tickets.db`       |
| `-v, --verbose` | Enable verbose / trace logging                          |
| `-h, --help`    | Print the built-in help message                          |

### Status values

A ticket can have one of these statuses:

- `open`
- `closed`
- `blocked`
- `inprogress`

### Ticket ids

Ticket ids are 32-bit hashes derived from the title, description, and creation
timestamp. They are displayed as 8-character lowercase hexadecimal strings. This
means two tickets created with identical data at the same time would collide, and
ids are not sequential.

### Database

Data is stored in SQLite through the `sqlx` crate. The schema is a single table:

| Column      | Type     | Notes                     |
|-------------|----------|---------------------------|
| `id`        | integer  | primary key               |
| `title`     | text     |                           |
| `description`| text    |                           |
| `date`      | datetime | creation timestamp (UTC)  |
| `status`    | text     | one of the status values  |

> NOTE: Next updates will implement multi-user handling, actions history and support 
for mysql/postgresql databases.

### Compilation

Requires a recent stable Rust toolchain (edition 2024).

```bash
cargo build --release
```

The binary will be at `target/release/tickets`.

### Usage examples

```bash
# create a ticket
tickets add "Fix login bug" "The login button does not respond"

# list all tickets
tickets list

# show a single ticket in detail
tickets getdetail a1b2c3d4

# change a ticket's status
tickets setstatus a1b2c3d4 closed

# delete a ticket
tickets delete a1b2c3d4

# use a custom database file
tickets list -f my_tickets.db

# enable verbose logging
tickets list -v
```

## Why?

The goal is not to build useful software but to learn Rust by doing. The project
intentionally touches a wide range of language and ecosystem features:

- **Memory and ownership**: `String`, `&str`, borrowing, and moving values around
  between functions.
- **Generics and traits**: methods like `Ticket::new` use the `Into` trait so both
  `&str` and `String` can be passed.
- **Enums and pattern matching**: `Action` and `TicketStatus` are enums, and the
  action dispatch in `state.rs` is a big `match`.
- **Error handling**: a custom `Errors` enum implements `Display`, `Error`, and
  `From` conversions so the `?` operator works across `std::io`, parsing, and sqlx
  failures.
- **Async programming**: the whole app runs on `tokio`; database calls are `async`.
- **SQL and a real database**: uses `sqlx` against SQLite with async queries,
  parameter binding, and runtime schema creation.
- **CLI argument parsing**: done by hand instead of with a parser crate, to practice
  iterating over `std::env::args`.
- **Logging**: uses `log` and `simple_logger`, toggled by a `--verbose` flag.
- **Date and time**: uses `chrono` for `DateTime<Utc>` timestamps and pretty printing.
- **Module organization**: the code is split into `models`, `utils`, `state`, and
  `errors` modules.

> PD: Around 1 year ago I tried to create a bigger project with Rust without all this 
> knowledge so i found really necesary to learn the basics to implement more complex 
logic and patterns, I work on CS area but it was fun and interesting use Rust and 
understand all its power. 

## License

MIT. This is a learning project. ༼ つ ╹ ╹ ༽つ
Any improvement, comment, fix or idea is well recieved.
Renato.
