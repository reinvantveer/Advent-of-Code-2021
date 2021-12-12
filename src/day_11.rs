use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

pub(crate) fn octopi_from_input(input: &Vec<String>) -> Vec<Vec<usize>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
#[test]
fn test_parse() {
    let input = read_lines("data/day_11_sample.txt");
    let octopi = octopi_from_input(&input);
    assert_eq!(octopi.len(), 10);
    assert_eq!(octopi[0].len(), 10);
}
