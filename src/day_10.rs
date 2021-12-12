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
    let opening_tokens = "{[("
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let closing_tokens = "}])"
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let mut cursor = 0_usize;

    for (idx, token) in line.iter().enumerate() {
        // If we already checked a particular section: continue to the cursor position
        if idx <= cursor { continue; }

        // Look for the matching closing token
        if opening_tokens.contains(token) {
            let opening_token = token;
            let opening_token_idx = opening_tokens
                .iter()
                .position(|t| t == token)
                .unwrap();
            let closing_token = closing_tokens[opening_token_idx].clone();

            // Find all closing positions
            let closing_idxs = line
                .iter()
                .enumerate()
                .filter(|(_idx, c)| c == &&closing_token)
                .map(|(idx, _c)| idx)
                .collect::<Vec<_>>();

            if closing_idxs.len() > 0 {
                // Find the closing token, if any
                for closing_idx in closing_idxs {
                    if line[idx..closing_idx].contains(&opening_token) {
                        // If in the space between the opening and closing token contains another
                        // opening token, it is not the matching closing token for this opening one.
                    } else {
                        // The closing token matches the opening one. It is a "chunk"

                        match syntax_check(&line[idx..closing_idx].to_vec()) {
                            // Within a complete chunk, only a fully correct tree of chunks is allowed
                            Incomplete  => return Incorrect(line[closing_idx].clone()),
                            // If the chunk itself is incorrect, return the line as incorrect
                            Incorrect(token) => return Incorrect(token),
                            Correct => cursor = closing_idx,
                        }

                    }
                }

            } else { // NO closing tokens in the rest of the line
                // The section is incomplete, but the rest of the section may contain syntactical errors

            }
        }
    }

    Correct
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
