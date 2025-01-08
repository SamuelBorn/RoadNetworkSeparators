use rand::Rng;
use std::collections::HashSet;

use crate::graph::Graph;

pub fn generate_random_tree(n: usize) -> Graph {
    let mut data = vec![Vec::new(); n];
    let mut visited = HashSet::new();
    let mut n1 = 0;
    visited.insert(n1);

    while visited.len() < n {
        let n2 = rand::thread_rng().gen_range(0..n);
        if !visited.contains(&n2) {
            visited.insert(n2);

            data[n1].push(n2);
            data[n2].push(n1);
        }
        n1 = n2;
    }

    Graph::new(data)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{self, Write};

    use super::*;

    #[test]
    fn test_generate_random_tree() {
        let graph = generate_random_tree(100);
        assert_eq!(graph.get_num_nodes(), 100);
        assert_eq!(graph.get_num_edges(), 99 * 2);

        for i in 0..graph.get_num_nodes() {
            assert!(graph.get_neighbors(i).len() >= 1);
        }
    }

    #[test]
    fn diameter_overview() {
        return;
        let start_n = 10000;
        let end_n = 100000;
        let step = 10000;
        let iterations = 5;

        let mut result = String::new();
        for n in (start_n..=end_n).step_by(step) {
            for _ in 0..iterations {
                let g = generate_random_tree(n);
                result.push_str(&format!("{} {}\n", n, g.get_diameter()));
            }
        }

        File::create("output/diameter_overview.txt")
            .unwrap()
            .write_all(result.as_bytes())
            .unwrap();
    }
}
