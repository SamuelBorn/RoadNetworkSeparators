use std::path::Path;

use geo::Point;

use crate::{graph::Graph, library};

use super::geometric_graph::GeometricGraph;

pub const DEGREE_DISTRIBUTION_GER: [f64; 10] = [
    0.0,
    0.21926495500273885,
    0.14653504242688228,
    0.5506052026552709,
    0.08178824037766508,
    0.0016999235485956178,
    0.00009951534464091634,
    0.00000642594721066999,
    0.0000005210227468110803,
    0.00000017367424893702675,
];

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
        Point::new(0.0, 0.0),
        Point::new(0.0, 1.0),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
        Point::new(2.0, 0.0),
        Point::new(3.0, 0.0),
        Point::new(3.0, 1.0),
        Point::new(4.0, 1.0),
        Point::new(4.0, 0.0),
    ];

    GeometricGraph::new(g, positions)
}

pub fn example_c4() -> GeometricGraph {
    let g = Graph::from_edge_list(vec![(0, 1), (1, 2), (2, 3), (3, 0)]);
    let positions = vec![
        Point::new(0.0, 0.0),
        Point::new(0.0, 1.0),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
    ];

    GeometricGraph::new(g, positions)
}

pub fn karlsruhe() -> Graph {
    Graph::from_file(Path::new("../Graphs/karlsruhe-connected-bidirectional")).unwrap()
}

pub fn germany() -> Graph {
    Graph::from_file(Path::new("../Graphs/germany-connected-bidirectional")).unwrap()
}

pub fn europe() -> Graph {
    Graph::from_file(Path::new("../Graphs/europe-connected-bidirectional")).unwrap()
}

pub fn geometric_karlsruhe() -> GeometricGraph {
    GeometricGraph::from_file(Path::new("../Graphs/karlsruhe-connected-bidirectional")).unwrap()
}

pub fn geometric_germany() -> GeometricGraph {
    GeometricGraph::from_file(Path::new("../Graphs/germany-connected-bidirectional")).unwrap()
}

pub fn geometric_europe() -> GeometricGraph {
    GeometricGraph::from_file(Path::new("../Graphs/europe-connected-bidirectional")).unwrap()
}

pub fn ord_karlsruhe() -> Vec<usize> {
    library::read_to_usize_vec(Path::new("./output/ord/karlsruhe"))
}

pub fn ord_germany() -> Vec<usize> {
    library::read_to_usize_vec(Path::new("./output/ord/germany"))
}

pub fn ord_europe() -> Vec<usize> {
    library::read_to_usize_vec(Path::new("./output/ord/europe"))
}
