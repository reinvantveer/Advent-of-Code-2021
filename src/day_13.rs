use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

#[derive(Debug, PartialEq)]
pub enum Fold {
    X(usize),
    Y(usize),
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
                fold_instructions.push(Y(amount));
            } else {
                fold_instructions.push(X(amount));
            }

            continue;
        }

        // Otherwise: the row is a set of coordinates
        let coords = input.split(",").collect::<Vec<_>>();
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();

        // Expand if necessary
        if x >= grid[0].len() {
            let extra_xs = vec![false; x + 1 - grid[0].len()];
            for x_idx in 0..grid.len() {
                grid[x_idx].extend(extra_xs.clone());
            }
        }

        if y >= grid.len() {
            let cloned_first_row: Vec<bool> = grid
                .get(0)
                .unwrap_or(&vec![]).clone();

            let extra_rows = vec![vec![false; cloned_first_row.len()]; y + 1 - grid.len()];

            grid.extend(extra_rows);
        }

        // Mark position
        grid[x][y] = true;
    }

    (grid, fold_instructions)
}

pub(crate) fn fold_grid(grid: &mut Vec<Vec<bool>>, fold: &Fold) {
    match fold {
        Y(fold_row) => {
            // Take all row indices below the fold
            // The example folds at row 7, so the row idxs are 8 thru 11
            let row_idxs_below_fold = (fold_row + 1)..grid.len();

            // Iterate over the range instead of the rows themselves, otherwise we get into bad
            // borrow territory
            for r_idx in row_idxs_below_fold {
                // We need all the column indices: they all get mirrored to the other side of the
                // fold
                let col_idxs = 0..grid[r_idx].len();

                for c_idx in col_idxs {
                    // We mirror to the opposite side of the fold
                    // So row 8 copies to row 6, which is the fold row 7 minus the difference
                    // between the fold row and the row below
                    let mirrored_row = fold_row + fold_row - r_idx;
                    grid[mirrored_row][c_idx] = grid[r_idx][c_idx];
                    grid[r_idx][c_idx] = false;
                }
            }
        }
        X(num) => {}
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
        vec![false, false, false, true, false, false, true, false, false, true, false],
        vec![false, false, false, false, true, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false],
        vec![true, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, true, false, false, false, false, true, false, true],
        vec![false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, false, false, false, false, false],
        vec![false, true, false, false, false, false, true, false, true, true, false],
        vec![false, false, false, false, true, false, false, false, false, false, false],
        vec![false, false, false, false, false, false, true, false, false, false, true],
        vec![true, false, false, false, false, false, false, false, false, false, false],
        vec![true, false, true, false, false, false, false, false, false, false, false],
    ];

    assert_eq!(grid, expected);

    assert_eq!(fold_instructions, vec![Y(7), X(5)]);
}

#[test]
fn test_first_fold() {
    let inputs = read_lines("data/day_13_sample.txt");
    let (mut grid, fold_instructions) = parse_grid(&inputs);

    fold_grid(&mut grid, &fold_instructions[0]);
    let dots = count_dots(&grid);
    assert_eq!(dots, 17);
}