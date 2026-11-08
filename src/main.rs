use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Finances!");
    println!("Please enter category.");


    let mut category = String::new();
    let mut card = String::new();
    let mut Date = String::new();

    io::stdin()
        .read_line(&mut category)
        .expect("Failed to read line");

    println!("You chose: {cat}");


}
