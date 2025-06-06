use hashbrown::{HashMap, HashSet};
use rand::Rng;
use rayon::prelude::*;

use crate::graph::Graph;

pub fn random_tree(n: usize) -> Graph {
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
    use std::path::Path;

    use rayon::iter::IntoParallelIterator;

    use super::*;

    #[test]
    fn test_generate_random_tree() {
        let graph = random_tree(100);
        assert_eq!(graph.get_num_nodes(), 100);
        assert_eq!(graph.get_num_edges(), 99);

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
                let g = random_tree(n);
                result.push_str(&format!("{} {}\n", n, g.get_hop_diameter_approx()));
            }
        }

        File::create("output/diameter_overview.txt")
            .unwrap()
            .write_all(result.as_bytes())
            .unwrap();
    }

    #[test]
    fn gen_avg_deg() {
        let sizes = vec![5_000_000, 4_000_000, 3_000_000, 2_000_000, 1_000_000];

        sizes.into_par_iter().for_each(|n| {
            let g = crate::local::no_locality(n, n * 5 / 4);
            g.recurse_separator(
                crate::separator::Mode::Eco,
                Some(Path::new(&format!("./output/sep/RandomAvgDeg_{}", n))),
            );
        });
    }
}
