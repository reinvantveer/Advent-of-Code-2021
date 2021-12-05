use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

type Board = Vec<Vec<Option<usize>>>;

/// Parses the bingo data, consisting of a first line of bingo number calls,
/// followed by blank-line separated bingo boards
pub(crate) fn parse_bingo_data(inputs: Vec<String>) -> (Vec<usize>, Vec<Board>) {
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
            .map(|n| Some(n.parse::<usize>().unwrap()))
            .collect::<Vec<Option<usize>>>();
        // println!("Appending {:?}", &numbers);
        board.push(numbers.clone());

        if board.len() == 5 {
            boards.push(board);
            board = Vec::new();
        }
    }

    (number_calls, boards)
}

pub(crate) fn mark_number(boards: &mut Vec<Board>, number: usize) {
    for board in boards {
        for row in board {
            for number_entry in row {
                if let Some(board_number) = number_entry {
                    if *board_number == number {
                        *number_entry = None;
                    }
                }
            }
        }
    }
}

/// Mark numbers on all boards until we have a bingo, which in that case returns Some(Board)
pub(crate) fn mark_until_bingo(numbers: Vec<usize>, boards: &mut Vec<Board>) -> Option<Board> {
    // The default option: None
    let mut board = None;

    for number in numbers {
        mark_number(boards, number);
        if let Some(board_idx) = bingo(&boards) {
            println!("Bingo on board {}", board_idx);
            board = Some(boards.get(board_idx).unwrap().clone());
        }
    }

    board
}

pub(crate) fn bingo(boards: &Vec<Board>) -> Option<usize> {
    None
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

#[test]
fn test_mark_number_on_board() {
    let inputs = read_lines("data/day_4_sample.txt");
    let (_, mut boards) = parse_bingo_data(inputs);
    mark_number(&mut boards, 22);

    // Validate that the very first number on the first board is now crossed off
    let first_board = boards.first().unwrap();
    let first_board_first_row = first_board.first().unwrap();
    let first_board_first_row_first_entry = first_board_first_row.first().unwrap();
    assert_eq!(*first_board_first_row_first_entry, None);

    // Validate that the next number isn't touched
    let first_board_first_row_second_entry = first_board_first_row.get(1).unwrap();
    assert_eq!(*first_board_first_row_second_entry, Some(13));
}

#[test]
fn test_bingo(){
    let inputs = read_lines("data/day_4_sample.txt");
    let (number_calls, mut boards) = parse_bingo_data(inputs);

    // The 13th draw should result in bingo on the third board
    for number in number_calls[..13].iter() {
        mark_number(&mut boards, *number)
    }
    assert_eq!(bingo(&boards), Some(3));

    let first_13_numbers = number_calls[..13]
        .iter()
        .map(|n| *n)
        .collect();
    let maybe_bingo = mark_until_bingo(first_13_numbers, &mut boards).unwrap();
    let first_board_entry = *maybe_bingo.first().unwrap().first().unwrap();
    assert_eq!(first_board_entry, Some(14));
}