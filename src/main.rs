use std::io::{self, Write};
use chrono::{NaiveDate};
use rusqlite::{params, Connection, Result};

struct Transaction {
    id: Option<i64>,
    category: String,
    card: String,
    date: NaiveDate,
    cost: f64,
    notes: Option<String>
}

fn main() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("finances.db")?;

    create_tables(&conn)?;

    println!("Welcome to Finances!");
    loop {
        print_menu();

        let mut menu_answer = String::new();
        io::stdin()
            .read_line(&mut menu_answer)
            .expect("Failed to read line");

        match menu_answer.trim() {
            "a" => {
                let t = create_transaction();
                insert_transaction(&conn, &t)?;
                println!("Transaction successful");
            }
            "s" => {
                let transactions = read_transactions(&conn)?;
                print_transactions(&transactions);
                let total = sum_transactions(&conn)?;
                println!("Total spent: ${total:.2}");
            }
            "d" => {
                let transactions = read_transactions(&conn)?;
                print_transactions(&transactions);

                let input = prompt("Enter the id to delete or leave blank to cancel");
                if input.is_empty() {
                    continue;
                }

                match input.parse::<i64>() {
                    Ok(id) => match delete_transaction(&conn, id)? {
                        0 => println!("No transaction with id {id}."),
                        _ => println!("Deleted transaction {id}"),
                    },
                    Err(_) => println!("That's not a valid id."),
                }
            }
            "q" => break,
            _ => println!("Unknown option"),
        }
    }


    // Note: https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html#create-a-sqlite-database
    Ok(())
}

// -------------- end of main ------------------

fn create_transaction() -> Transaction {

    let mut category = String::new();
    let mut card = String::new();
    let mut cost = String::new(); 
    let mut notes = String::new();

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

    println!("Are there any notes?");
    {
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");
        if answer.trim().to_string() == "y" {
            println!("What are your notes");
            io::stdin()
                .read_line(&mut notes)
                .expect("Failed to read line");
        }
    }
    let notes = notes.trim();

    println!("{} {} {} {cost_num}", category.trim(), card.trim(), date_formatted);
    println!("{}", notes.trim());

    let transaction = Transaction {
        id: None,
        category: category.trim().to_string(),
        card: card.trim().to_string(),
        date: date_formatted,
        cost: cost_num,
        notes: if notes.is_empty() { None } else { Some(notes.to_string()) },
    };

    transaction
}

fn print_menu() {
    println!("What do you want to do?");
    println!("");
    println!("a. Add transaction");
    println!("s. Check transactions");
    println!("d. Delete transaction");
    println!("q. Quit");
    println!();
}

fn prompt(message: &str) -> String {
    println!();
    print!("{message} ");
    io::stdout().flush().expect("Failed to flush");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn print_transactions(transactions: &[Transaction]) {
    for t in transactions {
        let id = t.id.map_or("-".to_string(), |id| id.to_string());
        println!(
            "{} | {} | {} | {} | ${:.2} | {}",
            id, t.category, t.card, t.date, t.cost,
            t.notes.as_deref().unwrap_or("-")
        );
    }
}


// --------- SQLite functions ----------------


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

fn insert_transaction(conn: &Connection, t: &Transaction) -> Result<()> {
    conn.execute(
        "INSERT INTO transactions (category, card, date, cost, notes)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            t.category,
            t.card,
            t.date,   // YYYY-MM-DD
            t.cost,
            t.notes,              // None becomes NULL
        ],
    )?;
    Ok(())
}

fn read_transactions(conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT id, category, card, date, cost, notes
        FROM transactions",
    )?;
    
    let rows = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            category: row.get(1)?,
            card: row.get(2)?,
            date: row.get(3)?,
            cost: row.get(4)?,
            notes: row.get(5)?,
        })
    })?;

    rows.collect()
}

fn delete_transaction(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute(
        "DELETE FROM transactions WHERE id = ?1",
        params![id],
        )
}

fn sum_transactions(conn: &Connection) -> Result<f64> {
    let total: Option<f64> = conn.query_row(
        "SELECT SUM(cost) FROM transactions",
        [],
        |row| row.get(0),
    )?;

    Ok(total.unwrap_or(0.0))
}
