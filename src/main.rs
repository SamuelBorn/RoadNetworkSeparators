pub mod cch;
pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use graph::delaunay::{length_restricted_delaunay, random_delaunay};
use graph::geometric_graph::{GeometricGraph, Position, AABB};
use graph::planar::naive_find_intersections;
use graph::{delaunay, example, grid, Graph};
use library::{read_bin_u32_vec_to_usize, read_binary_vec, read_text_vec, write_binary_vec};
use separator::Mode::*;

fn main() {
    //let mut g = example::karlsruhe();
    //let ord = read_bin_u32_vec_to_usize(Path::new("output/ord_karlsruhe.bin"));
    let g = example::germany_server();
    g.to_file(Path::new("output/germany"));

    //let ord = read_bin_u32_vec_to_usize(Path::new("output/ord_germany_new.bin"));
    //cch::compute_separator_sizes_from_order(&g, &ord);
}
