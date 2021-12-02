use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

pub(crate) struct Movement {
    direction: String,
    amount: usize,
}

pub(crate) fn parse_movements(descriptions: &Vec<String>) -> Vec<Movement> {
    descriptions
        .iter()
        .map(|descr| {
            let parts: Vec<_> = descr.split(" ").collect();
            let direction = parts[0];
            let amount = parts[1];
            Movement{
                direction: String::from(direction),
                amount: amount.parse::<usize>().unwrap() }
        })
        .collect()
}

pub(crate) fn calculate_position(movements: Vec<Movement>) -> (i32, i32) {
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for movement in movements {
        match movement.direction.as_str() {
            "forward" => horizontal += movement.amount as i32,
            "backward" => horizontal -= movement.amount as i32,
            "up" => depth -= movement.amount as i32,
            "down" => depth += movement.amount as i32,
            _ => { unreachable!("Direction was not recognized: {}", movement.direction)},
        }
    }

    (horizontal, depth)
}

#[cfg(test)]
#[test]
fn test_movement() {
    let movement_descriptions = read_lines("data/day_2_sample.txt");
    let movements = parse_movements(&movement_descriptions);
    assert_eq!(movements.len(), 6);

    let (horizontal, depth) = calculate_position(movements);
    assert_eq!(horizontal, 15);
    assert_eq!(depth, 10);
}