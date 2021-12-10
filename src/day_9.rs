use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_9_input.txt");
    let dem = parse_dem(&inputs);
    let risk_levels = collect_local_minima(&dem);
    let risk_sum = risk_levels
        .iter()
        .map(|m| m.risk )
        .sum::<usize>();
    println!("Risk levels total: {}", risk_sum);
}

// A digital elevation model (DEM)
type DEM = Vec<Vec<usize>>;

pub(crate) fn parse_dem(inputs: &Vec<String>) -> DEM {
    let mut dem = Vec::new();

    for input in inputs {
        let row = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        dem.push(row);
    }

    dem
}

fn collect_local_minima(dem: &DEM) -> Vec<usize> {
    let mut local_minima = Vec::new();

    for (row_idx, row) in dem.iter().enumerate() {
        for (col_idx , entry) in row.iter().enumerate() {

            // Continue if larger than the one above
            if row_idx > 0 {
                if entry >= &dem[row_idx - 1][col_idx] { continue }
            }

            // Continue if entry is larger than the one below
            if let Some(next_row) = dem.get(row_idx + 1) {
                if entry >= &next_row[col_idx] { continue }
            }

            // Continue if entry is larger than the one on the left
            if col_idx > 0 {
                if entry >= &dem[row_idx][col_idx - 1] { continue }
            }

            // Continue if entry is larger than the one on the right
            if let Some(entry_to_right) = dem[row_idx].get(col_idx + 1) {
                if entry >= &entry_to_right { continue }
            }

            local_minima.push(*entry + 1);
        }
    }

    local_minima
}

#[cfg(test)]
#[test]
fn test_parse_dem() {
    let inputs = read_lines("data/day_9_sample.txt");
    let dem = parse_dem(&inputs);
    assert_eq!(dem.len(), 5);

    for row in &dem {
        assert_eq!(row.len(), 10)
    }

    assert_eq!(dem[4][9], 8)
}

#[test]
fn test_local_minima() {
    let inputs = read_lines("data/day_9_sample.txt");
    let dem = parse_dem(&inputs);
    let risk_levels = collect_local_minima(&dem);
    assert_eq!(risk_levels, vec![2, 1, 6, 6]);
    let risk_level = risk_levels.iter().sum::<usize>();
    assert_eq!(risk_level, 15);
}