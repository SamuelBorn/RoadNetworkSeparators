pub mod cch;
pub mod graph;
pub mod library;
pub mod local;
pub mod separator;
pub mod kruskal;

use std::fs;
use std::path::Path;

use cch::compute_separator_sizes_from_order;
use graph::geometric_graph::GeometricGraph;
use graph::highway::build_highway_network;
use graph::voronoi::{voronoi_example, voronoi_example_small};
use graph::{cbrt_maximal, delaunay, example, grid, nested_grid, Graph};
use library::{read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec};
use separator::Mode::*;
use example::*;

fn main() {
    let mut g = cbrt_maximal::generate_cbrt_maximal(10000);
    g.enforce_average_degree_connected(2.0);
    g.info();
    g.save(Path::new("./output/cbrt_maximal_avg_deg"));
    g.degree_distribution().iter().enumerate().for_each(|(i, &x)| {
        println!("{} {}", i, x);
    });
}
