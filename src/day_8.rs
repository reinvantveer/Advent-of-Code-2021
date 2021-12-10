use std::usize;
use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_8_input.txt");
    let segment_sets = parse_input_output_signals(&inputs);
    let easy_count = count_easy_segments(&segment_sets);
    println!("The total of easy digits is {}", easy_count);

    let full_signals = parse_full_signals(&segment_sets);
    let total = sum_outputs(&full_signals);
    println!("The total of the display numbers is {}", total);
}

type SegmentPatternSet = (Vec<String>, Vec<String>);
type Signal = Vec<u8>;
type SegmentSignalSet = (Vec<Signal>, Vec<Signal>);

pub(crate) fn parse_input_output_signals(inputs_outputs: &Vec<String>) -> Vec<SegmentPatternSet> {
    let mut segment_sets = Vec::new();

    for i_o in inputs_outputs {
        let pipe_split = i_o.split(" | ")
            .map(|pattern| pattern.to_string() )
            .collect::<Vec<String>>();

        let inputs = pipe_split[0]
            .split(" ")
            .map(|pattern| pattern.to_string() )
            .collect();

        let outputs = pipe_split[1]
            .split(" ")
            .map(|pattern| pattern.to_string() )
            .collect();

        segment_sets.push((inputs, outputs));
    }

    segment_sets
}

pub(crate) fn parse_full_signals(segment_sets: &Vec<SegmentPatternSet>) -> Vec<SegmentSignalSet>{
    let mut segment_signal_sets = Vec::new();

    for pattern_set in segment_sets {
        let mut signal_set = (Vec::new(), Vec::new());

        for input in &pattern_set.0 {
            let signals = input
                .chars()
                .map(|c| {
                    let ascii_number = c.to_string().into_bytes();
                    ascii_number[0] as u8 - 97
                })
                .collect();

            signal_set.0.push(signals)
        }

        for output in &pattern_set.1 {
            let signals = output
                .chars()
                .map(|c| {
                    let ascii_number = c.to_string().into_bytes();
                    ascii_number[0] as u8 - 97
                } )
                .collect();

            signal_set.1.push(signals)
        }

        segment_signal_sets.push(signal_set);
    }

    segment_signal_sets
}

///
//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg
//

// only one contains  2 segments
// only four contains 4 segments
// only seven contains 3 segments
// only eight contains 7 segments

