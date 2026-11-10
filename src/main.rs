use std::io;
use chrono::{NaiveDate};
use rusqlite::{params, Connection, Result};

fn main() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("finances.db")?;
    create_tables(&conn)?;

    // ------- Ask for transaction info ----

    println!("Welcome to Finances!");

    let mut category = String::new();
    let mut card = String::new();
    let mut cost = String::new(); 

    println!("Please enter category.");
    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");

    println!("Please enter card.");
    io::stdin()
        .read_line(&mut card)
        .expect("Failed to read line");

    let date_formatted = loop {
        println!("Please enter date(MM-DD-YYYY).");
        let mut date = String::new();
        io::stdin().read_line(&mut date). expect("Failed to read line");

        match NaiveDate::parse_from_str(date.trim(), "%m-%d-%Y") {
            Ok(d) => break d,
            Err(_) => println!("Invalid date, try again."),
        }
    };

    println!("Please enter cost.");
    io::stdin()
        .read_line(&mut cost)
        .expect("Failed to read line");
    let cost_num: f64 = cost.trim().parse().expect("Not a number");

    println!("{} {} {} {cost_num}", category.trim(), card.trim(), date_formatted);

    insert_transaction(
        &conn,
        category.trim(),
        card.trim(),
        &date_formatted.to_string(),
        cost_num,
        "",
    )?;

    // Note: https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html#create-a-sqlite-database
    Ok(())
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists transactions (
            id integer primary key,
            category text not null,
            card text not null,
            date text not null,
            cost real not null,
            notes text
        )",
        (),
    )?;

    Ok(())
}

fn insert_transaction(    
    conn: &Connection, 
    category: &str, 
    card: &str, 
    date: &str, 
    cost: f64,
    notes: &str
    ) -> Result<()> {
    conn.execute(
        "INSERT INTO transactions (category, card, date, cost, notes) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
         params![category, card, date, cost, notes],
    )?;
    Ok(())
}
