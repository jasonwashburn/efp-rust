use std::io::{stdin, stdout, Write};

fn main() {
    let (amount_from, rate_from) = get_user_inputs();
    let amount_to = calc_exchanged_currency(amount_from, rate_from);
    print_output(amount_from, rate_from, amount_to);
}

fn prompt_user_for_float(message: &str) -> f32 {
    let mut input = String::new();
    print!("{message} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin!");
    input.trim().parse().expect("Unable to parse input to f32")
}

fn get_user_inputs() -> (f32, f32) {
    let amount_from = prompt_user_for_float("How many Euros are you exchanging?");
    let rate_from = prompt_user_for_float("What is the exchange rate? ");
    (amount_from, rate_from)
}

fn calc_exchanged_currency(amount_from: f32, rate_from: f32) -> f32 {
    ((amount_from * rate_from) * 100.0).round() / 100.0
}

fn print_output(amount_from: f32, rate_from: f32, amount_to: f32) {
    println!("{amount_from} euros at an exchange rate of {rate_from} is {amount_to} U.S. dollars.");
}
