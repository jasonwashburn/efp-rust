use std::io;

struct Quote {
    author: String,
    text: String,
}

fn main() {
    let quote = get_user_input();
    let output = build_output(quote);
    print_output(output);
}

fn get_user_input() -> Quote {
    let mut text = String::new();
    let mut author = String::new();
    println!("What is the quote?");
    io::stdin()
        .read_line(&mut text)
        .expect("Unable to get input from user!");
    println!("Who said it?");
    io::stdin()
        .read_line(&mut author)
        .expect("Unable to get input from user!");
    let text = text.trim().to_string();
    let author = author.trim().to_string();
    Quote { author, text }
}

fn build_output(quote: Quote) -> String {
    quote.author + r#" says, ""# + &quote.text + r#"""#
}

fn print_output(output: String) {
    println!("{output}")
}
