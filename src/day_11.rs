use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

type OctopusGrid = Vec<Vec<usize>>;

pub(crate) fn octopi_from_input(input: &Vec<String>) -> OctopusGrid {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

pub(crate) fn simple_energy_increase(octopi: &mut OctopusGrid) {
    for octopus_row in octopi {
        for octopus in octopus_row {
            *octopus += 1;
        }
    }
}

pub(crate) fn flash_octopi(octopi: &mut OctopusGrid) -> usize {
    let mut flashes = 0;

    let mut row = 0;
    let mut col = 0;

    while row < octopi.len() {
        if octopi[row][col] > 9 {
            flashes += 1;
            octopi[row][col] = 0;
            propagate_energy(octopi, row, col);

            // Go back 2 higher and to left: the `match` below will advance one
            if row > 1 { row -= 2; } else if row > 0 { row -= 1; }
            if col > 1 { col -= 2; } else if col > 0 { col -= 1; }
        }

        // Advance position along the grid
        match col < octopi[0].len() - 1 {
            true => col += 1,
            false => { col = 0; row += 1 }
        }
    }

    flashes
}

pub(crate) fn propagate_energy(octopi: &mut OctopusGrid, row: usize, col: usize) {
    // Update upper left
    let rows = octopi.len();
    let cols = octopi[0].len();

    let has_left = col > 0;
    let has_above = row > 0;
    let has_right = col < cols - 1;
    let has_below = row < rows - 1;
    let has_upper_left = has_above && has_left;
    let has_upper_right = has_above && has_right;
    let has_bottom_left = has_below && has_left;
    let has_bottom_right = has_below && has_right;

    if has_upper_left {
        if octopi[row - 1][col - 1] > 0 {
            octopi[row - 1][col - 1] += 1;
        }
    }

    if has_above {
        if octopi[row - 1][col] > 0 {
            octopi[row - 1][col] += 1;
        }
    }

    if has_upper_right {
        if octopi[row - 1][col + 1] > 0 {
            octopi[row - 1][col + 1] += 1;
        }
    }

    if has_left {
        if octopi[row][col - 1] > 0 {
            octopi[row][col - 1] += 1;
        }
    }

    if has_right {
        if octopi[row][col + 1] > 0 {
            octopi[row][col + 1] += 1;
        }
    }

    if has_bottom_left {
        if octopi[row + 1][col - 1] > 0 {
            octopi[row + 1][col - 1] += 1;
        }
    }

    if has_below {
        if octopi[row + 1][col] > 0 {
            octopi[row + 1][col] += 1;
        }
    }

    if has_bottom_right {
        if octopi[row + 1][col + 1] > 0 {
            octopi[row + 1][col + 1] += 1;
        }
    }
}

#[cfg(test)]
#[test]
fn test_parse() {
    let input = read_lines("data/day_11_sample.txt");
    let octopi = octopi_from_input(&input);
    assert_eq!(octopi.len(), 10);
    assert_eq!(octopi[0].len(), 10);
}

#[test]
fn test_increase() {
    let input = read_lines("data/day_11_sample.txt");
    let mut octopi = octopi_from_input(&input);
    assert_eq!(octopi[0][0], 5);
    assert_eq!(octopi[9][9], 6);

    simple_energy_increase(&mut octopi);
    assert_eq!(octopi[0][0], 6);
    assert_eq!(octopi[9][9], 7);
}

#[test]
fn test_energy_increase() {
    let input = read_lines("data/day_11_sample.txt");
    let mut octopi = octopi_from_input(&input);

    // Once - nothing fancy happens yet
    simple_energy_increase(&mut octopi);
    let expected = vec![
        vec![6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
        vec![3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
        vec![6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
        vec![7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
        vec![7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
        vec![5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
        vec![3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
        vec![7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
        vec![5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
        vec![6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
    ];
    assert_eq!(octopi, expected);
}

#[test]
fn test_flash() {
    let input = read_lines("data/day_11_sample.txt");
    let mut octopi = octopi_from_input(&input);

    // Once - nothing fancy happens yet
    simple_energy_increase(&mut octopi);
    assert_eq!(flash_octopi(&mut octopi), 0);

    // Twice - now flashes start happening
    simple_energy_increase(&mut octopi);
    assert_eq!(flash_octopi(&mut octopi), 35);
    let expected = vec![
        vec![8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
        vec![5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
        vec![8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
        vec![8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
        vec![8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
        vec![6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
        vec![6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
        vec![0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
        vec![9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
        vec![8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
    ];
    assert_eq!(octopi, expected);

    // Third iteration
    simple_energy_increase(&mut octopi);
    assert_eq!(
        flash_octopi(&mut octopi), 45);
    let expected = vec![
        vec![0, 0, 5, 0, 9, 0, 0, 8, 6, 6],
        vec![8, 5, 0, 0, 8, 0, 0, 5, 7, 5],
        vec![9, 9, 0, 0, 0, 0, 0, 0, 3, 9],
        vec![9, 7, 0, 0, 0, 0, 0, 0, 4, 1],
        vec![9, 9, 3, 5, 0, 8, 0, 0, 6, 3],
        vec![7, 7, 1, 2, 3, 0, 0, 0, 0, 0],
        vec![7, 9, 1, 1, 2, 5, 0, 0, 0, 9],
        vec![2, 2, 1, 1, 1, 3, 0, 0, 0, 0],
        vec![0, 4, 2, 1, 1, 2, 5, 0, 0, 0],
        vec![0, 0, 2, 1, 1, 1, 9, 0, 0, 0],
    ];
    assert_eq!(octopi,expected)
}

#[test]
fn test_100_steps() {
    let input = read_lines("data/day_11_sample.txt");
    let mut octopi = octopi_from_input(&input);
    let mut flashes = 0;

    for _ in 0..100 {
        simple_energy_increase(&mut octopi);
        flashes += flash_octopi(&mut octopi);
    }
    assert_eq!(flashes, 1656);
}