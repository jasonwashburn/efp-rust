use std::io::{stdin, stdout, Write};

fn main() {
    let user_input = get_user_inputs();
    let result = calc_interest(&user_input);
    output_result(&result)
}

struct UserInput {
    principal: f32,
    interest_rate: f32,
    num_years: i32,
}

struct InterestResult {
    num_years: i32,
    interest_rate: f32,
    final_value: f32,
    annual_values: Vec<f32>,
}

fn prompt_user_for_float(prompt: &str) -> f32 {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush standard out!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin");
    input.trim().parse().expect("Unable to parse input to f32")
}

fn prompt_user_for_int(prompt: &str) -> i32 {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush standard out!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin");
    input.trim().parse().expect("Unable to parse input to i32")
}

fn get_user_inputs() -> UserInput {
    let principal = prompt_user_for_float("Enter the principal:");
    let interest_rate = prompt_user_for_float("Enter the interest rate:");
    let num_years = prompt_user_for_int("Enter the number of years:");
    UserInput {
        principal,
        interest_rate,
        num_years,
    }
}

fn calc_interest(input: &UserInput) -> InterestResult {
    let mut annual_values: Vec<f32> = Vec::new();
    let mut yearly_principal = input.principal;
    annual_values.push(input.principal);
    for _ in 0..input.num_years {
        yearly_principal = calc_annual_interest(yearly_principal, input.interest_rate);
        annual_values.push(yearly_principal)
    }
    InterestResult {
        num_years: input.num_years,
        interest_rate: input.interest_rate,
        final_value: yearly_principal,
        annual_values,
    }
}

fn calc_annual_interest(principal: f32, interest_rate: f32) -> f32 {
    // A = P(1 + rt), where P is the principal amount, r is the annual rate of interest,
    // t is the number of years the amount is invested, and A is the amount at the
    // end of the investment.
    let num_years = 1.0;
    principal * (1.0 + (interest_rate / 100.0) * num_years)
}

fn output_result(result: &InterestResult) {
    println!(
        "After {num_years} years at {interest_rate}%, the investment will be worth ${final_value:.02}.",
        num_years = result.num_years,
        interest_rate = result.interest_rate,
        final_value = result.final_value
    );
    println!("--------------------------------------");
    for (year, value) in result.annual_values.iter().enumerate() {
        match year {
            0 => println!("Starting Value:\t\t\t${value:.02}"),
            _ => println!("Value at the end of year {year}: \t${value:.02}"),
        };
    }
}
