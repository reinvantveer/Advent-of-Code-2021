pub mod day_12_part_2;

use std::fs::File;
use std::io;
use std::io::BufRead;
use petgraph::{EdgeType, Graph};
use petgraph::graph::{IndexType, NodeIndex};

pub fn read_lines(file: &str) -> Vec<String> {
    let input = File::open(file).unwrap();
    let reader = io::BufReader::new(input);
    let inputs: Vec<String> = reader.lines()
        .filter_map(io::Result::ok)
        .collect();
    inputs
}

pub fn parse_vec_usize(inputs: &Vec<String>) -> Vec<usize> {
    let vec = inputs[0]
        .split(",")
        .map(|f| f.parse::<usize>().unwrap())
        .collect();

    vec
}

pub fn find_node<N, E, D, I>(
    graph: &Graph<N, E, D, I>,
    needle: &N
) -> Option<NodeIndex<I>>
    where N: PartialEq, D: EdgeType, I: IndexType
{
    graph
        .node_indices()
        .find(|idx| graph[*idx] == *needle)
}

#[cfg(test)]
#[test]
fn test_vec_usize_from_input() {
    let inputs = read_lines("data/day_6_sample.txt");
    let school = parse_vec_usize(&inputs);
    assert_eq!(school, vec![3, 4, 3, 1, 2]);
}

#[test]
fn test_find_node() {
    let inputs = read_lines("data/day_9_sample.txt");
    let dem = parse_dem(&inputs);
    let dem_graph = graph_from_dem(&dem);

    let global_minimum = DEMPoint{
        row: 2,
        column: 2,
        risk: 6
    };

    let found_node = find_node(&dem_graph, global_minimum).unwrap();
    assert_eq!(found_node.index(), 0)

}
