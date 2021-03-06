use std::collections::HashMap;
use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_14_input.txt");
    let (mut template, rules) = parse_inputs(&inputs);

    for _ in 0..10 {
        expand_polymer(&mut template, &rules);
    }

    let (min, max) = count_elems(&template);
    println!("The max {} minus min {} is {} after 10 iterations", max, min, max - min);

    let (template, rules) = parse_inputs(&inputs);
    let rules_map = rules_as_map(&rules);
    let (pair_counts, start) = even_faster_expand_iter(&template, &rules_map, 40);
    let (min, max) = min_max_from_pairs(&pair_counts, start);
    println!("The max {} minus min {} is {} after 40 iterations", max, min, max - min);
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

pub(crate) fn rules_as_map(rules: &Vec<InsertRule>) -> HashMap<String, String> {
    let mut rules_map = HashMap::new();

    for rule in rules {
        let key = rule.first_match.clone() + &rule.adjacent_match;
        let value = rule.to_insert.clone();
        rules_map.insert(key, value);
    }

    rules_map
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

pub(crate) fn fast_expand(template: &mut Vec<String>, rules_map: &HashMap<String, String>) {
    // Look behind: cursor starts with 1 and finds a look-behind match in the rules map
    let mut cursor = 1;
    let mut template_size = template.len();

    while cursor < template_size {
        // Look behind
        let sequence_at_cursor = template[cursor - 1].clone() + &template[cursor].clone();

        if let Some(to_insert) = rules_map.get(sequence_at_cursor.as_str()) {
            template.insert(cursor, to_insert.clone());
            // Skip over the just inserted element
            cursor += 1;
            template_size += 1;
        }

        cursor += 1;
    }
}

pub(crate) fn even_faster_expand_iter(
    template: &Vec<String>,
    rules_map: &HashMap<String, String>,
    iterations: usize,
) -> (HashMap<String, usize>, String) {
    let mut pair_counts: HashMap<String, usize> = HashMap::new();

    // Start pair is needed for the count
    let mut start_pair = template[0].clone() + &template[1];

    // Build initial hashmap
    for elem_idx in 0..template.len() - 1 {
        let pair_at_idx = template[elem_idx].clone() + &template[elem_idx + 1];
        let entry = pair_counts.entry(pair_at_idx).or_insert(0);
        *entry += 1;
    }

    for _ in 0..iterations {
        even_faster_expand(&mut pair_counts, rules_map, &mut start_pair)
    }

    (pair_counts, start_pair)
}

pub fn even_faster_expand(
    pair_counts: &mut HashMap<String, usize>,
    rules_map: &HashMap<String, String>,
    start_pair: &mut String
) {
    let unmodified_counts = pair_counts.clone();

    // Start gets modified only once each expansion: it is the first pair
    let mut is_start_modified = false;

    for (unmodified_pair, unmodified_count) in unmodified_counts {
        if rules_map.get(&unmodified_pair) == None { continue; }
        if unmodified_count == 0 { continue; }

        let elem_to_insert = rules_map.get(&unmodified_pair).unwrap();

        // First: decrease the count for the pair that gets split to insert a new element
        let pair_to_split_count = pair_counts.get_mut(&unmodified_pair).unwrap();

        // Inserting destroys all previous pairs of the type being split
        *pair_to_split_count -= unmodified_count;

        // Then: increase the count on what pairs are added to left and right of the insert
        let pair_chars = unmodified_pair.chars().collect::<Vec<_>>();
        let first_elem = pair_chars[0].to_string();
        let second_elem = pair_chars[1].to_string();

        let new_pair_to_left = first_elem + &elem_to_insert;
        let new_pair_to_right = elem_to_insert.to_string() + &second_elem;

        let entry_to_left = pair_counts.entry(new_pair_to_left.clone()).or_insert(0);
        *entry_to_left += unmodified_count;

        let entry_to_right = pair_counts.entry(new_pair_to_right).or_insert(0);
        *entry_to_right += unmodified_count;

        // Update the start pair to the new pair to the left
        if unmodified_pair == start_pair.clone() && !is_start_modified{
            *start_pair = new_pair_to_left;
            is_start_modified = true;
        }
    }
}

pub(crate) fn min_max_from_pairs(pair_counts: &HashMap<String, usize>, start_pair: String) -> (usize, usize) {
    let mut elem_counts: HashMap<String, usize> = HashMap::new();

    for (pair, count) in pair_counts {
        if *count == 0 { continue; }

        let first_elem = pair.chars().collect::<Vec<_>>()[0].to_string();
        let last_elem = pair.chars().collect::<Vec<_>>()[1].to_string();

        let elem_count = elem_counts.entry(last_elem.clone()).or_insert(0);
        *elem_count += *count;

        // Except for the start pair: it has a non-overlapping first element
        if pair == &start_pair {
            let elem_count = elem_counts.entry(first_elem).or_insert(0);
            *elem_count += *count;
        }
    }

    let mut counts = elem_counts.values().collect::<Vec<_>>();
    counts.sort();

    let min = *counts.first().unwrap().clone();
    let max = *counts.last().unwrap().clone();

    (min, max)
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

pub(crate) fn count_elems(template: &Vec<String>) -> (usize, usize) {
    let mut counts = HashMap::new();

    for elem in template {
        let counter = counts.entry(elem).or_insert(0);
        *counter += 1;
    }

    let mut values = counts.values().collect::<Vec<_>>();
    values.sort();

    (**values.first().unwrap(), **values.last().unwrap())
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
    // Starts from       vec!["N", "N", "C", "B"]; cursor = 1
    // To:               vec!["N", "C", "N", "C", "B"]; cursor is now 3
    // To:               vec!["N", "C", "N", "B", "C", "B"]; cursor is now 5
    // To:               vec!["N", "C", "N", "B", "C", "B"]; cursor is now 7
    // To:               vec!["N", "C", "N", "B", "C", "H", "B"]; cursor is now 9, which is > size
    assert_eq!(template, vec!["N", "C", "N", "B", "C", "H", "B"]);

    expand_polymer(&mut template, &rules);
    assert_eq!(template, vec!["N", "B", "C", "C", "N", "B", "B", "B", "C", "B", "H", "C", "B"]);

    expand_polymer(&mut template, &rules);
    assert_eq!(template,
               vec!["N", "B", "B", "B", "C", "N", "C", "C", "N", "B", "B", "N", "B", "N", "B", "B", "C", "H", "B", "H", "H", "B", "C", "H", "B"]);
}

#[test]
fn test_manual_fast_iterate() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (mut template, rules) = parse_inputs(&inputs);
    let rules_map = rules_as_map(&rules);

    fast_expand(&mut template, &rules_map);
    assert_eq!(template, vec!["N", "C", "N", "B", "C", "H", "B"]);

    fast_expand(&mut template, &rules_map);
    assert_eq!(template, vec!["N", "B", "C", "C", "N", "B", "B", "B", "C", "B", "H", "C", "B"]);

    fast_expand(&mut template, &rules_map);
    assert_eq!(template,
               vec!["N", "B", "B", "B", "C", "N", "C", "C", "N", "B", "B", "N", "B", "N", "B", "B", "C", "H", "B", "H", "H", "B", "C", "H", "B"]);
}

#[test]
fn test_even_faster_iterate() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (template, rules) = parse_inputs(&inputs);
    let rules_map = rules_as_map(&rules);

    // NCNBCHB
    let (pair_counts, start) = even_faster_expand_iter(&template, &rules_map, 1);
    let (min, max) = min_max_from_pairs(&pair_counts, start.clone());
    assert_eq!(max - min, 2 - 1);
    assert_eq!(start, "NC");
    assert_eq!(pair_counts, HashMap::from([
        ("NN".to_string(), 0),
        ("CB".to_string(), 0),
        ("NB".to_string(), 1),
        ("BC".to_string(), 1),
        ("CN".to_string(), 1),
        ("NC".to_string(), 1),
        ("CH".to_string(), 1),
        ("HB".to_string(), 1),
    ]));

    // NBCCNBBBCBHCB
    // 1 H, 2 N's, 4 C's, 6 B's
    let (pair_counts, start) = even_faster_expand_iter(&template, &rules_map, 2);
    let (min, max) = min_max_from_pairs(&pair_counts, start.clone());
    assert_eq!(max - min, 6 - 1);
    assert_eq!(start, "NB");
    assert_eq!(pair_counts, HashMap::from([
        ("NN".to_string(), 0),
        ("CH".to_string(), 0),
        ("NC".to_string(), 0),
        ("HB".to_string(), 0),
        ("NB".to_string(), 2),
        ("BC".to_string(), 2),
        ("CC".to_string(), 1),
        ("CN".to_string(), 1),
        ("BB".to_string(), 2),
        ("CB".to_string(), 2),
        ("BH".to_string(), 1),
        ("HC".to_string(), 1),
    ]));

    // NBBBCNCCNBBNBNBBCHBHHBCHB
    // 5 Ns, 5 Cs, 4 Hs, 11 Bs
    let (pair_counts, start) = even_faster_expand_iter(&template, &rules_map, 3);
    let (min, max) = min_max_from_pairs(&pair_counts, start.clone());
    assert_eq!(max - min, 11 - 4);
    assert_eq!(start, "NB");
    assert_eq!(pair_counts, HashMap::from([
        ("NN".to_string(), 0),
        ("HC".to_string(), 0),
        ("CB".to_string(), 0),
        ("NB".to_string(), 4),
        ("BB".to_string(), 4),
        ("BC".to_string(), 3),
        ("CN".to_string(), 2),
        ("CC".to_string(), 1),
        ("CC".to_string(), 1),
        ("BN".to_string(), 2),
        ("CH".to_string(), 2),
        ("BH".to_string(), 1),
        ("HH".to_string(), 1),
        ("NC".to_string(), 1),
        ("HB".to_string(), 3),
    ]));

    let (pair_counts, start) = even_faster_expand_iter(&template, &rules_map, 10);
    let (min, max) = min_max_from_pairs(&pair_counts, start);
    assert_eq!(max, 1749);
    assert_eq!(min, 161);
    assert_eq!(max - min, 1588);
}

#[test]
fn test_count() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (mut template, rules) = parse_inputs(&inputs);

    for _ in 0..10 {
        expand_polymer(&mut template, &rules);
    }

    let (min, max) = count_elems(&template);
    assert_eq!(max - min, 1588);
}

#[test]
fn test_pair_count_hashmap_count() {
    let inputs = read_lines("data/day_14_sample.txt");
    let (template, rules) = parse_inputs(&inputs);
    let rules_map = rules_as_map(&rules);

    let (pair_counts, start) = even_faster_expand_iter(&template, &rules_map, 40);
    let (min, max) = min_max_from_pairs(&pair_counts, start);
    assert_eq!(max - min, 2188189693529);
}
