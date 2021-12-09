use advent_of_code_2021::{parse_vec_usize, read_lines};

pub(crate) fn run() {
}

pub(crate) fn fuel_total_for_alignment(positions: &Vec<usize>, align_position: usize) -> usize {
    let mut fuel_total = 0;

    for position in positions {
        fuel_total += (align_position as isize - *position as isize).abs() as usize;
    }

    fuel_total
}

#[cfg(test)]
#[test]
fn test_parse_sub_positions() {
    let inputs = read_lines("data/day_7_sample.txt");
    let hor_positions = parse_vec_usize(&inputs);
    let fuel_consumption = fuel_total_for_alignment(&hor_positions, 2);

    assert_eq!(fuel_consumption, 37);
}
