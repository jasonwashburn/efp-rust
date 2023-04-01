use std::io::{stdin, stdout, Write};

struct MadLibInputs {
    noun: String,
    verb: String,
    adjective: String,
    adverb: String,
}

fn main() {
    let inputs = get_user_input();
    let story = build_story(inputs);
    print_output(story);
}

fn print_output(story: String) {
    println!("{story}")
}

fn get_user_input() -> MadLibInputs {
    let noun = get_part_of_speech("noun");
    let verb = get_part_of_speech("verb");
    let adjective = get_part_of_speech("adjective");
    let adverb = get_part_of_speech("adverb");

    MadLibInputs {
        noun,
        verb,
        adjective,
        adverb,
    }
}

fn get_part_of_speech(part: &str) -> String {
    let mut input = String::new();
    print!("Enter a {part}: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut input)
        .expect("Unable to read user input!");
    input.trim().to_string()
}

fn build_story(inputs: MadLibInputs) -> String {
    format!(
        "Do you {verb} your {adjective} {noun} {adverb}? That's hilarious!",
        verb = inputs.verb,
        adjective = inputs.adjective,
        noun = inputs.noun,
        adverb = inputs.adverb
    )
}
