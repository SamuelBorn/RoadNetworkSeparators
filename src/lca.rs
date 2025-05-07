use std::collections::HashSet;
use rayon::prelude::*;

use crate::graph::Graph;

pub struct LcaUtil {
    n: usize,
    log_n: usize,
    parent: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
}

impl LcaUtil {
    pub fn new(tree: &Graph) -> Self {
        let root = 0;
        let n = tree.data.len();

        let mut log_n_val = 0;
        while (1 << log_n_val) <= n {
            log_n_val += 1;
        }

        let mut parent_table = vec![vec![None; log_n_val]; n];
        let mut depth_vec = vec![0; n];
        let mut visited_dfs = vec![false; n];

        let mut dfs_stack: Vec<(usize, usize, Option<usize>)> = Vec::new();

        dfs_stack.push((root, 0, None));
        visited_dfs[root] = true;

        while let Some((u_node, d, parent_of_u_opt)) = dfs_stack.pop() {
            depth_vec[u_node] = d;
            parent_table[u_node][0] = parent_of_u_opt;

            if let Some(neighbors) = tree.data.get(u_node) {
                for &v_neighbor in neighbors {
                    if v_neighbor < n {
                        let is_dfs_parent =
                            parent_of_u_opt.map_or(false, |p_node| p_node == v_neighbor);
                        if !is_dfs_parent {
                            if !visited_dfs[v_neighbor] {
                                visited_dfs[v_neighbor] = true;
                                dfs_stack.push((v_neighbor, d + 1, Some(u_node)));
                            }
                        }
                    }
                }
            }
        }

        for j_power in 1..log_n_val {
            for i_node in 0..n {
                if let Some(p_i_j_minus_1) = parent_table[i_node][j_power - 1] {
                    parent_table[i_node][j_power] = parent_table[p_i_j_minus_1][j_power - 1];
                }
            }
        }

        LcaUtil {
            n,
            log_n: log_n_val,
            parent: parent_table,
            depth: depth_vec,
        }
    }

    pub fn query(&self, mut u_node: usize, mut v_node: usize) -> usize {
        if self.n == 0 {
            panic!("Query called on LcaUtil for an empty graph.");
        }
        if u_node >= self.n || v_node >= self.n {
            panic!(
                "Node index out of bounds in query. u: {}, v: {}, n: {}",
                u_node, v_node, self.n
            );
        }

        if self.depth[u_node] < self.depth[v_node] {
            std::mem::swap(&mut u_node, &mut v_node);
        }

        for j_power in (0..self.log_n).rev() {
            if let Some(ancestor_u) = self.parent[u_node][j_power] {
                if self.depth[ancestor_u] >= self.depth[v_node] {
                    u_node = ancestor_u;
                }
            }
        }

        if u_node == v_node {
            return u_node;
        }

        for j_power in (0..self.log_n).rev() {
            match (self.parent[u_node][j_power], self.parent[v_node][j_power]) {
                (Some(p_u), Some(p_v)) if p_u != p_v => {
                    u_node = p_u;
                    v_node = p_v;
                }
                _ => {}
            }
        }
        self.parent[u_node][0].expect("LCA logic error or graph not connected as assumed")
    }

    pub fn distance(&self, u_node: usize, v_node: usize) -> usize {
        let lca = self.query(u_node, v_node);
        self.depth[u_node] + self.depth[v_node] - 2 * self.depth[lca]
    }
}

#[cfg(test)]
mod test {
    use rayon::iter::IntoParallelIterator;

    use super::*;
    use crate::graph::tree::generate_random_tree;

    #[test]
    fn lca_simple() {
        let n = 100_000;
        let g = generate_random_tree(n);
        let lca = LcaUtil::new(&g);

        println!("LCA initialized for {} nodes", n);
        for i in 0..100 {
            dbg!(i);
            (0..n).into_par_iter().for_each(|j| {
                lca.distance(i, j);
            });
        }
        println!("LCA distance queries completed for {} nodes", n);

        // for i in 0..n {
        //     for j in i + 1..n {
        //         println!("LCA of {} and {} is {}", i, j, lca.query(i, j));
        //         println!("Distance between {} and {} is {}", i, j, lca.distance(i, j));
        //     }
        // }
    }
}
