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
    //let order = read_bin_u32_vec_to_usize(Path::new("output/ord_germany_new.bin"));
    //let root = *order.last().unwrap();

    let g = example::geometric_germany_server();
    let g = g.largest_subgraph();
    g.to_file(Path::new("output/germany_connected"));
}
