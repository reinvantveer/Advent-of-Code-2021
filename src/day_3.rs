use std::collections::HashMap;
use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_3_input.txt");
    let (gamma, epsilon) = calculate_gamma_epsilon(&inputs);
    println!("Gamma is {}, epsilon is {}", gamma, epsilon);
    println!("These multiplied by each other is {}", gamma * epsilon);

    let o2_entry = filter_o2_input(&inputs);
    let co2_entry = filter_co2_input(&inputs);
    let o2_rating = usize_from_binary_string(o2_entry);
    let co2_rating = usize_from_binary_string(co2_entry);
    println!("O2 rating is {}, CO2 rating is {}", &o2_rating, &co2_rating);
    println!("These multiplied is {}", o2_rating * co2_rating);
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
    for pos in 0..input_0_len {
        println!("Iterating over {} of {} input positions", &pos, &input_0_len);
        // the most common bit has to be recalculated for each jump to the next bit
        // and re-applied on the remaining members of the hashmap
        let mut map_keys = Vec::new();

        for key in input_map.keys() {
            map_keys.push(key.to_string().clone())
        }

        // Update the most common bit value at position
        let most_common_at_pos = most_common_bit_for_pos(&map_keys, pos);
        println!("Updated most common bit {} for {} remaining entries at position {}",
            &most_common_at_pos, &map_keys.len(), &pos
        );

        filter_to_last_at_pos(&mut input_map, &mut correct_input_to_return, &pos, &most_common_at_pos);
        if correct_input_to_return != "".to_string() {
            break;
        }
    }

    correct_input_to_return
}

fn filter_to_last_at_pos(
    input_map: &mut HashMap<&String, Vec<usize>>,
    correct_input_to_return: &mut String,
    pos: &usize,
    most_common_at_pos: &usize
) {
    // Now, iterate over the hashmap and drop entries as long as they don't meet the most_common
    // bit until one entry in the hashmap remains
    for (key, value) in input_map.clone() {
        // Return if the last filtered-out value was found
        let value_at_pos = value.get(*pos).unwrap();

        if value_at_pos != most_common_at_pos && input_map.len() > 1 {
            input_map.remove(&key);
            println!("Removing {}: it does not have {} at position {}", &key, &most_common_at_pos, &pos);
        } else {
            println!("Keeping {} for {}", &key, &most_common_at_pos);
        }

        if input_map.len() == 1 {
            let last_remaining_key = input_map
                .keys()
                .map(|key| key.to_string())
                .collect::<Vec<String>>()
                .first()
                .unwrap()
                .clone()
                .to_string();
            *correct_input_to_return = last_remaining_key.clone();
            println!("Found last remaining match: {}", &last_remaining_key);
            break;
        } else {
            println!("{} entries left", &input_map.len());
        }

    }
}

pub(crate) fn filter_co2_input(inputs: &Vec<String>) -> String {
    let mut input_map = hashmap_from_inputs(inputs);
    let mut correct_input_to_return = "".to_string();
    let input_0_len = inputs.get(0).unwrap().len();

    // advance one position in the input length a time to filter values
    for pos in 0..input_0_len {
        println!("Iterating over {} of {} input positions", &pos, &input_0_len);
        // the most common bit has to be recalculated for each jump to the next bit
        // and re-applied on the remaining members of the hashmap
        let mut map_keys = Vec::new();

        for key in input_map.keys() {
            map_keys.push(key.to_string().clone())
        }

        // Update the most common bit value at position
        let least_common_at_pos = least_common_bit_for_pos(&map_keys, pos);
        println!("Updated most common bit {} for {} remaining entries at position {}",
                 &least_common_at_pos, &map_keys.len(), &pos
        );

        filter_to_last_at_pos(&mut input_map, &mut correct_input_to_return, &pos, &least_common_at_pos)
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
    let half_of_inputs= inputs.len() as f32 / 2 as f32;
    let bit_counts = bits_column_sum(inputs);
    let count_for_position = bit_counts.get(position).unwrap();

    if *count_for_position as f32 >= *&half_of_inputs {
        println!("Ones count {} at position {} over half ({}) of {}",
                 &count_for_position, position, &half_of_inputs, &inputs.len());
        1
    } else {
        0
    }
}

fn least_common_bit_for_pos(inputs: &Vec<String>, position: usize) -> usize {
    let half_of_inputs= inputs.len() as f32 / 2 as f32;
    let bit_counts = bits_column_sum(inputs);
    let count_for_position = bit_counts.get(position).unwrap();

    if *count_for_position as f32 >= *&half_of_inputs {
        println!("Ones count {} at position {} over half ({}) of {}",
                 &count_for_position, position, &half_of_inputs, &inputs.len());
        0
    } else {
        1
    }
}

pub(crate) fn usize_from_binary_string(input: String) -> usize {
    let mut number = 0 as usize;

    for (idx, char) in input.chars().enumerate() {
        let reversed_bit_order_idx = input.len() - 1 - idx;
        let bit = char.to_string().parse::<usize>().unwrap();
        number += bit * 2_usize.pow(reversed_bit_order_idx as u32);
    }

    number
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

    let o2_entry = filter_o2_input(&inputs);
    assert_eq!(o2_entry, "10111");

    let co2_entry = filter_co2_input(&inputs);
    assert_eq!(co2_entry, "01010");

    let o2 = usize_from_binary_string(o2_entry);
    let co2 = usize_from_binary_string(co2_entry);
    assert_eq!(o2, 23);
    assert_eq!(co2, 10);
}