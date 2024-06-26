mod utils;
mod crud;

use utils::*;
use crud::*;
use rusqlite::Result;

// main
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let conn = init()?;

    let arg1 = match args.get(1) {
        Some(v) => v,
        None => "",
    };

    match arg1 {
        "add" => add(&conn)?,
        "list" => list(&conn)?,
        "delete" => del(&conn)?,
        "update" => update(&conn)?,
        "search" => search(&conn)?,
        "today" => today(&conn)?,
        "help" => help(),
        "" => list(&conn)?,
        _ => eprintln!("no such command."),
    }

    Ok(())
}