use advent_of_code_2021::read_lines;
use crate::day_10::Syntactical::{Correct, Incomplete, Incorrect};

pub(crate) fn run() {

}

pub(crate) fn read_tokens(inputs: &Vec<String>) -> Vec<Vec<String>> {
    inputs
        .iter()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_string())
                .collect()
        })
        .collect()
}

#[derive(PartialEq, Debug)]
pub(crate) enum Syntactical {
    Incomplete,
    Incorrect(String),
    Correct
}

pub(crate) fn syntax_check(line: &Vec<String>) -> Syntactical {
    let closing_tokens = "}])"
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let matching_tokens = vec!["[]".to_string(), "{}".to_string(), "()".to_string()];

    let mut mutable_line = line.join("");

    loop {
        let mut contains_matching_tokens = false;
        for token_set in &matching_tokens {
            if let Some(position) = mutable_line.find(token_set.as_str()) {
                contains_matching_tokens = true;
                mutable_line = mutable_line[0..position].to_string() + &mutable_line[position + 2..]
            }
        }
        if !contains_matching_tokens { break };
    }

    let mut first_remaining_closing_chars = None;
    for char in mutable_line.chars() {
        let char_as_string = char.to_string();
        if closing_tokens.contains(&char_as_string) {
            first_remaining_closing_chars = Some(char_as_string);
            break;
        }
    }

    if mutable_line.len() == 0 {
        return Correct
    } else if let Some(closing_char) = first_remaining_closing_chars {
        return Incorrect(closing_char)
    } else {
        return Incomplete
    }
}

#[cfg(test)]
#[test]
fn test_read_input() {
    let inputs = read_lines("data/day_10_sample.txt");
    let token_lines = read_tokens(&inputs);
    assert_eq!(token_lines[0].len(), 24)
}

#[test]
fn test_syntax_check() {
    let inputs = read_lines("data/day_10_sample.txt");
    let token_lines = read_tokens(&inputs);

    let first_line = token_lines[0].clone();
    assert_eq!(syntax_check(&first_line), Incomplete);

    let third_line = token_lines[2].clone();
    assert_eq!(syntax_check(&third_line), Incorrect("}".to_string()))
}
