use hashbrown::HashSet;
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
    if n <= 1 {
        return Graph::with_node_count(1);
    }

    let sep_size = (n as f64).cbrt() as usize;
    let subgraph_size = (n - sep_size) / 2;

    let mut g = generate_clique(sep_size);
    let g1 = generate_cbrt_maximal(subgraph_size);
    let g2 = g1.clone();
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
    use super::*;

    #[test]
    fn cbrt_maximal() {
        let g = generate_cbrt_maximal(2000);
        g.info();
        assert_eq!(g.get_num_nodes(), 2506);
        assert_eq!(g.get_num_edges(), 87974);
    }
}
