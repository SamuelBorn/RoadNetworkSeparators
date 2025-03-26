pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use cch::compute_separator_sizes_from_order;
use graph::example::*;
use graph::{cbrt_maximal, delaunay, grid, highway, nested_grid, voronoi};
use graph::{geometric_graph::GeometricGraph, Graph};
use library::{read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec};
use separator::{get_ord, Mode::*};
use std::path::Path;

fn main() {}
