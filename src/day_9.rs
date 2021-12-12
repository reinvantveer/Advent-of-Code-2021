use std::ops::Index;
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
        basins.push(expand_basin(minimum, &dem_graph, &dem).to_owned());
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let three_largest_basins = basins[0..3].to_vec();

    three_largest_basins
}

pub(crate) fn graph_from_dem(dem: &DEM) -> DiGraph<DEMPoint, ()> {
    let mut dem_graph= DiGraph::new();
    let starting_point = DEMPoint {
        row: 0,
        column: 0,
        risk: dem[0][0] + 1
    };
    dem_graph.add_node(starting_point);

    for (row_idx, row) in dem.iter().enumerate() {
        for (col_idx, entry) in row.iter().enumerate() {
            let dem_point_at_point = DEMPoint { row: row_idx, column: col_idx, risk: dem[row_idx][col_idx] + 1};

            let node_at_point = find_node(&dem_graph, &dem_point_at_point)
                .expect(&format!(
                    "Didn't find node at row {}, column {}, risk {}",
                    row_idx, col_idx, dem[row_idx][col_idx] + 1
                ));

            let row_idx_below = row_idx + 1;

            // We only have to compare below and to the right in order to construct the graph
            if let Some(row_below) = dem.get(row_idx_below) {
                let point_below = DEMPoint {
                    row: row_idx_below,
                    column: col_idx,
                    risk: dem[row_idx_below][col_idx] + 1
                };

                let node_below_idx;
                if let Some(nb) = find_node(&dem_graph, &point_below) {
                    node_below_idx = nb;
                } else {
                    node_below_idx = dem_graph.add_node(point_below);
                }

                if row_below[col_idx] > *entry {
                    dem_graph.add_edge(node_at_point, node_below_idx, ());
                } else if *entry > row_below[col_idx] {
                    dem_graph.add_edge(node_below_idx, node_at_point, ());
                }
            }

            let col_to_right = col_idx + 1;
            if let Some(_) = row.get(col_to_right) {
                let point_to_right = DEMPoint {
                    row: row_idx,
                    column: col_to_right,
                    risk: dem[row_idx][col_to_right] + 1
                };

                let node_to_right;

                if let Some(ntr) = find_node(&dem_graph, &point_to_right){
                    node_to_right = ntr;
                } else {
                    node_to_right = dem_graph.add_node(point_to_right);
                }

                if row[col_to_right] > *entry {
                    dem_graph.add_edge(node_at_point, node_to_right, ());
                } else if *entry > row[col_to_right] {
                    dem_graph.add_edge(node_to_right, node_at_point, ());
                }
            }
        }
    }

    dem_graph
}

pub(crate) fn find_node<T>(
    dem_graph: &DiGraph<T, ()>,
    needle: &T
) -> Option<NodeIndex>
where T: PartialEq
{
    dem_graph
        .node_indices()
        .find(|idx| dem_graph[*idx] == *needle)
}

pub(crate) fn expand_basin(minimum: &DEMPoint, dem_graph: &DiGraph<DEMPoint, ()>, dem: &DEM) -> Basin {
    let minimum_node_idx = find_node(&dem_graph, minimum).unwrap();

    let connected_nodes = dijkstra(dem_graph, minimum_node_idx, None, |_| 1)
        .iter()
        .map(|(node_idx, _)| dem_graph.index(node_idx.clone()).clone() )
        .collect::<Vec<DEMPoint>>();

    let basin = connected_nodes
        .iter()
        .filter(|dem_point| is_surrounded(dem_point, &connected_nodes, dem) )
        .map(|dem_point| dem_point.clone() )
        .collect();

    basin
}

pub(crate) fn is_surrounded(dem_point: &DEMPoint, connected_nodes: &Vec<DEMPoint>, dem: &DEM) -> bool {
    let row_idx = dem_point.row;
    let col_idx = dem_point.column;

    // Check for 9: it isn't part of any basin
    if dem_point.risk == 10 { return  false };

    // Check above if present
    if row_idx > 0 {
        let row_idx_above = row_idx - 1;
        let point_above = DEMPoint {
            row: row_idx_above,
            column: col_idx,
            risk: dem[row_idx_above][col_idx] + 1
        };

        if !connected_nodes.contains(&point_above) { return false; }
    }

    // Check below if present
    let row_idx_below = row_idx + 1;
    if let Some(_) = dem.get(row_idx_below) {
        let point_below = DEMPoint {
            row: row_idx_below,
            column: col_idx,
            risk: dem[row_idx_below][col_idx] + 1
        };

        if !connected_nodes.contains(&point_below) { return false; }
    }

    // Check to left if present
    if col_idx > 0 {
        let col_idx_to_left = col_idx - 1;

        let point_to_left = DEMPoint {
            row: row_idx,
            column: col_idx_to_left,
            risk: dem[row_idx][col_idx_to_left] + 1
        };

        if !connected_nodes.contains(&point_to_left) { return false;}
    }

    // Check to right if present
    let col_idx_to_right = col_idx + 1;
    if let Some(_) = dem[row_idx].get(col_idx_to_right) {
        let point_to_right = DEMPoint {
            row: row_idx,
            column: col_idx_to_right,
            risk: dem[row_idx][col_idx_to_right] + 1
        };

        if !connected_nodes.contains(&point_to_right) { return false; }
    }

    true
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

    let needle = &DEMPoint { row: 0, column: 0, risk: 3 };
    let found = find_node(&dem_graph, needle).unwrap();
    assert_eq!(found.index(), 0);

    let dem_rows = dem.len();
    let dem_cols = dem[0].len();
    assert_eq!(dem_graph.node_indices().len(), dem_rows * dem_cols)
}

#[test]
fn test_is_surrounded() {
    let inputs = read_lines("data/day_9_sample.txt");
    let dem = parse_dem(&inputs);
    let dem_graph = graph_from_dem(&dem);

    let global_minimum = DEMPoint{
        row: 2,
        column: 2,
        risk: 6
    };
    let glob_min_idx = find_node(&dem_graph, &global_minimum).unwrap();
    let connected_to_global_minimum = dijkstra(&dem_graph, glob_min_idx, None, |_| 1)
        .iter()
        .map(|(node_idx, _)| dem_graph.index(node_idx.clone()).clone() )
        .collect::<Vec<DEMPoint>>();

    let test_point = DEMPoint {
        row: 4,
        column: 0,
        risk: 10,
    };
    let is_point_surrounded = is_surrounded(&test_point, &connected_to_global_minimum, &dem);
    assert_eq!(is_point_surrounded, false);
}

#[test]
fn test_find_three_largest_basins() {
    let inputs = read_lines("data/day_9_sample.txt");
    let dem = parse_dem(&inputs);
    let minima = collect_local_minima(&dem);
    let largest_basins = find_three_largest_basins(&minima, &dem);

    assert_eq!(largest_basins.len(), 3);

    let first_basin = largest_basins[0].clone();
    assert_eq!(first_basin.len(), 14)
}