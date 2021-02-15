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
    let mut input_chars = input.chars();
    let mut word_chars = word.chars();

    loop {
        match (input_chars.next(), word_chars.next()) {
            (Some(input_char), Some(word_char)) => {
                if !input_char.eq_ignore_ascii_case(&word_char) {
                    return false;
                }
            }

            // input is longer than word, which means we cannot match!
            (Some(_), None) => return false,

            // input is over, we have matched successfully
            (None, _) => return true,
        }
    }
}
