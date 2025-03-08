pub mod cch;
pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use cch::{chordalize_and_tree, compute_separator_sizes_from_order, get_subtree_sizes};
use graph::geometric_graph::GeometricGraph;
use graph::highway::build_highway_network;
use graph::voronoi::{voronoi_example, voronoi_example_small};
use graph::{delaunay, example, grid, nested_grid, Graph};
use library::{read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec};
use separator::Mode::*;

fn main() {
    let mut g = Graph::from_file(Path::new("./output/germany_planar/")).unwrap();
    let mut ord = read_to_usize_vec(Path::new("./output/ord_germany_planar.bin"));
    compute_separator_sizes_from_order(&g, &ord, Path::new("./output/sep_germany_planar.txt"));
}
