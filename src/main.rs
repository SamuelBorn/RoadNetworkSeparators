pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use cch::compute_separator_sizes_from_order;
use graph::example::{self, *};
use graph::planar::planarize;
use graph::{cbrt_maximal, delaunay, grid, hierachical_disks, highway, nested_grid, voronoi};
use graph::{geometric_graph::GeometricGraph, Graph};
use library::{
    read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec, write_text_vec,
};
use separator::{get_ord, Mode::*};
use std::path::Path;

fn main() {
    //let city_percentage = vec![1.0, 0.4, 0.4, 0.4];
    //let radii = vec![1500.0, 400.0, 60.0, 10.0];
    //let points_per_level = vec![1000, 270, 50, 10];
    let city_percentage = vec![1.0, 0.5, 0.5, 0.5];
    let radii = vec![120.0, 20.0, 5.0, 1.0];
    let points_per_level = vec![50, 50, 50, 50];

    let g = hierachical_disks::generate_circle_center_graph_v2(
        &points_per_level,
        &city_percentage,
        &radii,
    );
    g.visualize("tmp");
    g.graph.info();
    println!(
        "{} ({})",
        g.graph.get_separator_size(Fast),
        (g.graph.get_num_nodes() as f32).powf(0.33)
    )
}
