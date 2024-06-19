use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Bday {
    name: String,
    year: i32,
    month: String,
    day: i32,
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE bdays (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            year  INTEGER NOT NULL,
            month TEXT NOT NULL,
            day INTEGER NOT NULL
        )",
        (), // empty list of parameters.
    )?;
    let me = Bday {
        name: "Savio".to_string(),
        year: 2002,
        month: "January".to_string(),
        day: 4,
    };
    conn.execute(
        "INSERT INTO bdays (name, year, month, day)
        VALUES (?1, ?2, ?3, ?4)",
        (&me.name, &me.year, &me.month, &me.day),
    )?;

    let mut stmt = conn.prepare("SELECT name, year, month, day FROM bdays")?;
    let bday_iter = stmt.query_map([], |row| {
        Ok(Bday {
            name: row.get(0)?,
            year: row.get(1)?,
            month: row.get(2)?,
            day: row.get(3)?,
        })
    })?;

    for bday in bday_iter {
        match bday {
            Ok(bday) => println!("{} - {} {} {}", bday.name, bday.year, bday.month, bday.day),
            Err(err) => println!("{}", err),
        }
        // println!("{0}: {1} {2} {3}",
        // bday.name, bday.year, bday.month, bday.day);
    }

    Ok(())
}
