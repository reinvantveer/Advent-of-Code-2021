use ndarray::{Array2};
use advent_of_code_2021::read_lines;
use crate::day_13::Fold::{Left, Up};

pub(crate) fn run() {

}

#[derive(Debug, PartialEq)]
pub enum Fold {
    Left(usize),
    Up(usize),
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
                fold_instructions.push(Up(amount));
            } else {
                fold_instructions.push(Left(amount));
            }

            continue;
        }

        // Otherwise: the row is a set of coordinates
        let coords = input.split(",").collect::<Vec<_>>();
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();

        // Expand if necessary
        if x >= grid.len() {
            let cloned_first_row: Vec<bool> = grid
                .get(0)
                .unwrap_or(&vec![]).clone();

            let extra_rows = vec![vec![false; cloned_first_row.len()]; x + 1 - grid.len()];

            grid.extend(extra_rows);
        }

        if y >= grid[0].len() {
            let extra_xs = vec![false; y + 1 - grid[0].len()];
            for x_idx in 0..grid.len() {
                grid[x_idx].extend(extra_xs.clone());
            }
        }

        // Mark position
        grid[x][y] = true;
    }

    (grid, fold_instructions)
}

pub(crate) fn fold_grid(grid: &mut Vec<Vec<bool>>, fold: &Fold) {
    match fold {
        Up(fold_position) => {
            // Take all row indices below the fold
            // The example folds at row 7, so the row idxs are 8 thru 15
            let y_idxs_below_fold = (fold_position + 1)..grid[0].len();

            // We need all the column indices: they all get mirrored to the other side of the
            // fold
            let x_idxs = 0..grid.len();

            // Iterate over the range instead of the rows themselves, otherwise we get into bad
            // borrow territory
            for y_below_fold_idx in y_idxs_below_fold {
                for x_idx in x_idxs.clone() {
                    // We mirror to the opposite side of the fold
                    // So row 8 copies to row 6, which is the fold row 7 minus the difference
                    // between the fold row and the row below
                    let mirrored_y = (2 * *fold_position) - y_below_fold_idx;

                    if grid[x_idx][y_below_fold_idx] == true {
                        grid[x_idx][mirrored_y] = grid[x_idx][y_below_fold_idx];
                    }

                    // Empty the stuff below the fold
                    grid[x_idx][y_below_fold_idx] = false;
                }
            }
        }
        Left(fold_position) => {
            // Take all x indices to the right of the fold
            // The example folds at x = 5, so the x to the right idxs are 5 thru 11
            let x_idxs_right_of_fold = (fold_position + 1)..grid.len();

            // We need all the y indices: they all get mirrored to the other side of the
            // fold
            let y_idxs = 0..grid[0].len();

            // Iterate over the range instead of the rows themselves, otherwise we get into bad
            // borrow territory
            for x_right_of_fold_idx in x_idxs_right_of_fold {
                for y_idx in y_idxs.clone() {
                    // We mirror to the opposite side of the fold
                    // So col 8 copies to row 6, which is the fold col 7 minus the difference
                    // between the fold col and the col to the right
                    let mut mirrored_x = fold_position.clone();
                    mirrored_x *= 2;
                    mirrored_x = mirrored_x - x_right_of_fold_idx;

                    if grid[x_right_of_fold_idx][y_idx] == true {
                        grid[mirrored_x][y_idx] = grid[x_right_of_fold_idx][y_idx];
                    }
                    grid[x_right_of_fold_idx][y_idx] = false;
                }
            }
        }
    }
}

pub(crate) fn count_dots(grid: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;

    for row in grid {
        for entry in row {
            if *entry { count += 1; }
        }
    }

    count
}
#[cfg(test)]
#[test]
fn test_parse_grid() {
    let inputs = read_lines("data/day_13_sample.txt");
    let (grid, fold_instructions) = parse_grid(&inputs);
    assert_eq!(grid.len(), 11);
    assert_eq!(grid[0].len(), 15);
    
    let expected = vec![
        false, false, false, true, false, false, true, false, false, true, false,
        false, false, false, false, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
        true, false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, false, false, false, false, true, false, true,
        false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
        false, true, false, false, false, false, true, false, true, true, false,
        false, false, false, false, true, false, false, false, false, false, false,
        false, false, false, false, false, false, true, false, false, false, true,
        true, false, false, false, false, false, false, false, false, false, false,
        true, false, true, false, false, false, false, false, false, false, false,
    ];
    let array = Array2::from_shape_vec((11, 15), expected).unwrap();
    assert_eq!(array.shape(), [11, 15]);
    assert_eq!(fold_instructions, vec![Up(7), Right(5)]);
}

#[test]
fn test_first_fold() {
    let inputs = read_lines("data/day_13_sample.txt");
    let (mut grid, fold_instructions) = parse_grid(&inputs);

    fold_grid(&mut grid, &fold_instructions[0]);
    let dots = count_dots(&grid);
    assert_eq!(dots, 17);
}