// three is the only five segment digit containing all segments of the seven
// nine is the only six-segment digit containing all the elements of the three
// The bottom left segment is the difference between the eight and the nine
// five is the only five-segment digit having no bottom left segment
// two is the only five-segment digit that is not a three and not a five
// zero is the only six-segment digit that is not a nine and has all the elements of the one
// six is the only six-segment digit that is not a zero and not a nine
pub(crate) fn decode_inputs(signal_set: &SegmentSignalSet) -> Vec<Signal> {
    let inputs = &signal_set.0;

    // only one contains  2 segments
    let mut one = inputs
        .iter()
        .filter(|s| s.len() == 2)
        .collect::<Vec<_>>()[0]
        .clone();
    one.sort();

    // only four contains 4 segments
    let mut four = inputs
        .iter()
        .filter(|s| s.len() == 4)
        .collect::<Vec<_>>()[0]
        .clone();
    four.sort();

    // only the seven contains 3 segments
    let mut seven = inputs
        .iter()
        .filter(|s| s.len() == 3)
        .collect::<Vec<_>>()[0]
        .clone();
    seven.sort();

    // only eight contains 7 segments
    let mut eight = inputs
        .iter()
        .filter(|s| s.len() == 7)
        .collect::<Vec<_>>()[0]
        .clone();
    eight.sort();

    // the three is the only five segment digit containing all segments of the seven
    let three_candidates = inputs
        .iter()
        .filter(|s| {
            s.len() == 5
                && s.contains(&seven[0])
                && s.contains(&seven[1])
                && s.contains(&seven[2])
        })
        .collect::<Vec<_>>();
    assert_eq!(three_candidates.len(), 1);

    let mut three = three_candidates[0].clone();
    three.sort();

    // nine is the only six-segment digit containing all the elements of the three
    let nine_candidates = inputs
        .iter()
        .filter(|s| {
            s.len() == 6
                && s.contains(&three[0])
                && s.contains(&three[1])
                && s.contains(&three[2])
                && s.contains(&three[3])
                && s.contains(&three[4])
        })
        .collect::<Vec<_>>();
    assert_eq!(nine_candidates.len(), 1);
    let mut nine= nine_candidates[0].clone();
    nine.sort();

    // The bottom left segment is the only one present in the eight but not the nine
    let bottom_left_candidates = eight
        .iter()
        .filter(|num| !nine.contains(num))
        .collect::<Vec<_>>();
    assert_eq!(bottom_left_candidates.len(), 1);
    let bottom_left = bottom_left_candidates[0].clone();

    // five is the only five-segment digit not being a three, having no bottom left segment
    let five_candidates = inputs
        .iter()
        .filter(|input| {
            input.len() == 5
                // Can't compare with `three`: it's already sorted, `input` is unsorted
                && *input != three_candidates[0]
                && !input.contains(&bottom_left)
        })
        .collect::<Vec<_>>();
    assert_eq!(five_candidates.len(), 1);
    let mut five= five_candidates[0].clone();
    five.sort();

    // two is the only five-segment digit that is not a three and not a five
    let two_candidates = inputs
        .iter()
        .filter(|input| {
            input.len() == 5
                // Can't compare with `three`: it's already sorted, `input` is unsorted
                && *input != three_candidates[0]
                && *input != five_candidates[0]
        })
        .collect::<Vec<_>>();
    assert_eq!(two_candidates.len(), 1);
    let mut two = two_candidates[0].clone();
    two.sort();

    // zero is the only six-segment digit that is not a nine and has all the elements of the one
    let zero_candidates = inputs
        .iter()
        .filter(|input| {
            input.len() == 6
                // Can't compare with `nine`: it's already sorted, `input` is unsorted
                && *input != nine_candidates[0]
                && input.contains(&one[0])
                && input.contains(&one[1])
        })
        .collect::<Vec<_>>();
    assert_eq!(zero_candidates.len(), 1);
    let mut zero = zero_candidates[0].clone();
    zero.sort();

    // six is the only six-segment digit that is not a nine and not a zero
    let mut six = inputs
        .iter()
        .filter(|input| {
            input.len() == 6
                // Can't compare with `nine`: it's already sorted, `input` is unsorted
                && *input != nine_candidates[0]
                && *input != zero_candidates[0]
        })
        .collect::<Vec<_>>()[0]
        .clone();
    six.sort();

    vec![zero, one, two, three, four, five, six, seven, eight, nine]
}

pub(crate) fn decode_outputs(decoded_inputs: &Vec<Signal>, encoded_outputs: &Vec<Signal>) -> usize {
    let mut display_number = 0;

    let mut smallest_digit_first = encoded_outputs.clone();
    smallest_digit_first.reverse();

    for (pos, signal) in smallest_digit_first.iter().enumerate() {
        let mut sorted_signal = signal.clone();
        sorted_signal.sort();

        let digit = decoded_inputs.iter().position(|s| s == &sorted_signal).unwrap();
        display_number += digit * 10_usize.pow(pos as u32);
    }

    display_number
}

pub(crate) fn count_easy_segments(segment_sets: &Vec<SegmentPatternSet>) -> usize {
    let mut count = 0;

    for segment_set in segment_sets {
        let outputs = &segment_set.1;
        for output in outputs {
            match output.len() {
                2 | 3 | 4 | 7  => count += 1,
                _ => continue,
            }
        }
    }

    count
}

