use std::io::{stdin, stdout, Write};

struct UserInput {
    principal: f32,
    rate: f32,
    num_years: i32,
    times_per_year: i32,
}

fn main() {
    let input = get_user_inputs();
    let principal = input.principal;
    let rate = input.rate;
    let num_years = input.num_years;
    let times_per_year = input.times_per_year;
    let result = calc_compound_interest(principal, rate, num_years, times_per_year);
    print_output(principal, rate, num_years, times_per_year, result);
}

fn print_output(principal: f32, rate: f32, num_years: i32, times_per_year: i32, result: f32) {
    println!("${principal:.02} invested at {rate}% for {num_years} years compounded {times_per_year} times per year is ${result:.02}");
}

fn prompt_user_for_float(prompt: &str) -> f32 {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin!");
    input.trim().parse().expect("Unable to parse input to f32!")
}

fn prompt_user_for_int(prompt: &str) -> i32 {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin!");
    input.trim().parse().expect("Unable to parse input to i32!")
}

fn get_user_inputs() -> UserInput {
    let principal = prompt_user_for_float("What is the principal amount?");
    let rate = prompt_user_for_float("What is the rate?");
    let num_years = prompt_user_for_int("What is the number of years?");
    let times_per_year =
        prompt_user_for_int("What is the number of times the interest is compounded per year?");
    UserInput {
        principal,
        rate,
        num_years,
        times_per_year,
    }
}

fn calc_compound_interest(principal: f32, rate: f32, num_years: i32, times_per_year: i32) -> f32 {
    principal * (1.0 + (rate / 100.0) / times_per_year as f32).powi(times_per_year * num_years)
}
