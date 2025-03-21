use hashbrown::HashSet;
use rand::seq::index::sample;
use rayon::prelude::*;

use super::Graph;

fn generate_clique(n: usize) -> Graph {
    let mut g = Graph::with_node_count(n);
    for i in 0..n {
        for j in i + 1..n {
            g.add_edge(i, j);
        }
    }
    g
}

// g2 will be added to g1
fn combine_graph(g1: &mut Graph, mut g2: Graph) {
    let mut offset_data = g2
        .data
        .par_iter()
        .map(|s| {
            s.iter()
                .map(|x| x + g1.get_num_nodes())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();
    g1.data.append(&mut offset_data);
}

pub fn generate_cbrt_maximal(n: usize) -> Graph {
    if n <= 2 {
        return Graph::with_node_count(1);
    }

    let sep_size = (n as f64).cbrt() as usize;
    let subgraph_size = (n - sep_size) / 2;

    let mut g = generate_clique(sep_size);
    let g1 = generate_cbrt_maximal(subgraph_size);
    let g2 = g1.clone();
    let subgraph_size = g1.get_num_nodes();
    combine_graph(&mut g, g1);
    combine_graph(&mut g, g2);

    for i in 0..sep_size {
        for j in 0..subgraph_size {
            g.add_edge(i, j + sep_size);
        }
        for j in 0..subgraph_size {
            g.add_edge(i, j + sep_size + subgraph_size);
        }
    }

    g
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::graph::example;

    use super::*;

    #[test]
    fn cbrt_maximal() {
        let g = generate_cbrt_maximal(2000);
        g.info();
        assert_eq!(g.get_num_nodes(), 2506);
        assert_eq!(g.get_num_edges(), 87974);
    }

    #[test]
    fn gen_cbrt_maximal_degree_distribution() {
        let mut g = generate_cbrt_maximal(20000);
        g.approx_degrees(&example::DEGREE_DISTRIBUTION_GER);
        g.info();
        g.save(Path::new("./output/cbrt_maximal_avg_deg_20k/"));
    }
}
