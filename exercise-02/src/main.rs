use std::io;

fn main() {
    let input = get_user_input();
    let output = build_output(input);
    println!("{output}");
}

fn get_user_input() -> String {
    let mut input = String::new();
    println!("What is the input string?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn build_output(input: String) -> String {
    let char_count = count_chars(&input);
    format!("{input} has {char_count} characters.")
}

fn count_chars(input: &str) -> i32 {
    input.chars().count() as i32
}
