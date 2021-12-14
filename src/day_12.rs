use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Graph, Undirected};
use petgraph::algo::has_path_connecting;
use petgraph::visit::EdgeRef;
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
        let node_a;
        let node_b;

        if let Some(node) = find_node(&caves, &a) {
             node_a = node;
        } else {
            node_a = caves.add_node(a);
        }

        if let Some(node) = find_node(&caves, &b) {
            node_b = node;
        } else {
            node_b = caves.add_node(b);
        }

        caves.add_edge(node_a, node_b, ());
    }

    caves
}

pub(crate) fn all_paths(caves: &Graph<String, (), Undirected>) -> Vec<Vec<NodeIndex>> {
    // Initialize start and end
    let start_id = find_node(&caves, &"start".to_string()).unwrap();
    let end_id = find_node(&caves, &"end".to_string()).unwrap();

    // Begin paths from just the start node
    let mut paths = vec![vec![start_id]];

    loop {
        // Control flag
        let mut modified = false;

        for path_id in 0..paths.len() {
            expand_paths(&caves, &mut paths, end_id, &mut modified, path_id)
        }

        if all_finished(&paths, &end_id) { break; }
        if !modified {
            let paths_str = paths_as_strings(caves, &paths);
            println!("Paths not modified in last iteration, but not complete: {:?}", paths_str);
            break;
        }
    }

    paths
}

// Converts paths as node indices to their weights: strings in this case
fn paths_as_strings(cave: &Graph<String, (), Undirected>, paths: &Vec<Vec<NodeIndex>>) -> Vec<Vec<String>> {
    let paths_strs = paths
        .iter()
        .map(|path| {
            path
                .iter()
                .map(|id| cave.index(*id).clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>();

    paths_strs
}

fn expand_paths(cave: &Graph<String, (), Undirected>, paths: &mut Vec<Vec<NodeIndex>>, end_id: NodeIndex, modified: &mut bool, path_id: usize) {
    let last_node_id = paths[path_id].last().unwrap();

    // If the path is already complete: no need to process further
    let last_idx = last_node_id.index();
    let end_idx = end_id.index();
    if last_idx == end_idx {
        println!("Path {:?} finished", paths[path_id]);
        return;
    }

    let edge_refs_from_last = cave
        .edges(*last_node_id)
        .collect::<Vec<_>>();

    let connected_nodes_from_last = edge_refs_from_last
        .iter()
        .map(|e| e.target())
        .collect::<Vec<_>>();

    // Start by mutating the path in-place
    let mut path_append_mode = false;

    for node_id in connected_nodes_from_last {
        // Skip if there is no path to the exit from here
        if !has_path_connecting(cave, node_id, end_id, None) {
            continue;
        }

        // Skip if the node "name" is lower case and already contained in the path
        let node = &cave[node_id];
        let is_lowercase = &node.to_lowercase() == node;
        if is_lowercase && paths[path_id].contains(&node_id) { continue };

        // Otherwise: add the node id to the path
        if !path_append_mode {
            // Modify in-place to re-use existing path
            paths[path_id].push(node_id);
            path_append_mode = true;

            // Set the control flag
            *modified = true;
        } else {
            // Otherwise: add a new "branch" to the list of paths
            let mut new_path = paths[path_id].clone();
            new_path.push(node_id);
            paths.push(new_path);

            // Set the control flag
            *modified = true;
        }
    }
}

pub(crate) fn all_finished(paths: &Vec<Vec<NodeIndex>>, end_id: &NodeIndex) -> bool {
    for path in paths {
        if path.last().unwrap() != end_id {
            return false
        }
    }

    true
}

#[cfg(test)]
#[test]
fn test_cave_parse() {
    let inputs = read_lines("data/day_12_sample.txt");
    let caves = parse_cave_system(&inputs);
    let start_node = find_node(&caves, &"start".to_string()).unwrap();
    assert_eq!(start_node.index(), 0);
}

#[test]
fn test_lowercase_comp() {
    let uppercase = "BLA".to_string();
    let is_equal = uppercase == uppercase.to_lowercase();
    assert_eq!(is_equal, false);
}

#[test]
fn test_simple_paths() {
    let inputs = read_lines("data/day_12_sample.txt");
    let caves = parse_cave_system(&inputs);
    let paths = all_paths(&caves);
    assert_eq!(paths.len(), 10);
}
