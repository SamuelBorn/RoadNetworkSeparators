pub mod cch;
pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use cch::{chordalize_and_tree, compute_separator_sizes_from_order, get_directed_graph, get_subtree_sizes};
use graph::delaunay::{length_restricted_delaunay, random_delaunay};
use graph::geometric_graph::GeometricGraph;
use graph::voronoi::{voronoi_example, voronoi_example_small};
use graph::{delaunay, example, grid, nested_grid, Graph};
use library::{read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec};
use separator::Mode::*;

fn main() {
    let g = Graph::from_file(Path::new("output/voronoi-non-disk-300top")).unwrap();
    let ord = library::read_to_usize_vec(Path::new("output/ord_voronoi-non-disk-300top.bin"));
    compute_separator_sizes_from_order(&g, &ord, Path::new("output/sep_voronoi-non-disk-300top.txt"));
}
