use std::io::{stdin, stdout, Write};

const SQFT_TO_METERS: f32 = 0.09290304;

fn main() {
    let length = prompt_user_for_number("What is the length of the room in feet?");
    let width = prompt_user_for_number("What is the width of the room in feet?");
    let area = calc_area_of_rectangle(length, width);
    let area_in_meters = area as f32 * SQFT_TO_METERS;
    print_output(length, width, area, area_in_meters);
}

fn print_output(length: i32, width: i32, area: i32, area_in_meters: f32) {
    println!("You entered dimensions of {length} feet by {width} feet.");
    println!("The area is");
    println!("{area} square feet.");
    println!("{area_in_meters} square meters.");
}

fn calc_area_of_rectangle(length: i32, width: i32) -> i32 {
    length * width
}

fn prompt_user_for_number(prompt: &str) -> i32 {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to get input from stdin!");
    input
        .trim()
        .parse()
        .expect("Unable to parse input to an i32!")
}
