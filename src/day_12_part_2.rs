type NodeArray = Vec<String>;
type EdgeArray = Vec<(String, String)>;
type Paths = Vec<Vec<String>>;

pub fn parse_simple_caves(inputs: &Vec<String>) -> (NodeArray, EdgeArray) {
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

pub fn valid_double_visit_paths(edges: &EdgeArray) -> Paths {
    let mut paths: Paths = vec![vec!["start".to_string()]];
    let mut iteration = 0;

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

            let connected_caves = edges
                .iter()
                .filter(|edge| edge.0 == *last_node)
                .map(|edge| edge.1.clone())
                .filter(|node| node != &"start".to_string())
                .collect::<NodeArray>();

            for next_cave in connected_caves {
                let is_small_cave =
                    next_cave.to_lowercase() == next_cave
                        && next_cave != "end".to_string();

                if is_small_cave
                    && is_double_visited(&paths[path_idx])
                    && paths[path_idx].contains(&next_cave) {
                    continue;
                }

                let mut new_path = paths[path_idx].clone();
                new_path.push(next_cave);
                paths.push(new_path);
                mutated = true;
            }

            if mutated {
                paths.remove(path_idx);
                break;
            }
        }

        if !mutated {
            println!("No more mutations, returning");
            break;
        }

        // Safety brake
        if paths.len() > 10000 {
            break;
        }
    }

    paths
        .iter()
        .filter(|path| path.last().unwrap() == "end")
        .map(|path| path.clone())
        .collect::<Vec<_>>()
}

pub(crate) fn is_double_visited(path: &Vec<String>) -> bool {
    let small_caves = path
        .iter()
        .filter(|cave| {
            &&cave.to_lowercase() == cave
                && cave != &&"start".to_string()
                && cave != &&"end".to_string()
        });

    for cave in small_caves {
        let occurrences = path
            .iter()
            .filter(|c| c == &cave)
            .count();

        if occurrences > 1 { return true }
    }

    false
}