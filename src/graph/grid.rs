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

#[cfg(test)]
mod tests {
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
}
