use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

type Point = (usize, usize);
type Line = Vec<Point>;

pub(crate) fn parse_lines(inputs: &Vec<String>) -> (Vec<Line>, Vec<Line>) {
    let mut hor_lines = Vec::new();
    let mut ver_lines = Vec::new();

    for input in inputs {
        let start_end = input
            .split(" -> ")
            .collect::<Vec<_>>();

        let start_str = *start_end.get(0).unwrap();
        let end_str = *start_end.get(1).unwrap();

        let start_point = start_str
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let end_point = end_str
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if start_point[0] == end_point[0] {
            println!("Line is vertical");
            // Range over y positions according to direction. Is it away from the origin?
            let range = match end_point[1] > start_point[1] {
                true => start_point[1]..end_point[1] + 1,
                false => end_point[1]..start_point[1] + 1,
            };

            let line = (range)
                .map(|y| (start_point[0], y))
                .collect::<Line>();
            ver_lines.push(line);
        } else if start_point[1] == end_point[1] {
            println!("Line is horizontal");
            let line = (start_point[0]..end_point[0] + 1)
                .map(|x| (x, start_point[1]))
                .collect::<Line>();
            hor_lines.push(line);
        }
    }

    (hor_lines, ver_lines)
}

#[cfg(test)]
#[test]
fn test_straight_line_parser() {
    let inputs = read_lines("data/day_5_sample.txt");
    let (hor_lines, ver_lines) = parse_lines(&inputs);

    assert_eq!(hor_lines[0], vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)]);
    assert_eq!(ver_lines[0], vec![(2, 1), (2, 2)]);
}