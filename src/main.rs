pub mod cch;
pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use cch::compute_separator_sizes_from_order;
use graph::geometric_graph::GeometricGraph;
use graph::highway::build_highway_network;
use graph::voronoi::{voronoi_example, voronoi_example_small};
use graph::{delaunay, example, grid, nested_grid, Graph};
use library::{read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec};
use separator::Mode::*;

fn main() {
    let mut g = example::germany();
    let mut ord = library::read_to_usize_vec(Path::new("output/ord_germany.bin"));
    compute_separator_sizes_from_order(&g, &ord, Path::new("output/sep_sizes_germany.txt"));
}
