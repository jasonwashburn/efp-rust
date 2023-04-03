use std::io::{stdin, stdout, Write};

fn main() {
    let (num_people, num_pizzas, slices_per_pizza) = get_user_input();
    let (pieces_per_person, leftover_pieces) =
        calc_pizza_pieces(num_people, num_pizzas, slices_per_pizza);
    print_output(num_people, num_pizzas, pieces_per_person, leftover_pieces);
}

fn print_output(num_people: i32, num_pizzas: i32, pieces_per_person: i32, leftover_pieces: i32) {
    println!("\n{num_people} people with {num_pizzas} pizzas.");
    println!("Each person gets {pieces_per_person} slices of pizza.");
    println!("There are {leftover_pieces} pieces left over.");
}

fn calc_pizza_pieces(num_people: i32, num_pizzas: i32, slices_per_pizza: i32) -> (i32, i32) {
    let total_slices = num_pizzas * slices_per_pizza;
    let pieces_per_person = total_slices / num_people;
    let leftover_pieces = total_slices % num_people;
    (pieces_per_person, leftover_pieces)
}

fn get_user_input() -> (i32, i32, i32) {
    let num_people = prompt_user_for_number("How many people?");
    let num_pizzas = prompt_user_for_number("How many pizzas do you have?");
    let slices_per_pizza = prompt_user_for_number("How many slices are ther per pizza?");
    (num_people, num_pizzas, slices_per_pizza)
}

fn prompt_user_for_number(prompt: &str) -> i32 {
    let mut input = String::new();
    print!("{prompt} ");
    stdout().flush().expect("Unable to flush stdout!");
    stdin()
        .read_line(&mut input)
        .expect("Unable to read from stdin!");
    input
        .trim()
        .parse::<i32>()
        .expect("Unable to parse input to i32")
}
