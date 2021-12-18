use petgraph::{Graph, Undirected};
use petgraph::algo::{astar, dijkstra};
use petgraph::graph::NodeIndex;
use advent_of_code_2021::{find_node, read_lines};

pub(crate) fn run() {

}

pub(crate) fn parse_grid(inputs: &Vec<String>) -> Vec<Vec<usize>> {
    inputs.iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub(crate) fn parse_graph(grid: &Vec<Vec<usize>>) -> Graph<(usize, usize), usize, Undirected>{
    let mut graph = Graph::new_undirected();

    // Add start node
    let from_node = graph.add_node((0, 0));

    // Add first-rows edges: it's not covered by edges to above and left in code below
    for col_idx in 1..grid[1].len() {
        add_edge_to_left(&grid, &mut graph, 0, col_idx, from_node)
    }

    // Add first-column edges: it's not covered by edges to above and left in code below
    for row_idx in 1..grid.len() {
        add_edge_to_above(&grid, &mut graph, row_idx, 0, from_node)
    }

    for row_idx in 1..grid.len() {
        for col_idx in 1..grid[1].len() {
            let from_node_idx;

            if let Some(n) = find_node(&graph, &(row_idx, col_idx)) {
                from_node_idx = n;
            } else {
                from_node_idx = graph.add_node((row_idx, col_idx))
            }

            add_edge_to_above(grid, &mut graph, row_idx, col_idx, from_node_idx);
            add_edge_to_left(grid, &mut graph, row_idx, col_idx, from_node_idx);
        }
    }

    graph
}

fn add_edge_to_above(
    grid: &Vec<Vec<usize>>,
    graph: &mut Graph<(usize, usize), usize, Undirected>,
    row_idx: usize,
    col_idx: usize,
    from_node_idx: NodeIndex
) {
// Add edge to entry above
    let row_above_idx = row_idx - 1;
    let risk = grid[row_above_idx][col_idx];
    let node_above_idx;

    if let Some(n) = find_node(&graph, &(row_above_idx, col_idx)) {
        node_above_idx = n;
    } else {
        node_above_idx = graph.add_node((row_above_idx, col_idx))
    }

    graph.add_edge(from_node_idx, node_above_idx, risk);
    graph.add_edge(node_above_idx, from_node_idx, risk);
}

pub(crate) fn add_edge_to_left(grid: &Vec<Vec<usize>>, graph: &mut Graph<(usize, usize), usize, Undirected>, row_idx: usize, col_idx: usize, from_node_idx: NodeIndex) {
// Add edge to entry to the left
    let col_to_left_idx = col_idx - 1;
    let risk = grid[row_idx][col_to_left_idx];
    let node_to_left;

    if let Some(n) = find_node(&graph, &(row_idx, col_to_left_idx)) {
        node_to_left = n;
    } else {
        node_to_left = graph.add_node((row_idx, col_to_left_idx))
    }

    graph.add_edge(from_node_idx, node_to_left, risk);
    // graph.add_edge(node_to_left, from_node_idx, risk);
}

#[cfg(test)]
#[test]
fn test_parse_grid() {
    let inputs = read_lines("data/day_15_sample.txt");
    let grid = parse_grid(&inputs);
    assert_eq!(grid.len(), 10);
    assert_eq!(grid[0].len(), 10);
}

#[test]
fn test_parse_graph() {
    let inputs = read_lines("data/day_15_sample.txt");
    let grid = parse_grid(&inputs);
    let graph = parse_graph(&grid);

    let from = find_node(&graph, &(1, 1)).unwrap();
    let to = find_node(&graph, &(0, 0)).unwrap();
    assert_eq!(graph.contains_edge(from, to), false);

    let from = find_node(&graph, &(0, 1)).unwrap();
    let to = find_node(&graph, &(0, 0)).unwrap();
    assert_eq!(graph.contains_edge(from, to), true);

    let from = find_node(&graph, &(1, 0)).unwrap();
    let to = find_node(&graph, &(0, 0)).unwrap();
    assert_eq!(graph.contains_edge(from, to), true);

    let from = find_node(&graph, &(0, 0)).unwrap();
    let to = find_node(&graph, &(0, 1)).unwrap();
    assert_eq!(graph.contains_edge(from, to), true);

    let from = find_node(&graph, &(0, 0)).unwrap();
    let to = find_node(&graph, &(1, 0)).unwrap();
    assert_eq!(graph.contains_edge(from, to), true);
}

#[test]
fn test_cheapest_path() {
    let inputs = read_lines("data/day_15_sample.txt");
    let grid = parse_grid(&inputs);
    let graph = parse_graph(&grid);

    let start_node = find_node(&graph, &(0, 0)).unwrap();
    let finish_node = find_node(&graph, &(9, 9)).unwrap();
    let cheapest = astar(
        &graph, start_node,
        |n| n == finish_node,
        |e| *e.weight(),
        |_| 0
    ).unwrap();
    println!("cheapest: {:?}", cheapest);
    println!("Route:");
    for node in cheapest.1 {
        print!("{:?} ", graph[node])
    }
    assert_eq!(cheapest.0, 40);
}