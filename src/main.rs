pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use graph::geometric_graph::{GeometricGraph, Position};
use graph::planar::naive_find_intersections;
use graph::{grid, Graph};
use separator::Mode::*;

fn main() {
    //let g = Graph::from_file(Path::new("../Graphs/karlsruhe")).unwrap();
    let g = GeometricGraph::from_file(Path::new("../Graphs/karlsruhe")).unwrap();
    //let g = Graph::from_file(Path::new("../Graphs/germany).unwrap();
}
