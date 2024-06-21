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

    let arg2 = match args.get(2) {
        Some(v) => v,
        None => "",
    };

    match arg1 {
        "add" => add(&conn)?,
        "list" => list(&conn)?,
        "del" => del(&conn)?,
        "update" => update(&conn)?,
        "search_name" => search_name(&conn, arg2)?,
        _ => list(&conn)?,
    }

    Ok(())
}