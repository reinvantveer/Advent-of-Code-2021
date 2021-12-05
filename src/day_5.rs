use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

type Point = (usize, usize);

pub(crate) fn parse_lines(inputs: &Vec<String>) -> (Vec<Point>, Vec<Point>) {
    let hori_lines = Vec::new();
    let verti_lines = Vec::new();

    for input in inputs {
        let start_end = input
            .split(" -> ")
            .collect::<Vec<_>>();

        let start_str = *start_end.get(0).unwrap();

        let start_point = start_str
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
    }

    (hori_lines, verti_lines)
}

#[cfg(test)]
#[test]
fn test_straight_line_parser() {
    let inputs = read_lines("data/day_5_sample.txt");
    let (hori_lines, vert_lines) = parse_lines(&inputs);
}