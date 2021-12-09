use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_8_sample.txt");
    let segment_sets = parse_input_output_signals(&inputs);
    let easy_count = count_easy_segments(&segment_sets);
    println!("The total of easy digits is {}", easy_count);
}

type SegmentSet = (Vec<String>, Vec<String>);

pub(crate) fn parse_input_output_signals(inputs_outputs: &Vec<String>) -> Vec<SegmentSet> {
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

pub(crate) fn count_easy_segments(segment_sets: &Vec<SegmentSet>) -> usize {
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