use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

pub(crate) fn read_tokens(inputs: &Vec<String>) -> Vec<Vec<String>> {
    inputs
        .iter()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_string())
                .collect()
        })
        .collect()
}

#[cfg(test)]
#[test]
fn test_read_input() {
    let inputs = read_lines("data/day_10_sample.txt");
    let token_lines = read_tokens(&inputs);
    assert_eq!(token_lines[0].len(), 24)
}