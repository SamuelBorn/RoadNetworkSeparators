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
use rayon::prelude::*;
use separator::{get_ord, Mode::*};
use std::path::Path;

fn main() {
    let g = example::europe();
    let sep = g
        .get_separator_wrapper(Eco)
        .into_iter()
        .collect::<Vec<_>>();

    println!("{:?}", sep);

    library::write_text_vec(&sep, Path::new("output/europe_top_level_sep.txt"));
}
