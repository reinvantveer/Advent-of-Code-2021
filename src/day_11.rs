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
            if row > 0 { row -= 1; }
            if col > 0 { col -= 1; }
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
fn test_flash() {
    let input = read_lines("data/day_11_sample.txt");
    let mut octopi = octopi_from_input(&input);

    // Once - nothing happens yet
    simple_energy_increase(&mut octopi);
    assert_eq!(flash_octopi(&mut octopi), 0);
    // Twice - now
    simple_energy_increase(&mut octopi);
    assert_eq!(flash_octopi(&mut octopi), 35)
}