use std::collections::HashSet;

use crate::graph::Graph;

fn compute_log_table(n: usize) -> Vec<usize> {
    let mut log_table = vec![0; n + 1];
    if n > 0 {
        for i in 2..=n {
            log_table[i] = log_table[i / 2] + 1;
        }
    }
    log_table
}

struct SparseTable {
    st: Vec<Vec<usize>>,
    log_table: Vec<usize>,
}

impl SparseTable {
    fn new(values_to_compare: &[usize]) -> Self {
        let n = values_to_compare.len();
        if n == 0 {
            return SparseTable {
                st: Vec::new(),
                log_table: compute_log_table(0),
            };
        }

        let num_cols = compute_log_table(n).last().copied().unwrap_or(0) + 1;

        let mut st_table = vec![vec![0; num_cols]; n];

        for i in 0..n {
            st_table[i][0] = i;
        }

        for j_power in 1..num_cols {
            let block_half_len = 1 << (j_power - 1);
            for i_start_idx in 0..=(n - (1 << j_power)) {
                let idx1 = st_table[i_start_idx][j_power - 1];
                let idx2 = st_table[i_start_idx + block_half_len][j_power - 1];
                if values_to_compare[idx1] <= values_to_compare[idx2] {
                    st_table[i_start_idx][j_power] = idx1;
                } else {
                    st_table[i_start_idx][j_power] = idx2;
                }
            }
        }

        SparseTable {
            st: st_table,
            log_table: compute_log_table(n),
        }
    }

    fn query_min_idx(&self, l: usize, r: usize, values_to_compare: &[usize]) -> usize {
        if self.st.is_empty() {
            panic!("Query on empty sparse table");
        }

        let length = r - l + 1;
        let j_power = self.log_table[length];

        let idx1 = self.st[l][j_power];
        let idx2 = self.st[r + 1 - (1 << j_power)][j_power];

        if values_to_compare[idx1] <= values_to_compare[idx2] {
            idx1
        } else {
            idx2
        }
    }
}

fn euler_dfs_recursive(
    u: usize,
    d: usize,
    parent_of_u: Option<usize>,
    graph: &Graph,
    nodes_tour: &mut Vec<usize>,
    depths_tour: &mut Vec<usize>,
    first_occ: &mut [usize],
    current_tour_idx: &mut usize,
    visited_dfs: &mut [bool],
) {
    visited_dfs[u] = true;
    nodes_tour.push(u);
    depths_tour.push(d);
    first_occ[u] = *current_tour_idx;
    *current_tour_idx += 1;

    if let Some(neighbors) = graph.data.get(u) {
        for &v_neighbor in neighbors {
            if Some(v_neighbor) == parent_of_u {
                continue;
            }
            if !visited_dfs[v_neighbor] {
                euler_dfs_recursive(
                    v_neighbor,
                    d + 1,
                    Some(u),
                    graph,
                    nodes_tour,
                    depths_tour,
                    first_occ,
                    current_tour_idx,
                    visited_dfs,
                );
                nodes_tour.push(u);
                depths_tour.push(d);
                *current_tour_idx += 1;
            }
        }
    }
}

pub struct LcaUtil {
    n_nodes: usize,
    nodes_in_tour: Vec<usize>,
    depth_in_tour: Vec<usize>,
    first_occurrence: Vec<usize>,
    sparse_table: SparseTable,
}

impl LcaUtil {
    pub fn new(graph: &Graph) -> Self {
        let root = 0;
        let n = graph.data.len();
        if n == 0 {
            return LcaUtil {
                n_nodes: 0,
                nodes_in_tour: Vec::new(),
                depth_in_tour: Vec::new(),
                first_occurrence: Vec::new(),
                sparse_table: SparseTable::new(&[]),
            };
        }
        if root >= n {
            panic!(
                "Root node index {} is out of bounds for graph with {} nodes.",
                root, n
            );
        }

        let tour_capacity = if n == 0 { 0 } else { 2 * n - 1 };
        let mut nodes_in_tour_vec = Vec::with_capacity(tour_capacity);
        let mut depth_in_tour_vec = Vec::with_capacity(tour_capacity);
        let mut first_occurrence_vec = vec![0; n];
        let mut visited_dfs_vec = vec![false; n];
        let mut current_tour_idx_val = 0;

        euler_dfs_recursive(
            root,
            0,
            None,
            graph,
            &mut nodes_in_tour_vec,
            &mut depth_in_tour_vec,
            &mut first_occurrence_vec,
            &mut current_tour_idx_val,
            &mut visited_dfs_vec,
        );

        let st = SparseTable::new(&depth_in_tour_vec);

        LcaUtil {
            n_nodes: n,
            nodes_in_tour: nodes_in_tour_vec,
            depth_in_tour: depth_in_tour_vec,
            first_occurrence: first_occurrence_vec,
            sparse_table: st,
        }
    }

    pub fn query(&self, u: usize, v: usize) -> usize {
        if self.n_nodes == 0 {
            panic!("Query on empty LcaUtil structure.");
        }
        if u >= self.n_nodes || v >= self.n_nodes {
            panic!(
                "Node index out of bounds in query. u: {}, v: {}, n_nodes: {}",
                u, v, self.n_nodes
            );
        }

        let mut idx_u_tour = self.first_occurrence[u];
        let mut idx_v_tour = self.first_occurrence[v];

        if idx_u_tour > idx_v_tour {
            std::mem::swap(&mut idx_u_tour, &mut idx_v_tour);
        }

        let tour_idx_of_lca_candidate =
            self.sparse_table
                .query_min_idx(idx_u_tour, idx_v_tour, &self.depth_in_tour);

        self.nodes_in_tour[tour_idx_of_lca_candidate]
    }

    pub fn distance(&self, u: usize, v: usize) -> usize {
        if self.n_nodes == 0 {
            panic!("Distance query on empty LcaUtil structure.");
        }
        if u >= self.n_nodes || v >= self.n_nodes {
            panic!(
                "Node index out of bounds in distance query. u: {}, v: {}, n_nodes: {}",
                u, v, self.n_nodes
            );
        }

        let lca = self.query(u, v);
        let depth_u = self.depth_in_tour[self.first_occurrence[u]];
        let depth_v = self.depth_in_tour[self.first_occurrence[v]];
        let depth_lca = self.depth_in_tour[self.first_occurrence[lca]];

        depth_u + depth_v - 2 * depth_lca
    }
}

#[cfg(test)]
mod test {
    use crate::graph::tree::generate_random_tree;

    #[test]
    fn lca_simple() {
        let n = 10;
        let g = generate_random_tree(n);
        g.print();
        let lca = super::LcaUtil::new(&g);
        for i in 0..n {
            for j in i + 1..n {
                println!("{} {} {}", i, j, lca.distance(i, j));
            }
        }
    }
}
