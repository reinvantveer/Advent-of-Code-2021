use advent_of_code_2021::read_lines;

pub(crate) fn run() {

}

#[derive(Clone)]
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

pub(crate) fn expand_polymer(template: &mut Vec<String>, rules: &Vec<InsertRule>) {
    // Create a set of insert positions that will contain all the indexes of where the new
    // element is to be inserted
    let mut insert_positions_per_rule = vec![Vec::new(); rules.len()];

    // First, find all the matching positions before mutation
    for (rule_idx, rule) in rules.iter().enumerate() {
        let positions = find_matches(&template, &rule);
        insert_positions_per_rule[rule_idx] = positions;
    }

    for rule_idx in 0..insert_positions_per_rule.len() {
        for position_idx in 0..insert_positions_per_rule[rule_idx].len() {
            // Insert the new element into the position
            template.insert(
                insert_positions_per_rule[rule_idx][position_idx],
                rules[rule_idx].to_insert.clone())
            ;

            // Update the positions for this rule
            for further_insert_idx in position_idx + 1..insert_positions_per_rule[rule_idx].len() {
                insert_positions_per_rule[rule_idx][further_insert_idx] += 1;
            }

            // Update the positions for the following rules
            for following_rules_idx in rule_idx + 1..insert_positions_per_rule.len() {
                for following_position_idx in 0..insert_positions_per_rule[following_rules_idx].len() {
                    if insert_positions_per_rule[following_rules_idx][following_position_idx] >= insert_positions_per_rule[rule_idx][position_idx] {
                        insert_positions_per_rule[following_rules_idx][following_position_idx] += 1;
                    }
                }
            }
        }
    }
}

pub(crate) fn find_matches(template: &Vec<String>, rule: &InsertRule) -> Vec<usize> {
    let mut matches = Vec::new();

    let elem_idxs = 0..template.len() - 1;
    for el_idx in elem_idxs {
        if template[el_idx] == rule.first_match && template[el_idx + 1] == rule.adjacent_match {
            // The insertion position would be one after the found position
            matches.push(el_idx + 1);
        }
    }
    matches
}

#[cfg(test)]
#[test]
fn test_parse() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (template, rules) = parse_inputs(&inputs);

    assert_eq!(template, vec!["N", "N", "C", "B"]);
    assert_eq!(rules.len(), 16)
}

#[test]
fn test_matching_seqs() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (template, rules) = parse_inputs(&inputs);

    let n_n = rules[7].clone();
    let positions = find_matches(&template, &n_n);
    assert_eq!(positions, vec![1])

}

#[test]
fn test_manual_iterate() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (mut template, rules) = parse_inputs(&inputs);

    expand_polymer(&mut template, &rules);
    assert_eq!(template, vec!["N", "C", "N", "B", "C", "H", "B"]);

    expand_polymer(&mut template, &rules);
    assert_eq!(template, vec!["N", "B", "C", "C", "N", "B", "B", "B", "C", "B", "H", "C", "B"]);

}