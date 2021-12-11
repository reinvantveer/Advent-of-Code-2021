use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex};
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
    let dem_graph = graph_from_dem(&dem);

    for minimum in minima {
        basins.push(expand_basin(minimum, &dem_graph).to_owned());
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let three_largest_basins = basins[0..3].to_vec();

    three_largest_basins
}

pub(crate) fn graph_from_dem(dem: &DEM) -> DiGraph<DEMPoint, ()> {
    let mut dem_graph= DiGraph::new();

    for (row_idx, row) in dem.iter().enumerate() {
        for (col_idx, entry) in row.iter().enumerate() {
            let node_at_point = dem_graph.add_node(DEMPoint {
                row: 0,
                column: 0,
                risk: dem[0][0] + 1
            });

            let row_idx_below = row_idx + 1;

            // We only have to compare below and to the right in order to construct the graph
            if let Some(row_below) = dem.get(row_idx_below) {
                let node_below_idx = dem_graph.add_node(DEMPoint{
                    row: row_idx_below,
                    column: col_idx,
                    risk: dem[row_idx_below][col_idx]
                });

                if row_below[col_idx] > *entry {
                    dem_graph.add_edge(node_at_point, node_below_idx, ());
                } else if *entry > row_below[col_idx] {
                    dem_graph.add_edge(node_below_idx, node_at_point, ());
                }
            }

            if let Some(col_to_right) = row.get(col_idx + 1) {
                let node_to_right = dem_graph.add_node(DEMPoint{
                    row: row_idx,
                    column: *col_to_right,
                    risk: dem[row_idx][*col_to_right]
                });

                if row[*col_to_right] > *entry {
                    dem_graph.add_edge(node_at_point, node_to_right, ());
                } else if *entry > row[*col_to_right] {
                    dem_graph.add_edge(node_to_right, node_at_point, ());
                }
            }
        }
    }

    dem_graph
}

pub(crate) fn expand_basin(minimum: &DEMPoint, dem_graph: &DiGraph<DEMPoint, ()>) -> Basin {
    let basin = Vec::new();
    let minimum_node_idx = dem_graph
        .node_indices()
        .find(|idx| dem_graph[*idx] == *minimum)
        .unwrap();

    let connected_node_ids = dijkstra(dem_graph, minimum_node_idx, None, |_| 1)
        .iter()
        .map(|(node_idx, distance)| node_idx.clone() )
        .collect::<Vec<NodeIndex>>();

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

    assert_eq!(dem_graph.node_indices().len(), 5)
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