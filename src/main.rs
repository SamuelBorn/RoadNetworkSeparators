pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod library;
pub mod local;
pub mod separator;
pub mod random_set;

use std::fs;
use std::path::Path;

use cch::compute_separator_sizes_from_order;
use example::*;
use graph::geometric_graph::GeometricGraph;
use graph::highway::build_highway_network;
use graph::voronoi::{voronoi_example, voronoi_example_small};
use graph::{cbrt_maximal, delaunay, example, grid, nested_grid, Graph};
use library::{read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec};
use separator::Mode::*;

fn main() {
    let g = Graph::from_file(Path::new("./output/cbrt_maximal_avg_deg_50k/")).unwrap();
    g.parallel_separator(Fast, Some(Path::new("./output/sep_cbrt_maximal_avg_deg_50k.txt")));
}
