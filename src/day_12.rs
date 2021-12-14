use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Graph, Undirected};
use petgraph::visit::EdgeRef;
use advent_of_code_2021::read_lines;
use advent_of_code_2021::find_node;

pub(crate) fn run() {
    let inputs = read_lines("data/day_12_input.txt");
    let caves = parse_cave_system(&inputs);
    let paths = all_paths(&caves, 1);
    println!("There are {} valid paths out of the caves", paths.len());
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

pub(crate) fn all_paths(caves: &Graph<String, (), Undirected>, max_small_cave_visits: usize) -> Vec<Vec<NodeIndex>> {
    // Initialize start and end
    let start_id = find_node(&caves, &"start".to_string()).unwrap();
    let end_id = find_node(&caves, &"end".to_string()).unwrap();

    // Begin paths from just the start node
    let mut paths = vec![vec![start_id]];

    loop {
        // Control flag
        let mut modified = false;

        for path_id in 0..paths.len() {
            expand_paths(&caves, &mut paths, path_id, end_id, &mut modified, max_small_cave_visits);
        }

        if all_finished(&paths, &end_id) { break; }
        if !modified {
            let paths_str = paths_as_strings(caves, &paths);
            println!("Paths not modified in last iteration, but not complete: {:?}", paths_str);
            return paths
                .iter()
                .filter(|path| path.last().unwrap() == &end_id)
                .map(|path| path.clone() )
                .collect()
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
                .map(|id| cave[*id].clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>();

    paths_strs
}

pub(crate) fn expand_paths(
    cave: &Graph<String, (), Undirected>,
    paths: &mut Vec<Vec<NodeIndex>>,
    path_id: usize,
    end_id: NodeIndex,
    modified: &mut bool,
    max_small_cave_visits: usize
) {
    let last_node_id = paths[path_id].last().unwrap();

    // If the path is already complete: no need to process further
    let last_idx = last_node_id.index();
    let end_idx = end_id.index();
    if last_idx == end_idx {
        // println!("Path {:?} finished", paths[path_id]);
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
        // Skip if the node "name" is lower case and already contained in the path
        let node = &cave[node_id];
        let is_lowercase = &node.to_lowercase() == node;
        let visits = paths[path_id]
            .iter()
            .filter(|i| *i == &node_id)
            .count();
        if is_lowercase && visits >= max_small_cave_visits { continue };

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
            // Get rid of the last node index: it was added in the modify-in-place pass
            new_path = new_path[0..new_path.len() - 1].to_owned();
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
fn test_single_loop_iteration_paths_expansion() {
    let inputs = read_lines("data/day_12_sample.txt");
    let caves = parse_cave_system(&inputs);

    let start_node = find_node(&caves, &"start".to_string()).unwrap();
    let end_node = find_node(&caves, &"end".to_string()).unwrap();
    let mut paths = vec![vec![start_node]];
    let mut modified = false;
    let first_path_idx= 0;

    expand_paths(&caves, &mut paths, first_path_idx, end_node, &mut modified, 1);

    let expected = vec![
        vec!["start", "b"],
        vec!["start", "A"],
    ];
    assert_eq!(paths_as_strings(&caves, &paths), expected);
}

#[test]
fn test_all_valid_paths() {
    let inputs = read_lines("data/day_12_sample.txt");
    let caves = parse_cave_system(&inputs);
    let paths = all_paths(&caves, 1);
    assert_eq!(paths.len(), 10);

    let inputs = read_lines("data/day_12_larger_sample.txt");
    let caves = parse_cave_system(&inputs);
    let paths = all_paths(&caves, 1);
    assert_eq!(paths.len(), 19);
}
