use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_6_input.txt");
    let mut school = parse_school(&inputs);
    procreate_for_days(&mut school, 80);

    println!("School size after 80 days is {}", school.len());

    let mut school_bins = parse_smart_school(&inputs);
    smart_procreate_for_days(&mut school_bins, 256);
    let school_size = school_bins.iter().sum::<usize>();
    println!("School size after 256 days is {}", school_size);
}

pub(crate) fn parse_school(inputs: &Vec<String>) -> Vec<usize> {
    let start_population = inputs[0]
        .split(",")
        .map(|f| f.parse::<usize>().unwrap())
        .collect();

    start_population
}

pub(crate) fn parse_smart_school(inputs: &Vec<String>) -> Vec<usize> {
    let mut age_population = vec![0; 9];

    inputs[0]
        .split(",")
        .for_each(|f| {
            let fish_age = f.parse::<usize>().unwrap();
            age_population[fish_age] += 1;
        });

    age_population
}

pub(crate) fn procreate(school: &mut Vec<usize>) {
    let mut new_fish = Vec::new();

    for fish_procreate_countdown in &mut *school {
        if *fish_procreate_countdown == 0 {
            *fish_procreate_countdown = 6;
            new_fish.push(8);
            continue;
        }

        *fish_procreate_countdown -= 1;
    }

    school.extend(new_fish);
}

pub(crate) fn smart_procreate(school_bins: &mut Vec<usize>) {
    school_bins.rotate_left(1);
    school_bins[6] += school_bins[8];
}

pub(crate) fn procreate_for_days(school: &mut Vec<usize>, days: usize) {
    for day in 0..days {
        procreate(school);
        if day % 10 == 0 {
            let school_size = school.len();
            println!("Procreated day {}, school size: {}", day, school_size);
        }
    }
}

pub(crate) fn smart_procreate_for_days(school_bins: &mut Vec<usize>, days: usize) {
    for day in 0..days {
        smart_procreate(school_bins);
    }
}

#[cfg(test)]
#[test]
fn test_procreate() {
    let inputs = read_lines("data/day_6_sample.txt");
    let mut school = parse_school(&inputs);
    assert_eq!(school, vec![3, 4, 3, 1, 2]);

    procreate(&mut school);
    assert_eq!(school, vec![2, 3, 2, 0, 1]);

    procreate(&mut school);
    assert_eq!(school, vec![1, 2, 1, 6, 0, 8]);
}

#[test]
fn test_procreate_for_80_days() {
    let inputs = read_lines("data/day_6_sample.txt");
    let mut school = parse_school(&inputs);

    procreate_for_days(&mut school, 80);
    assert_eq!(school.len(), 5934);
}

#[test]
fn test_parse_smart_school() {
    let inputs = read_lines("data/day_6_sample.txt");
    let mut school = parse_smart_school(&inputs);

    assert_eq!(school, vec![0, 1, 1, 2, 1, 0, 0, 0, 0])
}

#[test]
fn test_count_smart_school_procreation() {
    let inputs = read_lines("data/day_6_sample.txt");
    let mut school_bins = parse_smart_school(&inputs);
    assert_eq!(school_bins, vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);

    smart_procreate(&mut school_bins);
    assert_eq!(school_bins, vec![1, 1, 2, 1, 0, 0, 0, 0, 0]);

    smart_procreate(&mut school_bins);
    assert_eq!(school_bins, vec![1, 2, 1, 0, 0, 0, 1, 0, 1]);

    // Already procreated twice
    smart_procreate_for_days(&mut school_bins, 80 - 2);
    assert_eq!(school_bins.iter().sum::<usize>(), 5934)
}
