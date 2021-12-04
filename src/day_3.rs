use std::collections::HashMap;
use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_3_input.txt");
    let (gamma, epsilon) = calculate_gamma_epsilon(&inputs);
    println!("Gamma is {}, epsilon is {}", gamma, epsilon);
    println!("These multiplied by each other is {}", gamma * epsilon);
}

pub(crate) fn calculate_gamma_epsilon(inputs: &Vec<String>) -> (usize, usize) {
    let mut gamma = 0;
    let mut epsilon= 0;
    let mut bit_counts = bits_column_sum(inputs);

    // Reverse to start with least significant bit first - it is on the rightmost element
    bit_counts.reverse();

    let half_of_inputs = &(inputs.len() / 2);

    for (idx, count) in bit_counts.iter().enumerate() {
        if count > half_of_inputs {
            gamma += 2_i32.pow(idx as u32);
        } else {
            epsilon += 2_i32.pow(idx as u32);
        };
    }

    (gamma as usize, epsilon as usize)
}

pub(crate) fn bits_column_sum(inputs: &Vec<String>) -> Vec<usize> {
    let mut bit_counts: Vec::<usize> = vec![0; inputs[0].len()];

    for input in inputs {
        for (idx, char) in input.chars().enumerate() {
            bit_counts[idx] += char.to_string().parse::<usize>().unwrap();
        }
    }
    bit_counts
}

fn filter_o2_input(inputs: &Vec<String>) -> String {
    let mut input_map = hashmap_from_inputs(inputs);

    let mut correct_input_to_return = "".to_string();
    let input_0_len = inputs.get(0).unwrap().len();

    // advance one position in the input length a time to filter values
    for pos in 0..input_0_len - 1 {
        // the most common bit has to be recalculated for each jump to the next bit
        // and re-applied on the remaining members of the hashmap
        let mut map_keys = Vec::new();

        for key in input_map.keys() {
            map_keys.push(key.to_string().clone())
        }

        let most_common = most_common_bit_for_pos(&map_keys, pos);

        // Now, iterate over the hashmap and drop entries as long as they don't meet the most_common
        // bit until one entry in the hashmap remains
        for (key, value) in input_map.clone() {
            // Return if the last filtered-out value was found
            if input_map.len() > 1 {
                let value_at_pos = value.get(pos).unwrap();

                if value_at_pos != &most_common {
                    input_map.remove(&key);
                }
            } else {
                correct_input_to_return = key.to_string();
                break;
            };
        }
    }

    correct_input_to_return
}

pub(crate) fn hashmap_from_inputs(inputs: &Vec<String>) -> HashMap<&String, Vec<usize>> {
    let mut input_map = HashMap::new();

    // Create hashmap for easier removal manipulation
    for input in inputs {
        let input_numbers: Vec<_> = input
            .chars()
            .map(|char| char.to_string().parse::<usize>().unwrap())
            .collect();
        input_map.insert(input, input_numbers);
    }
    input_map
}

fn most_common_bit_for_pos(inputs: &Vec<String>, position: usize) -> usize {
    let half_of_inputs = inputs.len() / 2;
    let bit_counts = bits_column_sum(inputs);
    let count_for_position = bit_counts.get(position).unwrap();

    if count_for_position >= &half_of_inputs {
        1
    } else {
        0
    }
}

#[cfg(test)]
#[test]
fn test_bit_counting() {
    let inputs = read_lines("data/day_3_sample.txt");
    let counts = bits_column_sum(&inputs);
    assert_eq!(counts, vec![7, 5, 8, 7, 5]);

    let mut gamma_bits = vec![0, 0, 0, 0, 0];
    for (idx, count) in counts.iter().enumerate() {
        let half_inputs_len = &(inputs.len() / 2);
        if count > half_inputs_len {
            gamma_bits[idx] = 1;
        }
    }
    assert_eq!(gamma_bits, vec![1, 0, 1, 1, 0]);

    let mut gamma = 0;
    gamma_bits.reverse();
    for (idx, bit) in gamma_bits.iter().enumerate() {
        gamma += bit * 2_i32.pow(idx as u32);
    }
    assert_eq!(gamma, 22);
}

#[test]
fn test_gamma_calculation() {
    let inputs = read_lines("data/day_3_sample.txt");
    let (gamma, epsilon) = calculate_gamma_epsilon(&inputs);
    assert_eq!(gamma, 22);
    assert_eq!(epsilon, 9);
}

#[test]
fn test_bit_filter() {
    let inputs = read_lines("data/day_3_sample.txt");
    let most_common_bit_1 = most_common_bit_for_pos(&inputs, 0);
    assert_eq!(most_common_bit_1, 1);

    let hashmap = hashmap_from_inputs(&inputs);
    let last_entry = hashmap.get(&"01010".to_string()).unwrap();
    assert_eq!(*last_entry, vec![0, 1, 0, 1, 0]);

    let o2_filter = filter_o2_input(&inputs);
    assert_eq!(o2_filter, "10111");

    // assert_eq!(o2, 23);
    // assert_eq!(co2, 10);
}