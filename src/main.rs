pub mod bidirectional;
pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod lca;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use geo::Rect;
use ordered_float::Pow;
use rayon::prelude::*;
use std::fs;
use std::path::Path;

use cch::{compute_separator_sizes_from_order, get_top_level_separator};
use graph::example::{self, *};
use graph::geometric_graph::GeometricGraph;
use graph::hierachical_delaunay::random_pruned_hierachical_delaunay;
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, grid, hierachical_delaunay, hierachical_disks, highway, nested_grid,
    tree, voronoi, noise
};
use separator::Mode::*;

fn main() {
    let g = noise::noise_scales(5_000_000, &[4.0, 8.0, 16.0, 32.0]);
    g.inertial_flowcutter("noise_low_only");

    let g = noise::noise_scales(5_000_000, &[64.0, 128.0, 256.0, 512.0, 1024.0]);
    g.inertial_flowcutter("noise_high_only");
}
