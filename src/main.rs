pub mod bidirectional;
pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod lca;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

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
    tree, voronoi,
};
use separator::Mode::*;

fn main() {
    let x1 = 1000.;
    let x2 = x1 / 50_f64.sqrt() * 2.;
    let x3 = x2 / 50_f64.sqrt() * 1.5;
    let x4 = x3 / 50_f64.sqrt() * 1.;

    let g = hierachical_delaunay::pruned_hierachical_delaunay(
        &[1.0, 0.4, 0.2, 0.1],
        &[50, 50, 50, 50],
        &[x1, x2, x3, x4],
    );
    // g.inertial_flowcutter("hierachical_delaunay_tmp");
    g.graph.info();
    g.visualize("hierachical_delaunay_tmp");
}
