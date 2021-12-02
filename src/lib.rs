use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_lines(file: &str) -> Vec<String> {
    let input = File::open(file).unwrap();
    let reader = io::BufReader::new(input);
    let inputs: Vec<String> = reader.lines()
        .filter_map(io::Result::ok)
        .collect();
    inputs
}