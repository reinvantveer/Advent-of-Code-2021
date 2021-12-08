use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_5_input.txt");
    let (hor_lines, ver_lines, dia_lines) = parse_lines(&inputs, false);
    let mut all_lines = hor_lines;
    all_lines.extend(ver_lines);
    all_lines.extend(dia_lines);
    let grid = grid_sum_from_lines(&all_lines);
    let score = hotspots_count(&grid);

    println!("The total number of crossing horizontal and vertical lines in the grid is {}", &score);
}

type Point = (usize, usize);
type Line = Vec<Point>;
type Grid = Vec<Vec<usize>>;

pub(crate) fn parse_lines(inputs: &Vec<String>, parse_diagonals: bool) -> (Vec<Line>, Vec<Line>, Vec<Line>) {
    let mut hor_lines = Vec::new();
    let mut ver_lines = Vec::new();
    let mut dia_lines = Vec::new();

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
            // println!("Line is vertical");
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
            // println!("Line is horizontal");
            let range = match end_point[0] > start_point[0] {
                true => start_point[0]..end_point[0] + 1,
                false => end_point[0]..start_point[0] + 1,
            };

            let line = (range)
                .map(|x| (x, start_point[1]))
                .collect::<Line>();
            hor_lines.push(line);
        } else {
            // println!("Line is diagonal");
            // Skip if we're on part 1
            if !parse_diagonals { continue; }

            let x_increases = end_point[0] > start_point[0];
            let x_range;

            match x_increases {
                true => x_range = start_point[0]..end_point[0] + 1,
                false => x_range = end_point[0]..start_point[0] + 1,
            };

            let start_y = start_point[1];
            let end_y = end_point[1];

            let y_increases = end_y > start_y;
            let mut y_range = match y_increases {
                true => (start_y..end_y + 1).collect::<Vec<_>>(),
                false => (end_y..start_y + 1).collect::<Vec<_>>(),
            };
            // If either x range is reversed XOR the y range: reverse the y range to align the range orders
            if x_increases ^ y_increases { y_range.reverse() }

            let line = (x_range).zip(y_range)
                .collect::<Line>();
            dia_lines.push(line);
        }
    }

    (hor_lines, ver_lines, dia_lines)
}

pub(crate) fn grid_sum_from_lines(lines: &Vec<Line>) -> Grid {
    let mut grid_size = 10;
    let mut all_points = Vec::new();

    // Calculate grid size from points max
    // And gather all points in a simpler vec
    for line in lines {
        for &point in line {
            all_points.push(point);

            if point.0 > grid_size {
                grid_size = point.0;
            } else if point.1 > grid_size {
                grid_size = point.1;
            }
        }
    }

    let mut grid: Grid = vec![vec![0_usize; grid_size + 1]; grid_size + 1];

    for point in all_points {
        grid[point.0][point.1] += 1;
    }

    grid
}

pub(crate) fn hotspots_count(grid: &Grid) -> usize {
    let mut score = 0;

    for line in grid {
        for entry in line {
            if *entry > 1 {
                score += 1;
            }
        }
    }

    score
}

#[cfg(test)]
#[test]
fn test_straight_line_parser() {
    let inputs = read_lines("data/day_5_sample.txt");
    let (hor_lines, ver_lines, dia_lines) = parse_lines(&inputs, false);

    assert_eq!(hor_lines.len(), 4);
    assert_eq!(ver_lines.len(), 2);
    assert_eq!(dia_lines.len(), 0);

    assert_eq!(hor_lines[0], vec![(0, 9), (1, 9), (2, 9), (3, 9), (4, 9), (5, 9)]);
    assert_eq!(ver_lines[0], vec![(2, 1), (2, 2)]);
}

#[test]
fn test_grid_sum_from_lines() {
    let inputs = read_lines("data/day_5_sample.txt");
    let (hor_lines, ver_lines, _) = parse_lines(&inputs, false);
    let mut all_lines = hor_lines;
    all_lines.extend(ver_lines);
    let grid = grid_sum_from_lines(&all_lines);
    assert_eq!(grid[0][0], 0);
    assert_eq!(grid[3][4], 2);
}

#[test]
fn test_number_of_points_where_at_least_two_straight_lines_overlap() {
    let inputs = read_lines("data/day_5_sample.txt");
    let (hor_lines, ver_lines, _) = parse_lines(&inputs, false);
    let mut all_lines = hor_lines;
    all_lines.extend(ver_lines);
    let grid = grid_sum_from_lines(&all_lines);

    let score = hotspots_count(&grid);
    assert_eq!(score, 5);
}

#[test]
fn test_diagonal_line_parser() {
    let inputs = read_lines("data/day_5_sample.txt");
    let (_, _, dia_lines) = parse_lines(&inputs, true);

    assert_eq!(dia_lines.len(), 4);

    let mut expected_line = vec![(8, 0), (7, 1), (6, 2), (5, 3), (4, 4), (3, 5), (2, 6), (1, 7), (0, 8)];
    expected_line.reverse();
    assert_eq!(dia_lines[0], expected_line);

    expected_line = vec![(2, 0), (3, 1), (4, 2), (5, 3), (6, 4)];
    assert_eq!(dia_lines[1], expected_line);

    expected_line = vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6), (7, 7), (8, 8)];
    assert_eq!(dia_lines[2], expected_line);

    expected_line = vec![(5, 5), (6, 4), (7, 3), (8, 2)];
    assert_eq!(dia_lines[3], expected_line);
}

#[test]
fn test_straight_and_diagonal_grid_hotspot_counts() {
    let inputs = read_lines("data/day_5_sample.txt");
    let (hor_lines, ver_lines, dia_lines) = parse_lines(&inputs, true);
    let mut all_lines = hor_lines;
    all_lines.extend(ver_lines);
    all_lines.extend(dia_lines);

    let grid = grid_sum_from_lines(&all_lines);
    let num_hotspots = hotspots_count(&grid);

    assert_eq!(num_hotspots, 12);

}