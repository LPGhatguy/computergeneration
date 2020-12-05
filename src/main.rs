use std::io::Read;

use structopt::StructOpt;

/// Generates completions based on a word list and a prompt.
///
/// Word list is expected to be provided via stdin, and newline-delimited.
#[derive(Debug, StructOpt)]
struct Options {
    /// Beginning of line to complete against.
    input: String,

    /// Whether matches should ignore case.
    #[structopt(long, short = "i")]
    case_insensitive: bool,
}

fn main() {
    let options = Options::from_args();

    let mut word_list = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut word_list)
        .unwrap();

    if options.case_insensitive {
        for word in word_list.lines() {
            if check_match_insensitive(&options.input, word) {
                println!("{}", word);
            }
        }
    } else {
        for word in word_list.lines() {
            if check_match(&options.input, word) {
                println!("{}", word);
            }
        }
    }
}

fn check_match(input: &str, word: &str) -> bool {
    word.starts_with(input)
}

fn check_match_insensitive(input: &str, word: &str) -> bool {
    for (input_char, word_char) in input.chars().zip(word.chars()) {
        if !input_char.eq_ignore_ascii_case(&word_char) {
            return false;
        }
    }

    true
}
