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
    let sites1 = 50;
    let sites2 = 30;
    let sites3 = 20;
    let sites4 = 10;
    let r1 = 1000.;
    let r2 = r1 / (sites1 as f64).sqrt() * 3.;
    let r3 = r2 / (sites2 as f64).sqrt() * 2.;
    let r4 = r3 / (sites3 as f64).sqrt() * 1.;
    println!("x1: {}, x2: {}, x3: {}, x4: {}", r1, r2, r3, r4);

    let g = hierachical_delaunay::pruned_hierachical_delaunay(
        &[1.0, 0.3, 0.2, 0.1],
        &[sites1, sites2, sites3, sites4],
        &[r1, r2, r3, r4],
    );
    g.graph.info();

    // g.inertial_flowcutter("tmp");
    g.visualize("tmp");
}
