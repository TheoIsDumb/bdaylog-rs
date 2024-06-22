use std::io::{self, Write};
use std::path::Path;
use rusqlite::{Connection, Result};

pub fn help() {
    println!("\x1B[2m{}\x1B[0m\n", "bdaylog");

    println!("\x1B[4m\x1B[2m{}\x1B[0m", "available commands");
    println!("list - lists all birthdays");
    println!("add - add new birthday");
    println!("update - update birthday details");
    println!("delete - delete a birthday");

    println!("search - search a birthday");
    println!("help - print this text");
}

// check if table exists
pub fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}';",
        table_name
    );

    let mut stmt = conn.prepare(&query)?;
    let exists = stmt.exists([])?;

    Ok(exists)
}

// reusable function to get user input
pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

// print table header
pub fn print_header() {
    println!(
        "\x1b[1m{0: <10} {1: <10} {2: <10}\x1b[0m",
        "ID", "NAME", "BIRTHDAY"
    );
}

// print table row
pub fn print_row(id: i32, name: String, date: String) {
    println!("{0: <10} {1: <10} {2: <10}", id, name, date);
}

// init - checks if directory, db and table exist
pub fn init() -> Result<Connection, rusqlite::Error> {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    let db_dir_path = home + "/.config/bdaylog/";
    let db_path = db_dir_path.clone() + "data.db";

    if !Path::new(&db_dir_path).exists() {
        std::fs::create_dir(&db_dir_path).unwrap();
        println!("created directory.");
    }

    let conn = Connection::open(&db_path)?;
    let exists = table_exists(&conn, "bdays")?;

    if !exists {
        conn.execute(
            "CREATE TABLE bdays (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            date  TEXT NOT NULL
        )",
            (),
        )?;
        println!("created table.");
    }

    Ok(conn)
}