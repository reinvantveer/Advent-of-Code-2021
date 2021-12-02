use advent_of_code_2021::read_lines;

pub(crate) fn run () {
    let inputs = read_lines("data/part_1_input.txt");

    let measurements: Vec<i32> = inputs
        .iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;
    for (idx, measurement) in measurements.iter().enumerate() {
        if idx == 0 { continue; };
        if measurement > &measurements[idx - 1] { increases += 1}
    }
    println!("There are {} increases in depth", increases);
}

pub(crate) fn window_increases(measurements: Vec<i32>, window_size: usize) -> usize {
    let mut increases = 0;
    let mut previous_window_sum = -1;

    for window_slice in measurements.windows(window_size) {
        let window_sum = window_slice.iter().sum();

        if previous_window_sum == -1 {
            println!("Skipping first measurement window: no window diff yet");
            previous_window_sum = window_sum;
        };

        if window_sum > previous_window_sum { increases += 1 };

        // Update for next iteration
        previous_window_sum = window_sum;
    }

    increases
}

#[cfg(test)]
#[test]
fn test_sliding_triplet_window() {
    let measurements: Vec<_> = read_lines("data/sample.txt")
        .iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect();

    let increases = window_increases(measurements.clone(), 1);
    assert_eq!(increases, 7);

    let increases = window_increases(measurements, 3);
    assert_eq!(increases, 5)
}