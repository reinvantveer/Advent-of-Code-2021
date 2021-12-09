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

    let mean_alignment = cheapest_expensive_alignment(&positions);
    println!("Alignment mean: {}", &mean_alignment);

    // Start guessing fuel consumption for some positions
    let mut lowest = 99999999999999_usize;

    for position in mean_alignment - 10..mean_alignment + 10 {
        let consumption = fuel_total_for_expensive_alignment(&positions, position);
        println!("Total fuel consumption for position {}: {}", position, consumption);
        if consumption < lowest { lowest = consumption; }
    }

    println!("Lowest consumption: {}", lowest);
}

pub(crate) fn fuel_total_for_alignment(positions: &Vec<usize>, align_position: usize) -> usize {
    let mut fuel_total = 0;

    for position in positions {
        fuel_total += (align_position as isize - *position as isize).abs() as usize;
    }

    fuel_total
}

pub(crate) fn fuel_total_for_expensive_alignment(positions: &Vec<usize>, align_position: usize) -> usize {
    let mut fuel_total = 0;

    for position in positions {
        let distance = (align_position as isize - *position as isize).abs() as usize;
        let stepped_increase: usize = (1..distance + 1).sum();
        fuel_total += stepped_increase;
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

pub(crate) fn cheapest_expensive_alignment(positions: &Vec<usize>) -> usize {
    let positions_as_floats = positions
        .clone()
        .iter()
        .map(|p| f64::from(*p as i16))
        .collect::<Vec<f64>>();
    let vector = Array1::from_vec(positions_as_floats);
    let mean = vector.mean().unwrap();

    println!("mean: {}", &mean);
    mean.round() as usize
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

#[test]
fn test_stepped_increase_consumption() {
    let inputs = read_lines("data/day_7_sample.txt");
    let hor_positions = parse_vec_usize(&inputs);
    let fuel_consumption = fuel_total_for_expensive_alignment(&hor_positions, 5);

    assert_eq!(fuel_consumption, 168);
    assert_eq!(cheapest_expensive_alignment(&hor_positions), 5)
}