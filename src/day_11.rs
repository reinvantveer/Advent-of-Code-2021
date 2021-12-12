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

    for octopus_row in octopi {
        for octopus in octopus_row {
            if *octopus > 9 {
                flashes += 1;
            }
        }
    }

    flashes
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