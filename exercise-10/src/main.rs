use std::io::{stdin, stdout, Write};

const TAX_PERCENT_RATE: f32 = 5.5;
fn main() {
    let items = get_items_from_user();
    let totals = calculate_totals(items);
    print_output(totals);
}

fn prompt_user_for_float(prompt: &str) -> Option<f32> {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin!");
    let input = input.trim();
    if input.is_empty() {
        None
    } else {
        Some(input.parse::<f32>().expect("Unable to parse input to f32"))
    }
}

fn prompt_user_for_int(prompt: &str) -> Option<i32> {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin!");
    let input = input.trim();
    if input.is_empty() {
        None
    } else {
        Some(input.parse::<i32>().expect("Unable to parse input to i32"))
    }
}
fn get_single_item_from_user() -> Option<(i32, i32)> {
    let price =
        match prompt_user_for_float("Enter the price of the item or <return> if no more items:") {
            Some(value) => (value * 100.0) as i32,
            None => return None,
        };
    let quantity =
        match prompt_user_for_int("Enter the quantity of the item or <return> if no more items:") {
            Some(value) => value,
            None => return None,
        };
    Some((quantity, price))
}
fn get_items_from_user() -> Vec<(i32, i32)> {
    let mut items: Vec<(i32, i32)> = Vec::new();
    loop {
        let item = get_single_item_from_user();
        match item {
            Some(value) => items.push(value),
            None => break,
        }
    }
    items
}

fn calculate_totals(items: Vec<(i32, i32)>) -> (i32, i32, i32) {
    let mut subtotal: i32 = 0;
    for item in items {
        subtotal += item.0 * item.1;
    }
    let tax = (subtotal as f32 * (TAX_PERCENT_RATE / 100.0)) as i32;
    let total = subtotal + tax;
    (subtotal, tax, total)
}

fn print_output(totals: (i32, i32, i32)) {
    let subtotal = totals.0 as f32 / 100.0;
    let tax = totals.1 as f32 / 100.0;
    let total = totals.2 as f32 / 100.0;

    println!("Subtotal: ${subtotal:.02}");
    println!("Tax: ${tax:.02}");
    println!("Total: ${total:.02}");
}
