pub mod bidirectional;
pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod lca;
pub mod library;
pub mod local;
pub mod osm;
pub mod random_set;
pub mod separator;

use geo::{Point, Rect};
use ordered_float::Pow;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

use cch::{compute_separator_sizes_from_order, get_top_level_separator};
use graph::example::{self, *};
use graph::geometric_graph::GeometricGraph;
use graph::hierachical_delaunay::random_pruned_hierachical_delaunay;
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, grid, hierachical_delaunay, hierachical_disks, highway, knn,
    nested_grid, noise, tree, voronoi,
};
use separator::Mode::*;

fn main() {
    let p = noise::get_noise_points(1_000_000);
    let g = knn::knn_points(&p, 3);
    let g = g.largest_connected_component();
    g.inertial_flowcutter("noise_knn");
}
