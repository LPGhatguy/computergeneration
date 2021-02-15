use std::{io::Read, str::FromStr};

use anyhow::format_err;
use structopt::StructOpt;

/// Generates completions based on a word list and a prompt.
///
/// Word list is expected to be provided via stdin, and newline-delimited.
#[derive(Debug, StructOpt)]
struct Options {
    /// Pattern to complete against.
    pattern: String,

    /// Case matching strategy to use
    ///
    /// * auto: Case insensitive if pattern is all lowercase
    /// * sensitive: Always case sensitive
    /// * insensitive: Always case insensitive
    #[structopt(long, default_value = "auto", verbatim_doc_comment)]
    case: CaseMode,
}

#[derive(Debug)]
enum CaseMode {
    Auto,
    Sensitive,
    Insensitive,
}

impl Default for CaseMode {
    fn default() -> Self {
        CaseMode::Auto
    }
}

impl FromStr for CaseMode {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        match input {
            "auto" => Ok(CaseMode::Auto),
            "sensitive" => Ok(CaseMode::Sensitive),
            "insensitive" => Ok(CaseMode::Insensitive),

            _ => Err(format_err!(
                "Unknown case mode '{}'. Expected one of \
                 'auto', 'sensitive', or 'insensitive'."
            )),
        }
    }
}

fn main() {
    let options = Options::from_args();

    let mut word_list = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut word_list)
        .unwrap();

    let case_sensitive = match options.case {
        CaseMode::Sensitive => true,
        CaseMode::Insensitive => false,
        CaseMode::Auto => options
            .pattern
            .chars()
            .any(|char| char.is_ascii_uppercase()),
    };

    let case_sensitive_matcher = |a, b| a == b;
    let case_insensitive_matcher = |a: char, b| a.eq_ignore_ascii_case(&b);

    for word in word_list.lines() {
        if case_sensitive {
            handle_entry(case_sensitive_matcher, &options.pattern, word);
        } else {
            handle_entry(case_insensitive_matcher, &options.pattern, word);
        }
    }
}

fn handle_entry<F>(matcher: F, pattern: &str, word: &str)
where
    F: Fn(char, char) -> bool,
{
    let mut pattern_chars = pattern.chars();
    let mut word_chars = word.chars();

    loop {
        match (pattern_chars.next(), word_chars.next()) {
            (Some(input_char), Some(word_char)) => {
                if !matcher(input_char, word_char) {
                    return;
                }
            }

            // input is longer than word, which means we cannot match!
            (Some(_), None) => return,

            // input is over, we have matched successfully
            (None, _) => {
                println!("{}", word);
                return;
            }
        }
    }
}
