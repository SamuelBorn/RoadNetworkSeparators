use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::Rng;

use crate::graph::tree;
use crate::graph::Graph;

pub fn generate_random_connected(n: usize, m: usize) -> Graph {
    let mut g = tree::generate_random_tree(n);
    let mut edge_count = n - 1;

    while edge_count < m {
        let u = rand::thread_rng().gen_range(0..n);
        let v = rand::thread_rng().gen_range(0..n);

        if !g.has_edge(u, v) {
            g.add_edge(u, v);
            edge_count += 1;
        }
    }

    g
}

pub fn generate_local_graph(n: usize, m: usize) -> Graph {
    let mut g = tree::generate_random_tree(n);
    let mut edge_count = n - 1;

    while edge_count < m {
        let u = rand::thread_rng().gen_range(0..n);

        let (nodes, distances): (Vec<_>, Vec<_>) =
            g.get_extended_neighborhood(u, 200).into_iter().unzip();
        let weights = distances
            .into_iter()
            .map(|d| 1.0 / d.pow(7) as f32)
            .collect::<Vec<_>>();
        let dist = WeightedIndex::new(&weights).unwrap();
        let mut rng = rand::thread_rng();
        let v = nodes[dist.sample(&mut rng)];

        if !g.has_edge(u, v) {
            g.add_edge(u, v);
            edge_count += 1;
        }
    }

    g
}

#[cfg(test)]
mod tests {
    use crate::graph::tree::generate_random_tree;

    use super::*;

    #[test]
    fn test_generate_local_graph() {
        return;
        let n = 12000;
        let m = 15000;
        let g = generate_local_graph(n, m);
        assert_eq!(g.get_num_nodes(), n);
        assert_eq!(g.get_num_edges(), m * 2);

        // expect 22 at highest level
        g.recurse_separator(crate::separator::Mode::Fast, None);
    }

    #[test]
    fn test_generate_random_connected() {
        let n = 12000;
        let m = 15000;
        // let g = generate_random_connected(n, m);
        let g = generate_random_tree(n);

        dbg!(g.get_diameter());
    }

    #[test]
    fn random_spanning_tree_overview() {
        for i in 2..21 {
            let n = 2_usize.pow(i);
            let g1 = generate_random_connected(n, (1.25 * n as f32) as usize);
            let g2 = generate_random_connected(n, (1.25 * n as f32) as usize);
            let g3 = generate_random_connected(n, (1.25 * n as f32) as usize);
            let d1 = g1.get_diameter();
            let d2 = g2.get_diameter();
            let d3 = g3.get_diameter();
            let avg = (d1 + d2 + d3) / 3;
            println!("{} {}", n, avg);
        }
    }
}
