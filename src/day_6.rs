use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

pub(crate) fn parse_school(inputs: &Vec<String>) -> Vec<usize> {
    let start_population = inputs[0]
        .split(",")
        .map(|f| f.parse::<usize>().unwrap())
        .collect();

    start_population
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
    for _ in 0..days {
        procreate(school);
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