use regex::Regex;
use rusqlite::{Connection, Result};
use crate::utils::*;

// add entries
pub fn add(conn: &Connection) -> Result<()> {
    let name = get_user_input("Enter name: ");
    let date = get_user_input("Enter date (YYYY-MM-DD): ");

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

pub fn update(conn: &Connection) -> Result<(), rusqlite::Error> {
    list(&conn)?;

    println!("");
    let index = get_user_input("Which entry to update? ");
    let name = get_user_input("Updated name (leave empty for previous value): ");
    let date = get_user_input("Updated date (leave empty for previous value): ");

    if !name.is_empty() && date.is_empty() {
        conn.execute("UPDATE bdays SET name = ? WHERE id = ?", [name, index])?;
        println!("Name updated successfully.");
    } else if !date.is_empty() && name.is_empty() {
        conn.execute("UPDATE bdays SET date = ? WHERE id = ?", [date, index])?;
        println!("Date updated successfully.");
    } else if !name.is_empty() && !date.is_empty() {
        conn.execute(
            "UPDATE bdays SET name = ?, date = ? WHERE id = ?",
            [name, date, index],
        )?;
        println!("Name and date updated successfully.");
    } else if name.is_empty() && date.is_empty() {
        println!("None updated.")
    }

    Ok(())
}

pub fn del(conn: &Connection) -> Result<(), rusqlite::Error> {
    list(&conn)?;

    println!("");
    let index = get_user_input("Which entry to delete? ");

    conn.execute("DELETE FROM bdays WHERE id = ?", [index])?;

    println!("Deleted successfully.");

    Ok(())
}

pub fn search_name(conn: &Connection, query: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, date FROM bdays WHERE name LIKE ?")?;

    let rows = stmt.query_map([format!("%{}%", query)], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let date: String = row.get(2)?;

        Ok((id, name, date))
    })?;

    print_header();

    for row in rows {
        let (id, name, date) = row?;
        print_row(id, name, date);
    }

    Ok(())
}

// list entries
pub fn list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, date FROM bdays")?;

    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        let date: String = row.get(2)?;

        Ok((id, name, date))
    })?;

    print_header();

    for row in rows {
        let (id, name, date) = row?;
        print_row(id, name, date);
    }

    Ok(())
}