pub mod cch;
pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use cch::{chordalize_and_tree, get_directed_graph, get_subtree_sizes};
use graph::delaunay::{length_restricted_delaunay, random_delaunay};
use graph::geometric_graph::{GeometricGraph, Position, AABB};
use graph::planar::naive_find_intersections;
use graph::{delaunay, example, grid, Graph};
use library::{read_bin_u32_vec_to_usize, read_binary_vec, read_text_vec, write_binary_vec};
use separator::Mode::*;

fn main() {
    let mut g = GeometricGraph::from_file(Path::new("../Graphs2/karlsruhe")).unwrap();
    g.graph.make_undirected();
    let g = g.largest_connected_component();
    g.save(Path::new("../Graphs2/karlsruhe-connected-bidirectional"));

    let mut g = GeometricGraph::from_file(Path::new("../Graphs2/germany")).unwrap();
    g.graph.make_undirected();
    let g = g.largest_connected_component();
    g.save(Path::new("../Graphs2/germany-connected-bidirectional"));
    println!("done");

    let mut g = GeometricGraph::from_file(Path::new("../Graphs2/europe")).unwrap();
    g.graph.make_undirected();
    let g = g.largest_connected_component();
    g.save(Path::new("../Graphs2/europe-connected-bidirectional"));
}
