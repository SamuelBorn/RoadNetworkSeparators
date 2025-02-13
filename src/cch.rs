use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    hash::Hash,
    path::Path,
};

use crate::{graph::Graph, library, separator};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
struct OrderedNode {
    position: usize,
    node: usize,
}

pub fn compute_separator_sizes_from_order(graph: &Graph, order: &[usize]) {
    let directed = get_directed_graph(graph, order);
    let tree = chordalize_and_tree(&directed, order);
    let root = *order.last().unwrap();
    let subtree_sizes = get_subtree_sizes(&tree, root);
    traverse_separator_tree(&tree, root, &subtree_sizes);
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

pub fn get_subtree_sizes(tree: &Graph, root: usize) -> Vec<usize> {
    let mut stack = Vec::from([(root, false)]);
    let mut sizes = vec![0; tree.get_num_nodes()];

    while let Some((node, processed)) = stack.pop() {
        if processed {
            sizes[node] = tree.get_neighbors(node).iter().map(|&v| sizes[v]).sum::<usize>() + 1;
        } else {
            stack.push((node, true));
            for &child in tree.get_neighbors(node) {
                stack.push((child, false));
            }
        }
    }

    sizes
}

pub fn traverse_separator_tree(tree: &Graph, root: usize, subtree_sizes: &[usize]) {
    let mut queue = vec![(root, 1)];

    while let Some((node, separator_size)) = queue.pop() {
        let cutoff_size = 10.max((0.1 * subtree_sizes[node] as f64) as usize);
        let large_children = tree
            .get_neighbors(node)
            .iter()
            .filter(|&&child| subtree_sizes[child] > cutoff_size)
            .collect::<Vec<_>>();

        match large_children.len() {
            0 => {}
            1 => {
                for child in large_children {
                    queue.push((*child, separator_size + 1));
                }
            }
            _ => {
                for child in large_children {
                    queue.push((*child, 1));
                }
                println!(
                    "{} {}",
                    subtree_sizes[node] + separator_size,
                    separator_size
                );
            }
        }
    }
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
        //let content = std::fs::read_to_string(tempfile).unwrap();
        //assert_eq!(content.lines().count(), 3);
        //assert_eq!(content.lines().nth(0).unwrap(), "1 1");
        //assert_eq!(content.lines().nth(1).unwrap(), "1 1");
        //assert_eq!(content.lines().nth(2).unwrap(), "4 2");
    }
}