pub(crate) fn sum_outputs(segment_signal_sets: &Vec<SegmentSignalSet>) -> usize {
    let mut sum = 0;

    for set in segment_signal_sets {
        let decoded_inputs = decode_inputs(set);
        let display = decode_display(&decoded_inputs, &set.1);
        sum += display
    }
    sum
}

#[cfg(test)]
#[test]
fn test_parse_segments() {
    let inputs = read_lines("data/day_8_sample.txt");
    let segment_sets = parse_input_output_signals(&inputs);
    assert_eq!(segment_sets.len(), 10);

    let expected = vec!["fdgacbe".to_string(), "cefdb".to_string(), "cefbgd".to_string(), "gcbe".to_string()];
    assert_eq!(segment_sets[0].1, expected)
}

#[test]
fn test_count_easy_segments() {
    let inputs = read_lines("data/day_8_sample.txt");
    let segment_sets = parse_input_output_signals(&inputs);
    let count = count_easy_segments(&segment_sets);
    assert_eq!(count, 26);
}

#[test]
fn test_full_signal_parse() {
    let inputs = read_lines("data/day_8_sample.txt");
    let segment_sets = parse_input_output_signals(&inputs);
    let segment_signal_sets = parse_full_signals(&segment_sets);

    let first_set = segment_signal_sets[0].clone();
    assert_eq!(first_set.1[0], vec![5, 3, 6, 0, 2, 1, 4]);
}

#[test]
fn test_full_decode() {
    let inputs = vec!["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string()];
    let segment_sets = parse_input_output_signals(&inputs);
    let segment_signal_sets = parse_full_signals(&segment_sets);

    let first_set = segment_signal_sets[0].clone();
    let input_signals =  decode_inputs(&first_set);

    //  dddd
    // e    a
    // e    a
    //  ffff
    // g    b
    // g    b
    //  cccc
    let top_right = 0;  // a
    let bottom_right = 1;  // b
    let bottom = 2;  // c
    let top = 3;  // d
    let top_left = 4;  // e
    let middle = 5;  // f
    let bottom_left = 6;  // g

    // one is two segments
    assert_eq!(input_signals[1], vec![top_right, bottom_right, ]);
    // four is four segments
    assert_eq!(input_signals[4], vec![top_right, bottom_right, top_left, middle]);
    // seven is three segments
    assert_eq!(input_signals[7], vec![top_right, bottom_right, top]);
    // eight is seven segments
    assert_eq!(input_signals[8], vec![top_right, bottom_right, bottom, top, top_left, middle, bottom_left]);
    // three is five segments
    assert_eq!(input_signals[3], vec![top_right, bottom_right, bottom, top, middle]);
    // nine is six segments
    assert_eq!(input_signals[9], vec![top_right, bottom_right, bottom, top, top_left, middle]);
    // five is five segments
    assert_eq!(input_signals[5], vec![bottom_right, bottom, top, top_left, middle]);
    // zero is six segments
    assert_eq!(input_signals[0], vec![top_right, bottom_right, bottom, top, top_left, bottom_left]);
    // six is six segments
    assert_eq!(input_signals[6], vec![bottom_right, bottom, top, top_left, middle, bottom_left]);
    // two is five segments
    assert_eq!(input_signals[2], vec![top_right, bottom, top, middle, bottom_left]);
}

#[test]
fn test_output_decode() {
    let inputs = vec!["acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string()];
    let segment_sets = parse_input_output_signals(&inputs);
    let segment_signal_sets = parse_full_signals(&segment_sets);

    let first_set = segment_signal_sets[0].clone();
    let input_signals =  decode_inputs(&first_set);
    let display = decode_display(&input_signals, &first_set.1);
    assert_eq!(display, 5353)
}

#[test]
fn test_output_sum() {
    let inputs = read_lines("data/day_8_sample.txt");
    let segment_sets = parse_input_output_signals(&inputs);
    let segment_signal_sets = parse_full_signals(&segment_sets);

    let sum = sum_outputs(&segment_signal_sets);
    assert_eq!(sum, 61229)
}
