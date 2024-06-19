use rusqlite::{Connection, Result};
use std::env::var;
use std::path::Path;

#[derive(Debug)]
struct Bday {
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

fn main() -> Result<()> {
    let home = var("HOME").expect("HOME environment variable not set");
    let db_dir_path = home + "/.config/bdaylog/";
    let db_path = db_dir_path.clone() + "data.db";

    if !Path::new(&db_dir_path).exists() {
        std::fs::create_dir(&db_dir_path).unwrap();
    } else {
        println!("directory exists, skipping creation.");
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
            (), // empty list of parameters.
        )?;
    } else {
        println!("table already created.");
    }

    // let me = Bday {
    //     name: "Savio".to_string(),
    //     year: 2002,
    //     month: "January".to_string(),
    //     day: 4,
    // };

    // conn.execute(
    //     "INSERT INTO bdays (name, year, month, day)
    //     VALUES (?1, ?2, ?3, ?4)",
    //     (&me.name, &me.year, &me.month, &me.day),
    // )?;

    let mut stmt = conn.prepare("SELECT name, date FROM bdays")?;
    let bday_iter = stmt.query_map([], |row| {
        Ok(Bday {
            name: row.get(0)?,
            date: row.get(1)?,
        })
    })?;

    for bday in bday_iter {
        match bday {
            Ok(bday) => println!("{} - {}", bday.name, bday.date),
            Err(err) => println!("{}", err),
        }
    }

    Ok(())
}
