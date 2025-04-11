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
    let mut g = geometric_karlsruhe();
    g.inertial_flowcutter("karlsruhe");
    let ord = read_to_usize_vec(Path::new("./output/ord/karlsruhe"));
    let top_level = get_top_level_separator(&g.graph, &ord);

    planarize(&mut g);
    g.inertial_flowcutter("karlsruhe_planar");
    let ord = read_to_usize_vec(Path::new("./output/ord/karlsruhe_planar"));
    let top_level_planar = get_top_level_separator(&g.graph, &ord);

    write_text_vec(&top_level, Path::new("output/karlsruhe_top_level_sep.txt"));
    write_text_vec(&top_level_planar, Path::new("output/karlsruhe_top_level_sep_planar.txt"));
}
