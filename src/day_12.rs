use std::collections::hash_map::Entry;
use std::collections::HashMap;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Graph, Undirected};
use petgraph::visit::EdgeRef;
use advent_of_code_2021::{read_lines, find_node};
use advent_of_code_2021::day_12_part_2::{valid_double_visit_paths, parse_simple_caves};

pub(crate) fn run() {
    let inputs = read_lines("data/day_12_input.txt");
    // let caves = parse_cave_system(&inputs);
    // let paths = all_paths(&caves, 1);
    // println!("There are {} valid paths out of the caves", paths.len());

    let (_, edges) = parse_simple_caves(&inputs);
    let paths = valid_double_visit_paths(&edges);
    println!("There are {} valid paths with a small cave visit max twice", paths.len());
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

pub(crate) fn all_paths(caves: &Graph<String, (), Undirected>, max_single_small_cave_visits: usize) -> Vec<Vec<NodeIndex>> {
    // Initialize start and end
    let start_id = find_node(&caves, &"start".to_string()).unwrap();
    let end_id = find_node(&caves, &"end".to_string()).unwrap();

    // Begin paths from just the start node
    let mut paths = vec![vec![start_id]];

    loop {
        // Control flag
        let mut modified = false;

        for path_id in 0..paths.len() {
            expand_paths(&caves, &mut paths, path_id, end_id, &mut modified, max_single_small_cave_visits);
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
fn paths_as_strings(caves: &Graph<String, (), Undirected>, paths: &Vec<Vec<NodeIndex>>) -> Vec<Vec<String>> {
    let paths_strs = paths
        .iter()
        .map(|path| {
            path
                .iter()
                .map(|id| caves[*id].clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<String>>>();

    paths_strs
}

pub(crate) fn expand_paths(
    caves: &Graph<String, (), Undirected>,
    paths: &mut Vec<Vec<NodeIndex>>,
    path_id: usize,
    end_id: NodeIndex,
    modified: &mut bool,
    max_single_small_cave_visits: usize
) {
    let last_node_id = paths[path_id].last().unwrap();

    // If the path is already complete: no need to process further
    let last_idx = last_node_id.index();
    let end_idx = end_id.index();
    if last_idx == end_idx {
        // println!("Path {:?} finished", paths[path_id]);
        return;
    }

    let edge_refs_from_last = caves
        .edges(*last_node_id)
        .collect::<Vec<_>>();

    let connected_nodes_from_last = edge_refs_from_last
        .iter()
        .map(|e| e.target())
        .collect::<Vec<_>>();

    // Start by mutating the path in-place
    let mut path_append_mode = false;

    for cave_id in connected_nodes_from_last {
        // Skip if the node "name" is lower case and already contained in the path
        let node = &caves[cave_id];
        let is_lowercase = &node.to_lowercase() == node;
        let paths_as_strings = paths_as_strings(&caves, &paths);

        // If the cave name is lower case
        // and the path already contains the node id
        // and the path already is at the maximum of small cave visits:
        // continue to the next cave
        if is_lowercase
            && paths[path_id].contains(&cave_id)
            && small_cave_visits_already_at_max(&paths_as_strings[path_id], max_single_small_cave_visits) {
            continue;
        }

        // If we doubled back to the start cave: continue
        if cave_id == *paths[path_id].first().unwrap() {
            continue;
        }

        // Otherwise: add the node id to the path
        path_append_mode = add_cave_to_path(paths, path_id, cave_id, path_append_mode, modified);
    }
}

fn add_cave_to_path(paths: &mut Vec<Vec<NodeIndex>>, path_id: usize, cave_id: NodeIndex, path_append_mode: bool, modified: &mut bool) -> bool {
    let mut new_append_mode = path_append_mode;

    if !path_append_mode {
        // Modify in-place to re-use existing path
        paths[path_id].push(cave_id);
        new_append_mode = true;

        // Set the control flag
        *modified = true;
    } else {
        // Otherwise: add a new "branch" to the list of paths
        let mut new_path = paths[path_id].clone();
        // Get rid of the last node index: it was added in the modify-in-place pass
        new_path = new_path[0..new_path.len() - 1].to_owned();
        new_path.push(cave_id);
        paths.push(new_path);

        // Set the control flag
        *modified = true;
    }

    new_append_mode
}

pub(crate) fn small_cave_visits_already_at_max(path_as_strings: &Vec<String>, max_single_small_cave_visits: usize) -> bool {
    let mut small_cave_counts = HashMap::new();

    for cave_name in path_as_strings {
        let is_lowercase = &cave_name.to_lowercase() == cave_name;

        if is_lowercase {
            let entry = match small_cave_counts.entry(cave_name) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(0_usize),
            };

            *entry += 1;

            if entry.clone() >= max_single_small_cave_visits {
                return true;
            }
        }
    }

    false
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

    let paths = all_paths(&caves, 2);
    assert_eq!(paths.len(), 36);

    let inputs = read_lines("data/day_12_larger_sample.txt");
    let caves = parse_cave_system(&inputs);
    let paths = all_paths(&caves, 1);
    assert_eq!(paths.len(), 19);
}

#[test]
fn test_simple_graph_parse() {
    let inputs = read_lines("data/day_12_sample.txt");
    let (nodes, edges) = parse_simple_caves(&inputs);
    assert_eq!(nodes.len(), 6);
    assert_eq!(edges.len(), inputs.len() * 2);
}

#[test]
fn test_simple_paths() {
    let inputs = read_lines("data/day_12_sample.txt");
    let (_, edges) = parse_simple_caves(&inputs);

    let mut paths = valid_double_visit_paths(&edges);
    paths.sort();

    let mut expected = read_lines("data/day_12_sample_output.txt")
        .iter()
        .map(|line| line.split(","))
        .map(|splits| splits.map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    expected.sort();

    for (idx, path) in paths.iter().enumerate() {
        assert_eq!(path, &expected[idx]);
        // println!("{}", idx + 1);
    }

    assert_eq!(paths.len(), 36);
}

#[test]
fn test_larger_sample() {
    let inputs = read_lines("data/day_12_larger_sample.txt");
    let (_, edges) = parse_simple_caves(&inputs);

    let paths = valid_double_visit_paths(&edges);
    assert_eq!(paths.len(), 103);
}

#[test]
fn test_largest_sample() {
    let inputs = read_lines("data/day_12_even_larger_sample.txt");
    let (_, edges) = parse_simple_caves(&inputs);

    let paths = valid_double_visit_paths(&edges);
    assert_eq!(paths.len(), 3509);
}