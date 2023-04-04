use std::io::{stdin, stdout, Write};

const PAINT_SQ_FT_PER_GALLON: i32 = 350;

fn main() {
    let (length, width) = get_user_input();
    let (sqft, paint_required) = calc_sqft_and_paint_required(length, width);
    display_output(sqft, paint_required);
}

fn prompt_user_for_num(prompt: &str) -> i32 {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdout");
    input.trim().parse().expect("Unable to parse input to i32")
}

fn get_user_input() -> (i32, i32) {
    let length = prompt_user_for_num("Enter the length of the room in feet:");
    let width = prompt_user_for_num("Enter the width of the room in feet: ");
    (length, width)
}

fn calc_sqft_and_paint_required(length: i32, width: i32) -> (i32, i32) {
    let total_sqft = length * width;
    let paint_required = (total_sqft + PAINT_SQ_FT_PER_GALLON - 1) / PAINT_SQ_FT_PER_GALLON;
    (total_sqft, paint_required)
}

fn display_output(sqft: i32, paint_required: i32) {
    println!("\nYou will need to purchase {paint_required} gallons to cover {sqft} square feet.")
}
