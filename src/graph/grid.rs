use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

use crate::graph::Graph;
use bimap::BiMap;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::graph::positioned_graph::Position;
use crate::graph::positioned_graph::PositionedGraph;

pub fn generate_radom_tree_on_grid(n: usize) -> PositionedGraph {
    let directions = vec![
        Position(1, 0),
        Position(0, 1),
        Position(-1, 0),
        Position(0, -1),
    ];
    let mut g = Graph::new(vec![Vec::new(); n]);
    let mut rng = rand::thread_rng();

    let mut node = 0 as usize;
    let mut position = Position(0, 0);
    let mut map = BiMap::new();
    map.insert(node, position);

    while map.len() < n {
        position = position + *directions.choose(&mut rng).unwrap();

        if !map.contains_right(&position) {
            g.add_edge(node, map.len());
            map.insert(map.len(), position);
        }

        node = map.get_by_right(&position).unwrap().clone();
    }

    return PositionedGraph::new(g, map);
}

pub fn generate_grid(side_length: usize) -> Graph {
    let mut g = Graph::with_node_count(side_length * side_length);

    for i in 0..side_length {
        for j in 0..side_length {
            let node = i * side_length + j;
            if i > 0 {
                g.add_edge(node, node - side_length);
            }
            if j > 0 {
                g.add_edge(node, node - 1);
            }
        }
    }

    g
}

pub fn generate_grid_with_avg_degree(side_length: usize, avg_degree: f64) -> Graph {
    let mut g = generate_grid(side_length);
    let num_edges = g.get_num_edges();
    let mut rng = rand::thread_rng();

    let goal_num_edges = (avg_degree * g.get_num_nodes() as f64 / 2.0) as usize;

    for i in goal_num_edges..num_edges {
        g.remove_random_edge();
    }

    g.get_largest_subgraph()
}

pub fn save_separator_distribution(
    step_size: usize,
    max_size: usize,
    num_samples: usize,
    output_file: &str,
) {
    for n in (step_size..max_size).step_by(step_size) {
        for _ in 0..num_samples {
            let g = generate_grid_with_avg_degree((n as f64).sqrt() as usize, 2.5);
            let s = g.get_separator_size(crate::separator::Mode::Strong);

            println!("{} {}", g.get_num_nodes(), s);
            fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(output_file)
                .unwrap()
                .write_all(format!("{} {}\n", g.get_num_nodes(), s).as_bytes());
        }
    }
}

pub fn save_separator_distribution_multithreaded(
    step_size: usize,
    max_size: usize,
    num_samples: usize,
    output_file: &str,
) {
    let num_cores = num_cpus::get();
    let pool = ThreadPool::new(num_cores);
    let (tx, rx) = channel();

    // Use Arc<Mutex<>> to safely share the output file across threads
    let output_file = Arc::new(Mutex::new(output_file.to_string()));

    for n in (step_size..=max_size).step_by(step_size) {
        for _ in 0..num_samples {
            let tx = tx.clone();
            let output_file = Arc::clone(&output_file);

            pool.execute(move || {
                let g = generate_grid_with_avg_degree((n as f64).sqrt() as usize, 2.5);
                let s = g.get_separator_size(crate::separator::Mode::Strong);

                tx.send((g.get_num_nodes(), s))
                    .expect("Failed to send data");

                let mut file = output_file.lock().expect("Failed to lock file");
                let mut file_handle = fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&*file)
                    .expect("Failed to open file");

                file_handle
                    .write_all(format!("{} {}\n", g.get_num_nodes(), s).as_bytes())
                    .expect("Failed to write to file");
            });
        }
    }

    drop(tx);

    for (num_nodes, separator_size) in rx.iter() {
        println!("{} {}", num_nodes, separator_size);
    }

    pool.join();
}

#[cfg(test)]
mod tests {
    use crate::separator::Mode::*;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_generate_random_tree_on_grid() {
        return;
        let g = generate_radom_tree_on_grid(10);
        // print coordinate map
        for i in 0..g.graph.get_num_nodes() {
            println!("{:?} -> {:?}", i, g.positions.get_by_left(&i).unwrap());
        }
        g.graph.print();
    }

    #[test]
    fn test_generate_grid() {
        return;
        let g = generate_grid(4);
        g.print();
    }

    #[test]
    fn test_generate_grid_with_avg_degree() {
        let g = generate_grid_with_avg_degree(5, 2.0);
        g.print();
    }
}
