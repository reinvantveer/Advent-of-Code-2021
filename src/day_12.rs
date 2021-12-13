use petgraph::graph::UnGraph;
use petgraph::{Graph, Undirected};
use advent_of_code_2021::read_lines;
use advent_of_code_2021::find_node;

pub(crate) fn run() {

}

pub(crate) fn parse_cave_system(inputs: &Vec<String>) -> Graph<String, (), Undirected>{
    let mut caves = UnGraph::new_undirected();

    let nodes_edges: Vec<_> = inputs
        .iter()
        .map(|line| {
            let parts = line.split("-").collect::<Vec<_>>();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();

    for (a, b) in nodes_edges {
        let node_a = caves.add_node(a);
        let node_b = caves.add_node(b);
        caves.add_edge(node_a, node_b, ());
    }

    caves
}

#[cfg(test)]
#[test]
fn test_parse() {
    let inputs = read_lines("data/day_12_sample.txt");
    let caves = parse_cave_system(&inputs);
    let start_node = find_node(&caves, &"start".to_string()).unwrap();
    assert_eq!(start_node.index(), 0);
}
