use std::collections::HashSet;

use crate::graph::Graph;
use bimap::BiMap;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::graph::positioned_graph::Position;
use crate::graph::positioned_graph::PositionedGraph;

pub fn generate_radom_tree_on_grid(n: usize) -> PositionedGraph {
    let directions = vec![
        Position(1, 0),
        Position(0, 1),
        Position(-1, 0),
        Position(0, -1),
    ];
    let mut g = Graph::new(vec![Vec::new(); n]);
    let mut rng = rand::thread_rng();

    let mut node = 0 as usize;
    let mut position = Position(0, 0);
    let mut map = BiMap::new();
    map.insert(node, position);

    while map.len() < n {
        position = position + *directions.choose(&mut rng).unwrap();

        if !map.contains_right(&position) {
            g.add_edge(node, map.len());
            map.insert(map.len(), position);
        }

        node = map.get_by_right(&position).unwrap().clone();
    }

    return PositionedGraph::new(g, map);
}

pub fn generate_grid(side_length: usize) -> Graph {
    let mut g = Graph::with_node_count(side_length * side_length);

    for i in 0..side_length {
        for j in 0..side_length {
            let node = i * side_length + j;
            if i > 0 {
                g.add_edge(node, node - side_length);
            }
            if j > 0 {
                g.add_edge(node, node - 1);
            }
        }
    }

    g
}

pub fn generate_grid_with_avg_degree(side_length: usize, avg_degree: f64) -> Graph {
    let mut g = generate_grid(side_length);
    let num_edges = g.get_num_edges();
    let mut rng = rand::thread_rng();

    let goal_num_edges = (avg_degree * g.get_num_nodes() as f64 / 2.0) as usize;

    for i in goal_num_edges..num_edges {
        g.remove_random_edge();
    }

    g.get_largest_subgraph()
}

pub fn generate_complete_grid(n: usize) -> PositionedGraph {
    let side_length = (n as f64).sqrt() as usize;

    let mut positions = BiMap::with_capacity(n);
    let mut g = Graph::with_node_count(n);

    for i in 0..side_length {
        for j in 0..side_length {
            let node = i * side_length + j;
            positions.insert(node, Position(i as i32, j as i32));
            if i > 0 {
                g.add_edge(node, node - side_length);
            }
            if j > 0 {
                g.add_edge(node, node - 1);
            }
        }
    }

    PositionedGraph::new(g, positions)
}

pub fn remove_random_edges_from_grid(g: &mut PositionedGraph, num_edges: usize) {
    let mut rng = rand::thread_rng();

    let side_length = (g.graph.get_num_nodes() as f64).sqrt() as usize;

    for _ in 0..num_edges {
        let i = rng.gen_range(0..side_length);
        let j = rng.gen_range(0..side_length);

        let node = i * side_length + j;
        let neighbors = g.graph.get_neighbors(node);
        let neighbor = neighbors.choose(&mut rng);

        if let Some(&v) = neighbor {
            g.graph.remove_edge(node, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::separator::Mode::*;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_generate_random_tree_on_grid() {
        return;
        let g = generate_radom_tree_on_grid(10);
        // print coordinate map
        for i in 0..g.graph.get_num_nodes() {
            println!("{:?} -> {:?}", i, g.positions.get_by_left(&i).unwrap());
        }
        g.graph.print();
    }

    #[test]
    fn test_generate_complete_grid() {
        return;
        let g = generate_complete_grid(9);
        // print coordinate map
        for i in 0..g.graph.get_num_nodes() {
            println!("{:?} -> {:?}", i, g.positions.get_by_left(&i).unwrap());
        }
        // print edges
        for i in 0..g.graph.get_num_nodes() {
            for &j in g.graph.get_neighbors(i) {
                println!("{:?} -> {:?}", i, j);
            }
        }
        g.graph.print();
    }

    #[test]
    fn test_remove_random_edges_from_grid() {
        return;
        let side_length = 300;
        let num_nodes = side_length * side_length;
        let num_edges = 2 * side_length * side_length - 2 * side_length;
        let desired_edges = (num_nodes as f64 * 1.25) as usize;
        let mut g = generate_complete_grid(num_nodes);
        remove_random_edges_from_grid(&mut g, num_edges - desired_edges);
        let separator: HashSet<usize> = HashSet::new();
        let subgraphs = g.graph.get_subgraphs(&separator);

        // pick the largest subgraph
        let mut max_size = 0;
        let mut max_index = 0;
        for (i, subgraph) in subgraphs.iter().enumerate() {
            if subgraph.get_num_nodes() > max_size {
                max_size = subgraph.get_num_nodes();
                max_index = i;
            }
        }

        let largest_subgraph = &subgraphs[max_index];

        println!(
            "Largest subgraph size: {}",
            largest_subgraph.get_num_nodes()
        );

        //let sep = g.graph.recurse_separator(1, Fast);
        //.get_separator(2, 0.33, 1, crate::separator::Mode::Strong);

        //println!("Separator size: {}", sep.len());
    }

    #[test]
    fn test_complete_grid_separator() {
        return;
        let side_length = 300;
        let num_nodes = side_length * side_length;
        let mut g = generate_complete_grid(num_nodes);

        let sep = g.graph.get_separator_wrapper(Eco);

        println!("Separator size complete grid: {}", sep.len());
    }

    #[test]
    fn test_generate_grid() {
        return;
        let g = generate_grid(4);
        g.print();
    }

    #[test]
    fn test_generate_grid_with_avg_degree() {
        let g = generate_grid_with_avg_degree(5, 2.0);
        g.print();
    }
}
