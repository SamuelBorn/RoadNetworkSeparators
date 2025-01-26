pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::path::Path;

use graph::geometric_graph::{GeometricGraph, Position};
use graph::{grid, Graph};
use separator::Mode::*;

fn main() {
    //let g = Graph::from_file(Path::new("../Graphs/karlsruhe")).unwrap();
    //let g = GeometricGraph::from_file(Path::new("../Graphs/karlsruhe")).unwrap();
    //let g = Graph::from_file(Path::new("../Graphs/germany).unwrap();

    let g = GeometricGraph::new(
        Graph::from_edge_list(vec![(0, 1), (1, 2), (2, 3), (3, 0)]),
        vec![
            Position(0.0, 0.0),
            Position(0.0, 1.0),
            Position(1.0, 1.0),
            Position(1.0, 0.0),
        ],
    );

    g.to_file(Path::new("output/geometric_graph")).unwrap();
}
