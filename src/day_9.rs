use petgraph::{Directed, Graph};
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

#[derive(Clone, PartialEq)]
pub(crate) struct DEMPoint {
    row: usize,
    column: usize,
    // Risk level is the height + 1
    risk: usize,
}

fn collect_local_minima(dem: &DEM) -> Vec<DEMPoint> {
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

            local_minima.push(DEMPoint {
                row: row_idx,
                column: col_idx,
                risk: *entry + 1
            });
        }
    }

    local_minima
}

type Basin = Vec<DEMPoint>;

pub(crate) fn find_three_largest_basins(minima: &Vec<DEMPoint>, dem: &DEM) -> Vec<Basin> {
    let mut basins = Vec::new();

    for minimum in minima {
        let basin = &mut vec![minimum.clone()];
        basins.push(expand_basin(minimum, ).to_owned());
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let three_largest_basins = basins[0..3].to_vec();
    three_largest_basins
}

pub(crate) fn graph_from_dem(dem: &DEM) -> Graph<DEMPoint, Directed> {
    let mut dem_graph= Graph::new();
    dem_graph.add_node(DEMPoint {row: 0, column: 0, risk: dem[0][0] + 1});

    for (row_idx, row) in dem.iter().enumerate() {
        for (col_idx, entry) in row.iter().enumerate() {
            // We only have to compare below and to the right in order to construct the graph
            if let Some(row_below) = dem.get(row_idx + 1) {
                if row_below[col_idx]
            }

            if let Some(col_to_right) = row.get(col_idx + 1) {

            }
        }
    }

    dem_graph
}

pub(crate) fn expand_basin(minimum: &DEMPoint, dem_graph: &Graph<N, E>) -> Basin {
    let mut extra_minima = Vec::new();
    let den_width = dem[0].len();

    for minimum in &*basin {
        // Check all directions if there is a point that we can add

        // Above
        if minimum.row > 0 {
            let row_above = minimum.row - 1;
            let entry_above = dem[row_above][minimum.column];
            let new_minimum = DEMPoint {row: row_above, column: minimum.column, risk: entry_above + 1 };

            // Check only if the new minimum isn't included already in the basin or what has been
            // collected already into the extra minima
            if !basin.contains(&new_minimum) && !extra_minima.contains(&new_minimum) {
                let mut is_lower_than_surrounding = true;
                // Check entry above for existence. Is it lower or equal? Then it is not to be included in the basin.
                if row_above > 0 {
                    if dem[row_above - 1][minimum.column] <= entry_above { is_lower_than_surrounding = false }
                }
                // Check entry to the left. Is it lower or equal? Then it is not to be included in the basin.
                if minimum.column > 0 {
                    if dem[row_above][minimum.column - 1] <= entry_above { is_lower_than_surrounding = false }
                }
                // Check entry to the left. Is it lower or equal? Then it is not to be included in the basin.
                if minimum.column < den_width - 1 {
                    if dem[row_above][minimum.column + 1] <= entry_above { is_lower_than_surrounding = false }
                }
            }
        }
    }

    basin.extend(extra_minima);
    basin
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
    let minima = collect_local_minima(&dem);

    assert_eq!(minima.len(), 4);
    let risk_level = minima.iter()
        .map(|m| m.risk )
        .sum::<usize>();
    assert_eq!(risk_level, 15);
}

#[test]
fn test_graph_from_den() {
    let inputs = read_lines("data/day_9_sample.txt");
    let dem = parse_dem(&inputs);
    let dem_graph = graph_from_dem(&dem);

    assert_eq!()
}

#[test]
fn test_find_three_largest_basins() {
    let inputs = read_lines("data/day_9_sample.txt");
    let dem = parse_dem(&inputs);
    let minima = collect_local_minima(&dem);
    let largest_basins = find_three_largest_basins(&minima, &dem);

    assert_eq!(largest_basins.len(), 3);

    let first_basin = &largest_basins[0];
    assert_eq!(first_basin.len(), 14)
}