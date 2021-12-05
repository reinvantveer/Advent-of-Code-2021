use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

/// Parses the bingo data, consisting of a first line of bingo number calls,
/// followed by blank-line separated bingo boards
pub(crate) fn parse_bingo_data(inputs: Vec<String>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut numbers = inputs.iter().next().unwrap();
    let number_calls = numbers
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::new();

    let mut row = 0;
    let mut column = 0;
    let mut board = Vec::new();

    for line in inputs[1..].iter() {
        row += 1;

        // Reset board
        if line == &"".to_string() {
            boards.push(board);
            board = Vec::new();
            row = 0;
            column = 0;
        }

    }

    (number_calls, boards)
}

#[cfg(test)]
#[test]
fn test_bingo_data_parser() {
    let inputs = read_lines("data/day_4_sample.txt");
    let (number_calls, boards) = parse_bingo_data(inputs);
    assert_eq!(
        number_calls,
        vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]
    )

}