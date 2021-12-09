use ndarray::{Array1, Axis};
use ndarray_stats::interpolate::Nearest;
use ndarray_stats::QuantileExt;
use noisy_float::types::n64;

use advent_of_code_2021::{parse_vec_usize, read_lines};

pub(crate) fn run() {
    let inputs = read_lines("data/day_7_input.txt");
    let positions = parse_vec_usize(&inputs);
    let alignment = cheapest_alignment(&positions);
    println!("Alignment median: {}", &alignment);

    let consumption = fuel_total_for_alignment(&positions, alignment);
    println!("Total fuel consumption: {}", consumption);

}

pub(crate) fn fuel_total_for_alignment(positions: &Vec<usize>, align_position: usize) -> usize {
    let mut fuel_total = 0;

    for position in positions {
        fuel_total += (align_position as isize - *position as isize).abs() as usize;
    }

    fuel_total
}

pub(crate) fn cheapest_alignment(positions: &Vec<usize>) -> usize {
    let mut vector = Array1::from_vec(positions.clone());
    let axis = Axis(0);
    let median = vector
        .quantile_axis_mut(axis, n64(0.5), &Nearest)
        .unwrap();

    println!("median: {}", &median);
    median.as_slice().unwrap()[0]
}

#[cfg(test)]
#[test]
fn test_parse_sub_positions() {
    let inputs = read_lines("data/day_7_sample.txt");
    let hor_positions = parse_vec_usize(&inputs);
    let fuel_consumption = fuel_total_for_alignment(&hor_positions, 2);

    assert_eq!(fuel_consumption, 37);
}

#[test]
fn test_cheapest_alignment() {
    let inputs = read_lines("data/day_7_sample.txt");
    let hor_positions = parse_vec_usize(&inputs);
    let alignment = cheapest_alignment(&hor_positions);
    assert_eq!(alignment, 2)
}