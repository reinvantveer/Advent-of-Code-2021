use std::fs::File;
use std::io;
use std::io::BufRead;

pub(crate) fn run () {
    println!("Day 1");
    let input = File::open("src/day_1/input.txt").unwrap();
    let reader = io::BufReader::new(input);
    let inputs: Vec<String> = reader.lines()
        .filter_map(io::Result::ok)
        .collect();
    let measurements: Vec<i32> = inputs
        .iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;
    for (idx, measurement) in measurements.iter().enumerate() {
        if idx == 0 { continue; };
        if measurement > &measurements[idx - 1] { increases += 1}
    }
    println!("There are {} increases", increases);
}
