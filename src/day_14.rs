use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

pub(crate) struct InsertRule {
    first_match: String,
    adjacent_match: String,
    to_insert: String,
}

pub(crate) fn parse_inputs(inputs: &Vec<String>) -> (Vec<String>, Vec<InsertRule>) {
    let mut rules = Vec::new();

    let template = inputs[0]
        .split("")
        .map(|c| c.to_string())
        .filter(|s| s != &"".to_string())
        .collect();

    for instruction in inputs[2..].iter() {
        let rule_parts = instruction.split(" -> ").collect::<Vec<_>>();

        let first_match = rule_parts[0]
            .chars()
            .collect::<Vec<_>>()[0]
            .to_string();

        let adjacent_match = rule_parts[0]
            .chars()
            .collect::<Vec<_>>()[1]
            .to_string();

        let to_insert = rule_parts[1].to_string();

        rules.push(InsertRule {
            first_match,
            adjacent_match,
            to_insert,
        });
    }

    (template, rules)
}

#[cfg(test)]
#[test]
fn test_parse() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (template, rules) = parse_inputs(&inputs);

    assert_eq!(template, vec!["N", "N", "C", "B"]);
    assert_eq!(rules.len(), 16)
}