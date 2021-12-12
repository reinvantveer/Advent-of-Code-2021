use std::collections::HashMap;
use advent_of_code_2021::read_lines;
use crate::day_10::Syntactical::{Correct, Incomplete, Incorrect};

pub(crate) fn run() {
    let inputs = read_lines("data/day_10_input.txt");
    let token_lines = read_tokens(&inputs);
    let incorrect_score = score_from_incorrect_tokens(&token_lines);
    println!("Total score for incorrect tokens: {}", incorrect_score);

    let incomplete_score = score_from_incomplete_lines(&token_lines);
    println!("Score for incomplete autocomplete: {}", incomplete_score);
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
    Incomplete(String),
    Incorrect(String),
    Correct
}

pub(crate) fn syntax_check(line: &Vec<String>) -> Syntactical {
    let closing_tokens = "}])>"
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let matching_tokens = vec![
        "[]".to_string(),
        "{}".to_string(),
        "()".to_string(),
        "<>".to_string(),
    ];

    let mut remaining = line.join("");

    loop {
        let mut contains_matching_tokens = false;
        for token_set in &matching_tokens {
            if let Some(position) = remaining.find(token_set.as_str()) {
                contains_matching_tokens = true;
                remaining = remaining[0..position].to_string() + &remaining[position + 2..]
            }
        }
        if !contains_matching_tokens { break };
    }

    let mut first_remaining_closing_chars = None;
    for char in remaining.chars() {
        let char_as_string = char.to_string();
        if closing_tokens.contains(&char_as_string) {
            first_remaining_closing_chars = Some(char_as_string);
            break;
        }
    }

    if remaining.len() == 0 {
        return Correct
    } else if let Some(closing_char) = first_remaining_closing_chars {
        return Incorrect(closing_char)
    } else {
        return Incomplete(remaining)
    }
}

pub(crate) fn score_from_incorrect_tokens(token_lines: &Vec<Vec<String>>) -> i32 {
    let token_scores = HashMap::from([
        (")".to_string(), 3),
        ("]".to_string(), 57),
        ("}".to_string(), 1197),
        (">".to_string(), 25137)
    ]);

    let mut score = 0;

    for line in token_lines {
        if let Incorrect(offending_token) = syntax_check(&line) {
            score += token_scores[&offending_token];
        }
    }

    score
}

pub(crate) fn score_from_incomplete_lines(token_lines: &Vec<Vec<String>>) -> usize {
    let token_scores = HashMap::from([
        (")".to_string(), 1),
        ("]".to_string(), 2),
        ("}".to_string(), 3),
        (">".to_string(), 4),
    ]);

    let mut scores = Vec::new();

    for line in token_lines {
        let mut score = 0;

        if let Incomplete(tokens) = syntax_check(&line)  {
            let completion_tokens = autocomplete(&tokens);

            for token in completion_tokens {
                score = score * 5 + token_scores[&token];
            }

            scores.push(score);
        }
    }


    scores.sort();
    let middle_idx = scores.len() / 2;  // Add one for correcting usize

    scores[middle_idx]
}

pub(crate) fn autocomplete(line: &String) -> Vec<String> {
    let closing_map: HashMap<String, String> = HashMap::from([
        ("[".to_string(), "]".to_string()),
        ("{".to_string(), "}".to_string()),
        ("(".to_string(), ")".to_string()),
        ("<".to_string(), ">".to_string()),
    ]);

    let mut closing_tokens = line
        .chars()
        .map(|c| closing_map[&c.to_string()].clone())
        .collect::<Vec<String>>();

    closing_tokens.reverse();

    closing_tokens
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
    assert_eq!(syntax_check(&first_line), Incomplete("[({([[{{".to_string()));

    let third_line = token_lines[2].clone();
    assert_eq!(syntax_check(&third_line), Incorrect("}".to_string()))
}

#[test]
fn test_score_incorrect_tokens() {
    let inputs = read_lines("data/day_10_sample.txt");
    let token_lines = read_tokens(&inputs);

    let score = score_from_incorrect_tokens(&token_lines);
    assert_eq!(score, 26397);
}

#[test]
fn test_score_incomplete_lines() {
    let inputs = read_lines("data/day_10_sample.txt");
    let token_lines = read_tokens(&inputs);

    let first_line = token_lines[0].clone();
    if let Incomplete(opening_chars) = syntax_check(&first_line) {
        let autocompleted = autocomplete(&opening_chars);
        let expected = "}}]])})]"
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert_eq!(autocompleted, expected);
    } else {
        unreachable!("Wrong Syntactical return type");
    };

    let score = score_from_incomplete_lines(&token_lines);
    assert_eq!(score, 288957);
}