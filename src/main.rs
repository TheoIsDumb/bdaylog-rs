use regex::Regex;
use rusqlite::{Connection, Result};
use std::io::{self, Write};
use std::path::Path;

// check if table exists
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}';",
        table_name
    );

    let mut stmt = conn.prepare(&query)?;
    let exists = stmt.exists([])?;

    Ok(exists)
}

// init - checks if directory, db and table exist
fn init() -> Result<Connection, rusqlite::Error> {
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

// add entries
fn add(conn: &Connection) -> Result<()> {
    let mut name = String::new();
    print!("Enter name: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let mut date = String::new();
    print!("Enter date (YYYY-MM-DD): ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");

    let re = Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").unwrap();

    if !re.is_match(date.as_str()) {
        eprintln!("Incorrect format. Enter date in YYYY-MM-DD format.");
        std::process::exit(1);
    }

    conn.execute(
        "INSERT INTO bdays (name, date)
        VALUES (?1, ?2)",
        (name.trim(), date.trim()),
    )?;

    Ok(())
}

// list entries
fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, date FROM bdays")?;

    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let date: String = row.get(2)?;

        Ok((id, name, date))
    })?;

    for row in rows {
        let (id, name, date) = row?;
        println!("{}. {} - {}", id, name, date);
    }

    Ok(())
}

// main
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let conn = init()?;

    let arg1 = match args.get(1) {
        Some(v) => v,
        None => "",
    };

    match arg1 {
        "list" => list(&conn)?,
        "add" => add(&conn)?,
        _ => list(&conn)?,
    }

    Ok(())
}
