use std::io;

// Ask the user for their name, and then prints a greeting.
fn main() {
    let name = get_user_input().expect("Something went wrong...");
    let response = build_response(name);
    print_response(response);
}

// Gets the user's name from standard input.
fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    println!("Please enter your name: ");
    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    Ok(input)
}

// Builds the greeting response using the provided name.
fn build_response(name: String) -> String {
    format!("Hello, {name}, nice to meet you!")
}

// Outputs the provided greeting to stdout.
fn print_response(response: String) {
    println!("{response}");
}
