use crate::graph::{geometric_graph::Position, Graph};

use super::geometric_graph::GeometricGraph;

pub fn example1() -> GeometricGraph {
    let g = Graph::from_edge_list(vec![
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (3, 4),
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 8),
        (8, 5),
    ]);

    let positions = vec![
        Position::new(0.0, 0.0),
        Position::new(0.0, 1.0),
        Position::new(1.0, 1.0),
        Position::new(1.0, 0.0),
        Position::new(2.0, 0.0),
        Position::new(3.0, 0.0),
        Position::new(3.0, 1.0),
        Position::new(4.0, 1.0),
        Position::new(4.0, 0.0),
    ];

    GeometricGraph::new(g, positions)
}

pub fn example_c4() -> GeometricGraph {
    let g = Graph::from_edge_list(vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let positions = vec![
        Position::new(0.0, 0.0),
        Position::new(0.0, 1.0),
        Position::new(1.0, 1.0),
        Position::new(1.0, 0.0),
    ];

    GeometricGraph::new(g, positions)
}
