use advent_of_code_2021::read_lines;
use crate::day_13::Fold::{Column, Row};

pub(crate) fn run() {

}

#[derive(Debug, PartialEq)]
pub(crate) enum Fold {
    Row(usize),
    Column(usize),
}

pub(crate) fn parse_grid(inputs: &Vec<String>) -> (Vec<Vec<bool>>, Vec<Fold>) {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut fold_instructions = Vec::new();

    for input in inputs {
        if input == &"".to_string() { continue; }

        // Handle fold instructions
        if input.starts_with("fold along ") {
            let direction_amount = input[11..]
                .split("=")
                .collect::<Vec<&str>>();

            let direction = direction_amount[0].clone();
            let amount = direction_amount[1].parse::<usize>().unwrap();

            if direction == "y" {
                fold_instructions.push(Row(amount));
            } else {
                fold_instructions.push(Column(amount));
            }

            continue;
        }

        // Otherwise: the row is a set of coordinates
        let coords = input.split(",").collect::<Vec<_>>();
        let row = coords[0].parse::<usize>().unwrap();
        let col = coords[1].parse::<usize>().unwrap();

        // Expand if necessary
        if row >= grid.len() {
            let cloned_first_row: Vec<bool> = grid
                .get(0)
                .unwrap_or(&vec![]).clone();

            let extra_rows = vec![cloned_first_row; row + 1 - grid.len()];

            grid.extend(extra_rows);
        }

        if col >= grid[0].len() {
            let extra_cols = vec![false; col + 1 - grid[0].len()];
            for row_idx in 0..grid.len() {
                grid[row_idx].extend(extra_cols.clone());
            }
        }

        // Mark position
        grid[row][col] = true;
    }

    (grid, fold_instructions)
}

#[cfg(test)]
#[test]
fn test_parse_grid() {
    let inputs = read_lines("data/day_13_sample.txt");
    let (grid, fold_instructions) = parse_grid(&inputs);
    assert_eq!(grid.len(), 11);
    assert_eq!(grid[0].len(), 15);
    assert_eq!(grid[6][10], true);

    assert_eq!(fold_instructions, vec![Row(7), Column(5)]);
}