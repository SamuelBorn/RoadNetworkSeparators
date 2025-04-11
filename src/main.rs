pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use cch::{compute_separator_sizes_from_order, get_top_level_separator};
use graph::example::{self, *};
use graph::planar::planarize;
use graph::{cbrt_maximal, delaunay, grid, hierachical_disks, highway, nested_grid, voronoi};
use graph::{geometric_graph::GeometricGraph, Graph};
use library::{
    read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec, write_text_vec,
};
use rayon::prelude::*;
use separator::{get_ord, Mode::*};
use std::path::Path;

fn main() {
    let mut g = GeometricGraph::from_file(Path::new("./output/europe_planar")).unwrap();
    g.inertial_flowcutter("europe_planar");
}
