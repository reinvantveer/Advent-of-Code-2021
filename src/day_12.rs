use std::collections::hash_map::Entry;
use std::collections::HashMap;
use advent_of_code_2021::read_lines;

pub(crate) fn run() {
    let inputs = read_lines("data/day_12_input.txt");
    // let caves = parse_cave_system(&inputs);
    // let paths = all_paths(&caves, 1);
    // println!("There are {} valid paths out of the caves", paths.len());

    let (_, edges) = parse_cave_system(&inputs);
    let paths = valid_double_visit_paths(&edges);
    println!("There are {} valid paths with a small cave visit max twice", paths.len());
}

type NodeArray = Vec<String>;
type EdgeArray = Vec<(String, String)>;
type Paths = Vec<Vec<String>>;

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
    let mut paths = vec![vec!["start".to_string()]];
    let mut paths_len = paths.len();

    loop {
        for path_id in 0..paths.len() {
            expand_paths(&caves, &mut paths, path_id, max_single_small_cave_visits);
        }

        if paths.len() == paths_len {
            println!("Paths not modified in last iteration, but not complete: {:?}", paths);
            let ending_paths = paths
                .iter()
                .filter(|path| path.last().unwrap() == &"end".to_string())
                .map(|path| path.clone())
                .collect();
            return ending_paths
        } else {
            paths_len = paths.len();
        }
    }
}

pub(crate) fn expand_paths(
    caves: &EdgeArray,
    paths: &mut Paths,
    path_id: usize,
    max_single_small_cave_visits: usize
) {
    let last_cave = paths[path_id].last().unwrap();

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
            && paths[path_id].contains(&cave)
            && small_cave_visits_already_at_max(&paths[path_id], max_single_small_cave_visits) {
            continue;
        }

        // If we doubled back to the start cave: continue
        if cave == "start" {
            continue;
        }

        // Otherwise: add the node id to the path
        add_cave_to_path(paths, path_id, cave);
    }
}

fn add_cave_to_path(paths: &mut Paths, path_id: usize, cave: String) {
    // Add a new "branch" to the list of paths
    let mut new_path = paths[path_id].clone();
    // Get rid of the last node index: it was added in the modify-in-place pass
    new_path.push(cave);

    if !paths.contains(&new_path) {
        paths.push(new_path);
    }
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

pub fn valid_double_visit_paths(cave_edges: &EdgeArray) -> Paths {
    let mut paths: Paths = vec![vec!["start".to_string()]];
    let mut iteration = 0;
    let small_caves = small_caves(&cave_edges);

    loop {
        iteration += 1;
        if iteration % 10 == 0 {
            println!("Running iteration {}", iteration);
        }

        let mut mutated = false;

        for path_idx in 0..paths.len() {
            // No need to do anything if the path is already at the finish
            if paths[path_idx].last().unwrap() == &"end".to_string() {
                // println!("Path finished: {:?}", paths[path_idx]);
                continue;
            }

            // It's is guaranteed there always to be a last node: the start node
            let last_node = paths[path_idx].last().unwrap();

            let connected_caves = cave_edges
                .iter()
                .filter(|edge| edge.0 == *last_node)
                .map(|edge| edge.1.clone())
                .filter(|node| node != &"start".to_string())
                .collect::<NodeArray>();

            for next_cave in connected_caves {
                let next_cave_is_small = small_caves.contains(&next_cave);

                if next_cave_is_small
                    && is_double_visited(&paths[path_idx], &small_caves)
                    && paths[path_idx].contains(&next_cave) {
                    continue;
                }

                let mut new_path = paths[path_idx].clone();
                new_path.push(next_cave);
                paths.push(new_path);
                mutated = true;
            }
        }

        if !mutated {
            println!("No more mutations, returning");
            break;
        }

        // Safety brake
        if paths.len() > 10000 {
            println!("Safety brake!");
            break;
        }
    }

    paths
        .iter()
        .filter(|path| path.last().unwrap() == "end")
        .map(|path| path.clone())
        .collect::<Vec<_>>()
}

pub(crate) fn small_caves(cave_edges: &EdgeArray) -> Vec<String> {
    let mut nodes = Vec::new();

    for edge in cave_edges {
        if !nodes.contains(&edge.0) {
            nodes.push(edge.0.clone())
        }

        if !nodes.contains(&edge.1) {
            nodes.push(edge.1.clone())
        }
    }

    let small_caves = nodes
        .iter()
        .filter(|cave| {
            &&cave.to_lowercase() == cave
                && cave != &&"start".to_string()
                && cave != &&"end".to_string()
        })
        .map(|n| n.clone())
        .collect();

    small_caves
}

pub(crate) fn is_double_visited(path: &Vec<String>, small_caves: &Vec<String>) -> bool {
    for cave in small_caves {
        let occurrences = path
            .iter()
            .filter(|c| *c == cave)
            .count();

        if occurrences > 1 { return true; }
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

    let mut paths = vec![vec!["start".to_string()]];
    let first_path_idx= 0;

    expand_paths(&edges, &mut paths, first_path_idx, 1);

    let expected = vec![
        vec!["start".to_string()],
        vec!["start".to_string(), "A".to_string()],
        vec!["start".to_string(), "b".to_string()],
    ];
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
fn test_is_double_visited() {
    let inputs = read_lines("data/day_12_sample.txt");
    let (_, edges) = parse_cave_system(&inputs);

    let path = vec!["start", "A", "b", "A", "c", "A", "end"]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>();

    let cave = "b".to_string();

    let occurrences = path
        .iter()
        .filter(|c| *c == &cave)
        .count();
    assert_eq!(occurrences, 1);

    let path = vec!["start", "A", "b", "A", "c", "A", "end"]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>();
    let small = small_caves(&edges);
    assert_eq!(small, vec!["b", "c", "d"]);

    assert_eq!(is_double_visited(&path, &small), false);

    let path2 = vec!["start", "A", "b", "A", "b", "A", "c", "A", "end"]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>();
    assert_eq!(is_double_visited(&path2, &small), true)
}

#[test]
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