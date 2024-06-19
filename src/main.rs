use rusqlite::{Connection, Result};
use std::path::Path;

struct Bday {
    id: i32,
    name: String,
    date: String,
}
fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let query = format!(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='{}';",
        table_name
    );

    let mut stmt = conn.prepare(&query)?;
    let exists = stmt.exists([])?;

    Ok(exists)
}

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

fn add(conn: &Connection) -> Result<()> {
    let mut name = String::new();
    println!("Enter name: ");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let mut date = String::new();
    println!("Enter date: ");
    std::io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");

    conn.execute(
        "INSERT INTO bdays (name, date)
        VALUES (?1, ?2)",
        (name.trim(), date.trim()),
    )?;

    Ok(())
}

fn print_entries(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, date FROM bdays")?;
    let bday_iter = stmt.query_map([], |row| {
        Ok(Bday {
            id: row.get(0)?,
            name: row.get(1)?,
            date: row.get(2)?,
        })
    })?;

    for bday in bday_iter {
        match bday {
            Ok(bday) => println!("{}. {} - {}", bday.id, bday.name, bday.date),
            Err(err) => println!("{}", err),
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let conn = init()?;

    let arg1 = match args.get(1) {
        Some(v) => v,
        None => "",
    };

    match arg1 {
        "list" => print_entries(&conn)?,
        "add" => add(&conn)?,
        _ => print_entries(&conn)?,
    }

    Ok(())
}
