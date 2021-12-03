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

pub(crate) fn o2_co2_ratings(inputs: &Vec<String>) -> (usize, usize) {
    let half_of_inputs = inputs.len() / 2;

    let mut bit_counts = bits_column_sum(inputs);

    let most_common_bits: Vec<usize> = get_most_common_bits(&half_of_inputs, &bit_counts);

    let mut filtered;
    for (idx, bit) in most_common_bits.iter().enumerate() {
        filtered = inputs.iter()
            .filter(|input| input[idx] == bit.to_string())
    }
    (12, 12)
}

fn get_most_common_bits(half_of_inputs: &usize, bit_counts: &Vec<usize>) -> Vec<usize> {
    bit_counts.iter()
        .map(|count| {
            if count >= &half_of_inputs {
                1 as usize
            } else {
                0 as usize
            }
        })
        .collect()
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
fn test_filter_function() {
    let inputs = read_lines("data/day_3_sample.txt");
    let (o2, co2) = o2_co2_ratings(&inputs);
    assert_eq!(o2, 23);
    assert_eq!(co2, 10);
}