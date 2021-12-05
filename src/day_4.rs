use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

/// Parses the bingo data, consisting of a first line of bingo number calls,
/// followed by blank-line separated bingo boards
pub(crate) fn parse_bingo_data(inputs: Vec<String>) -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    let numbers = inputs.iter().next().unwrap();
    let number_calls = numbers
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = Vec::new();
    let mut board = Vec::new();

    for line in inputs[2..].iter() {
        // Skip empty lines
        if line.to_string() == "".to_string() { continue; }

        let numbers = &mut line
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        // println!("Appending {:?}", &numbers);
        board.push(numbers.clone());

        if board.len() == 5 {
            boards.push(board);
            board = Vec::new();
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
    );
    assert_eq!(boards.len(), 3);

    let first_board = boards.first().unwrap();
    let last_board = boards.last().unwrap();
    assert_eq!(first_board.len(), 5);
    assert_eq!(last_board.len(), 5);

    let first_board_first_row = first_board.first().unwrap();
    let last_board_last_row = last_board.last().unwrap();
    assert_eq!(first_board_first_row.len(), 5);
    assert_eq!(last_board_last_row.len(), 5);

}