use std::io::{self, stdout, Write};

struct MathInput {
    num_one: i32,
    num_two: i32,
}

struct MathOutput {
    num_one: i32,
    num_two: i32,
    sum: i32,
    difference: i32,
    product: i32,
    quotient: i32,
}

fn main() {
    let input = get_user_inputs();
    let results = calculate_math_results(input);
    print_results(results);
}

fn print_results(results: MathOutput) {
    println!(
        "{num_one} + {num_two} = {sum}\n\
        {num_one} - {num_two} = {difference}\n\
        {num_one} * {num_two} = {product}\n\
        {num_one} / {num_two} = {quotient}\n",
        num_one = results.num_one,
        num_two = results.num_two,
        sum = results.sum,
        difference = results.difference,
        product = results.product,
        quotient = results.quotient
    )
}

fn get_user_inputs() -> MathInput {
    let num_one = prompt_user_for_number("What is the first number?");
    let num_two = prompt_user_for_number("What is the second number?");
    MathInput { num_one, num_two }
}

fn calculate_math_results(input: MathInput) -> MathOutput {
    let num_one = input.num_one;
    let num_two = input.num_two;

    let sum = num_one + num_two;
    let difference = num_one - num_two;
    let product = num_one * num_two;
    let quotient = num_one / num_two;

    MathOutput {
        num_one,
        num_two,
        sum,
        difference,
        product,
        quotient,
    }
}

fn prompt_user_for_number(message: &str) -> i32 {
    let mut input = String::new();
    print!("{message} ");
    stdout().flush().expect("Unable to flush stdout!");
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read user input!");
    input
        .trim()
        .parse::<i32>()
        .expect("Unable to parse input into i32!")
}
