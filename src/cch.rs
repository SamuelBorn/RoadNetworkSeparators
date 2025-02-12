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

    for i in 0..data.len() {
        if i & 0b1111111 == 0 {
            println!("{} / {}", i, data.len());
        }

        let v = order[i];
        if data[v].is_empty() {
            continue;
        }

        // deduplicate, find lowest neighbor, remember edge
        data[v].sort_by(|a, b| pos[*b].cmp(&pos[*a]));
        data[v].dedup();
        let lowest_neighbor = data[v].pop().unwrap();
        tree.add_directed_edge(lowest_neighbor, v);

        // add neighbors to lowest neighbor
        let mut temp = std::mem::take(&mut data[v]);
        data[lowest_neighbor].append(&mut temp);
    }

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
    let num_nodes = tree.get_num_nodes();
    // Every node initially counts as size 1 (itself)
    let mut subtree_sizes = vec![1; num_nodes];
    // The stack holds tuples: (current_node, parent_of_current_node, visited_flag)
    let mut stack = Vec::new();
    stack.push((root, None, false));

    while let Some((node, parent, visited)) = stack.pop() {
        if !visited {
            // Push the node back as visited to process after its children.
            stack.push((node, parent, true));
            // Push each child. No need to reverse because HashSet doesn't implement DoubleEndedIterator.
            for &child in tree.get_neighbors(node) {
                stack.push((child, Some(node), false));
            }
        } else {
            // All children of `node` have been processed, so update parent's subtree size.
            if let Some(p) = parent {
                subtree_sizes[p] += subtree_sizes[node];
            }
        }
    }

    subtree_sizes
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
    let cutoff_size = usize::max(((0.1 * total_size as f64) as usize), 100);
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
