use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_6_input.txt");
    let mut school = parse_school(&inputs);
    procreate_for_days(&mut school, 80);

    println!("School size after 80 days is {}", school.len());

    procreate_for_days(&mut school, 256 - 80);
    println!("School size after 256 days is {}", school.len());

}

pub(crate) fn parse_school(inputs: &Vec<String>) -> Vec<usize> {
    let start_population = inputs[0]
        .split(",")
        .map(|f| f.parse::<usize>().unwrap())
        .collect();

    start_population
}

pub(crate) fn parse_dense_school(inputs: &Vec<String>) -> Vec<usize> {
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

    for fish in &mut *school {
        if *fish == 0 {
            *fish = 6;
            new_fish.push(8);
            continue;
        }

        *fish -= 1;
    }

    school.extend(new_fish);
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