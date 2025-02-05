pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use graph::geometric_graph::{GeometricGraph, Position, AABB};
use graph::planar::naive_find_intersections;
use graph::{delaunay, grid, Graph};
use separator::Mode::*;

fn main() {
    let mut g = GeometricGraph::from_file(Path::new("../Graphs/karlsruhe")).unwrap();
}
