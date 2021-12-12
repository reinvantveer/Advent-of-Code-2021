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

pub fn parse_vec_usize(inputs: &Vec<String>) -> Vec<usize> {
    let vec = inputs[0]
        .split(",")
        .map(|f| f.parse::<usize>().unwrap())
        .collect();

    vec
}

#[cfg(test)]
#[test]
fn test_vec_usize_from_input() {
    let inputs = read_lines("data/day_6_sample.txt");
    let school = parse_vec_usize(&inputs);
    assert_eq!(school, vec![3, 4, 3, 1, 2]);
}

