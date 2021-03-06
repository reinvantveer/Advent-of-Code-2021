use std::collections::hash_map::Entry;
use std::collections::HashMap;
use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_12_input.txt");
    let (_, cave_edges) = parse_cave_system(&inputs);
    let paths = all_paths(&cave_edges, 1);
    println!("There are {} valid paths out of the caves with one small cave visit", paths.len());

    let (_, edges) = parse_cave_system(&inputs);
    let paths = all_paths(&edges, 2);
    println!("There are {} valid paths with a small cave visit max twice", paths.len());
}

type NodeArray = Vec<String>;
type EdgeArray = Vec<(String, String)>;
type Paths = HashMap<String, Vec<String>>;

pub(crate) fn parse_cave_system(inputs: &Vec<String>) -> (NodeArray, EdgeArray) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for line in inputs {
        let node_a_b = line.split("-").collect::<Vec<_>>();

        let from = node_a_b[0].to_string();
        if !nodes.contains(&from) {
            nodes.push(from.clone());
        }

        let to = node_a_b[1].to_string();
        if !nodes.contains(&to) {
            nodes.push(to.clone());
        }

        // Undirected
        edges.push((from.clone(), to.clone()));
        edges.push((to, from))

    }

    (nodes, edges)
}

pub(crate) fn all_paths(caves: &EdgeArray, max_single_small_cave_visits: usize) -> Paths {
    // Begin paths from just the start node
    let mut paths = HashMap::new();
    let start = "start".to_string();
    paths.insert(start.clone(), vec![start]);
    let mut paths_len = paths.len();

    loop {
        for (path_id, _) in paths.clone() {
            expand_paths(&caves, &mut paths, &path_id, max_single_small_cave_visits);
        }

        if paths.len() == paths_len {
            println!("Paths not modified in last iteration, returning paths");
            let ending_paths = paths
                .iter()
                .filter(|(key, _)| key.ends_with(&"end".to_string()))
                .map(|(key, path)| (key.clone(), path.clone()))
                .collect::<Paths>();
            return ending_paths
        } else {
            paths_len = paths.len();
        }
    }
}

pub(crate) fn expand_paths(
    caves: &EdgeArray,
    paths: &mut Paths,
    path_id: &String,
    max_single_small_cave_visits: usize
) {
    let last_cave = paths[&path_id.clone()].last().unwrap();

    // If the path is already complete: no need to process further
    if last_cave == &"end".to_string() {
        return;
    }

    let connected_nodes_from_last = caves
        .iter()
        .filter(|edge| edge.0 == *last_cave)
        .map(|e| e.1.clone())
        .collect::<Vec<_>>();

    for cave in connected_nodes_from_last {
        // If the cave name is lower case
        // and the path already contains the node id
        // and the path already is at the maximum of small cave visits:
        // continue to the next cave

        if (cave.to_lowercase() == cave)
            && paths[&path_id.clone()].contains(&cave)
            && small_cave_visits_already_at_max(&paths[&path_id.clone()], max_single_small_cave_visits) {
            continue;
        }

        // If we doubled back to the start cave: continue
        if cave == "start" {
            continue;
        }

        // Otherwise: add the node id to the path
        add_cave_to_path(paths, &path_id, cave);
    }
}

fn add_cave_to_path(paths: &mut Paths, path_id: &String, cave: String) {
    // Add a new "branch" to the list of paths
    let mut new_path = paths[path_id].clone();
    // Get rid of the last node index: it was added in the modify-in-place pass
    new_path.push(cave);
    let key = new_path.join("");
    paths.insert(key.clone(), new_path);
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

#[cfg(test)]
#[test]
fn test_lowercase_comp() {
    let uppercase = "BLA".to_string();
    let is_equal = uppercase == uppercase.to_lowercase();
    assert_eq!(is_equal, false);
}

#[test]
fn test_single_loop_iteration_paths_expansion() {
    let inputs = read_lines("data/day_12_sample.txt");
    let (_, edges) = parse_cave_system(&inputs);

    let mut paths = HashMap::new();
    let start = "start".to_string();
    paths.insert(start.clone(), vec![start.clone()]);

    expand_paths(&edges, &mut paths, &start, 1);

    let capital_a = "A".to_string();
    let b = "b".to_string();
    let expected = HashMap::from([
        (start.clone(), vec![start.clone()]),
        (start.clone() + &*capital_a.clone(), vec![start.clone(), capital_a]),
         (start.clone() + &*b.clone(), vec![start, b]),
    ]);
    assert_eq!(paths, expected);
}

#[test]
fn test_simple_graph_parse() {
    let inputs = read_lines("data/day_12_sample.txt");
    let (nodes, edges) = parse_cave_system(&inputs);
    assert_eq!(nodes.len(), 6);
    assert_eq!(edges.len(), inputs.len() * 2);
}

#[test]
fn test_all_valid_paths_small_sample() {
    let inputs = read_lines("data/day_12_sample.txt");
    let (_, cave_edges) = parse_cave_system(&inputs);

    let paths = all_paths(&cave_edges, 1);
    assert_eq!(paths.len(), 10);

    let paths = all_paths(&cave_edges, 2);
    assert_eq!(paths.len(), 36);
}

#[test]
fn test_valid_paths_larger_sample() {
    let inputs = read_lines("data/day_12_larger_sample.txt");
    let (_, cave_edges) = parse_cave_system(&inputs);

    let paths = all_paths(&cave_edges, 1);
    assert_eq!(paths.len(), 19);

    let paths = all_paths(&cave_edges, 2);
    assert_eq!(paths.len(), 103);
}

#[test]
fn test_largest_sample() {
    let inputs = read_lines("data/day_12_even_larger_sample.txt");
    let (_, cave_edges) = parse_cave_system(&inputs);

    let paths = all_paths(&cave_edges, 1);
    assert_eq!(paths.len(), 226);

    let paths = all_paths(&cave_edges, 2);
    assert_eq!(paths.len(), 3509);
}