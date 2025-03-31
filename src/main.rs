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
    //let points_per_level = vec![200, 50, 10];
    //let city_percentage = vec![1.0, 0.4, 0.4, 0.4];
    //let radii = vec![400.0, 600.0, 50.0];
    //
    //let g = hierachical_disks::generate_circle_center_graph_v2(
    //    &points_per_level,
    //    &city_percentage,
    //    &radii,
    //);
}
