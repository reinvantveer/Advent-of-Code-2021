use advent_of_code_2021::read_lines;

pub(crate) fn run () {
    let inputs = read_lines("data/part_1_input.txt");

    let measurements: Vec<i32> = inputs
        .iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;
    for (idx, measurement) in measurements.iter().enumerate() {
        if idx == 0 { continue; };
        if measurement > &measurements[idx - 1] { increases += 1}
    }
    println!("There are {} increases in depth", increases);
}

