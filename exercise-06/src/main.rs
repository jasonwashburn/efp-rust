use chrono::{self, Datelike};
use std::io::{stdin, stdout, Write};

fn main() {
    let current_year = chrono::offset::Local::now().year();
    let current_age = get_current_age();
    let retirement_age = get_retirement_age();
    let years_left = retirement_age - current_age;
    let retirement_year = current_year + years_left;
    println!("You have {years_left} years left until you can retire.");
    println!("It's {current_year}, so you can retire in {retirement_year}.")
}

fn get_current_age() -> i32 {
    prompt_user_for_number("What is your current age?")
}

fn get_retirement_age() -> i32 {
    prompt_user_for_number("At what age would you like to retire?")
}

fn prompt_user_for_number(message: &str) -> i32 {
    let mut input = String::new();
    print!("{message} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read user input!");
    input
        .trim()
        .parse::<i32>()
        .expect("Unable to parse input to i32")
}
