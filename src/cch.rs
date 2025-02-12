use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    path::Path,
};

use crate::{graph::Graph, library};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
struct OrderedNode {
    position: usize,
    node: usize,
}

pub fn compute_separator_sizes_from_order(graph: &Graph, order: &[usize], output: &Path) {
    let directed = get_directed_graph(graph, order);
    let tree = chordalize_and_tree(&directed, order);
    //let tree = get_lowest_neighbor_tree(&chordalized, order);
    let root = *order.last().unwrap();
    let subtree_sizes = get_subtree_sizes(&tree, root);
    library::clear_file(output);
    traverse_separator_tree(&tree, root, &subtree_sizes, [root].to_vec(), output);
}

pub fn chordalize_and_tree(directed_graph: &Graph, order: &[usize]) -> Graph {
    let pos = get_positions_from_order(order);
    let mut data: Vec<Vec<usize>> = directed_graph
        .nodes_iter()
        .map(|v| {
            directed_graph
                .get_neighbors(v)
                .into_iter()
                .copied()
                .collect::<Vec<_>>()
        })
        .collect();

    let mut tree = Graph::with_node_count(directed_graph.get_num_nodes());

    for i in 0..directed_graph.get_num_nodes() {
        if i & 0b11111 == 0 {
            println!("Chordalizing: {}/{}", i, directed_graph.get_num_nodes());
        }

        let v = order[i];
        if data[v].is_empty() {
            continue;
        }
        data[v].sort_by_key(|&x| pos[x]);
        data[v].dedup();
        let lowest_neighbor = data[v][0];

        if lowest_neighbor < v {
            let (l, r) = data.split_at_mut(v);
            l[lowest_neighbor].extend(&r[0][1..]);
        } else {
            let (l, r) = data.split_at_mut(v + 1);
            r[lowest_neighbor - v - 1].extend(&l[v][1..]);
        }
        tree.add_directed_edge(v, lowest_neighbor);
        data[v].clear();
    }

    tree.invert();
    tree
}

pub fn get_lowest_neighbor_tree(chordalized_graph: &Graph, order: &[usize]) -> Graph {
    let mut pos = get_positions_from_order(order);

    let mut tree = Graph::with_node_count(chordalized_graph.get_num_nodes());

    for u in 0..chordalized_graph.get_num_nodes() {
        let v = chordalized_graph
            .get_neighbors(u)
            .iter()
            .filter(|&&v| pos[v] > pos[u])
            .min_by_key(|&v| pos[*v]);

        if let Some(&v) = v {
            tree.add_directed_edge(v, u);
        }
    }

    tree
}

pub fn get_subtree_sizes(tree: &Graph, root: usize) -> Vec<usize> {
    let mut subtree_sizes = vec![1; tree.get_num_nodes()];
    get_subtree_sizes_rec(tree, root, &mut subtree_sizes);
    subtree_sizes
}

fn get_subtree_sizes_rec(tree: &Graph, node: usize, subtree_sizes: &mut [usize]) {
    for child in tree.get_neighbors(node) {
        get_subtree_sizes_rec(tree, *child, subtree_sizes);
        subtree_sizes[node] += subtree_sizes[*child];
    }
}

pub fn traverse_separator_tree(
    tree: &Graph,
    node: usize,
    subtree_sizes: &[usize],
    mut separator: Vec<usize>,
    output: &Path,
) {
    let children = tree.get_neighbors(node);
    let sizes = children
        .iter()
        .map(|&x| subtree_sizes[x])
        .collect::<Vec<_>>();
    let total_size = sizes.iter().sum::<usize>();
    let cutoff_size = usize::max(((0.1 * total_size as f64) as usize), 0);
    let branches = sizes.iter().filter(|&size| size > &cutoff_size).count();

    if branches == 1 {
        for (child, size) in children.iter().zip(sizes.iter()) {
            if size > &cutoff_size {
                separator.push(*child);
                traverse_separator_tree(tree, *child, subtree_sizes, separator, output);
                return;
            }
        }
    }

    for (child, size) in children.iter().zip(sizes.iter()) {
        if size > &cutoff_size {
            traverse_separator_tree(tree, *child, subtree_sizes, [*child].to_vec(), output);
        }
    }

    library::append_to_file(
        output,
        &format!("{} {}\n", total_size + separator.len(), separator.len()),
    );
}

// turns an order into a position array: at index i is the position of node i
// makes O(1) lookups for the position of a node possible
pub fn get_positions_from_order(order: &[usize]) -> Vec<usize> {
    let mut pos = vec![0; order.len()];
    order.iter().enumerate().for_each(|(i, &v)| pos[v] = i);
    pos
}

pub fn get_directed_graph(graph: &Graph, order: &[usize]) -> Graph {
    let mut pos = get_positions_from_order(order);
    let mut g = Graph::with_node_count(graph.get_num_nodes());

    for (u, v) in graph.get_edges() {
        if pos[u] < pos[v] {
            g.add_directed_edge(u, v);
        } else {
            g.add_directed_edge(v, u);
        }
    }

    g
}

#[cfg(test)]
mod test {
    use crate::graph::example::example_c4;

    use super::*;

    #[test]
    fn c4_chordalization() {
        let g = example_c4().graph;
        let order = vec![0, 2, 1, 3];
        let directed = get_directed_graph(&g, &order);
        let tree = chordalize_and_tree(&directed, &order);
        tree.print();

        //chordalized.print();
        //assert_eq!(chordalized.get_neighbors(0), &HashSet::from([1, 3]));
        //assert_eq!(chordalized.get_neighbors(1), &HashSet::from([3]));
        //assert_eq!(chordalized.get_neighbors(2), &HashSet::from([1, 3]));
        //assert_eq!(chordalized.get_neighbors(3), &HashSet::new());
    }

    #[test]
    fn c4_tree() {
        let g = example_c4().graph;
        let order = vec![0, 2, 1, 3];
        let directed = get_directed_graph(&g, &order);
        let chordalized = chordalize_and_tree(&directed, &order);
        let tree = get_lowest_neighbor_tree(&chordalized, &order);
        assert_eq!(tree.get_neighbors(0), &HashSet::new());
        assert_eq!(tree.get_neighbors(1), &HashSet::from([0, 2]));
        assert_eq!(tree.get_neighbors(2), &HashSet::new());
        assert_eq!(tree.get_neighbors(3), &HashSet::from([1]));
    }

    #[test]
    fn c4_separator() {
        let g = example_c4().graph;
        let order = vec![0, 2, 1, 3];
        let tempfile = tempfile::NamedTempFile::new().unwrap();
        compute_separator_sizes_from_order(&g, &order, tempfile.path());
        let content = std::fs::read_to_string(tempfile).unwrap();
        assert_eq!(content.lines().count(), 3);
        assert_eq!(content.lines().nth(0).unwrap(), "1 1");
        assert_eq!(content.lines().nth(1).unwrap(), "1 1");
        assert_eq!(content.lines().nth(2).unwrap(), "4 2");
    }
}
