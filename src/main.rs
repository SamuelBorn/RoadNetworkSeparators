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
    tree, voronoi,
};
use separator::Mode::*;

fn main() {
    let f1 = 1.0;
    let f2 = 0.3;
    let f3 = 0.2;
    let f4 = 0.1;

    let s1 = 50;
    let s2 = 50;
    let s3 = 50;
    let s4 = 50;

    let r1 = 1000.0;
    let r2 = 424.0;
    let r3 = 120.0;
    let r4 = 17.0;

    println!("fract: {:.1}, {:.1}, {:.1}, {:.1}", f1, f2, f3, f4);
    println!("sites: {:.0}, {:.0}, {:.0}, {:.0}", s1, s2, s3, s4);
    println!("radii: {:.0}, {:.0}, {:.0}, {:.0}\n", r1, r2, r3, r4);

    let g = hierachical_delaunay::pruned_hierachical_delaunay(
        &[f1, f2, f3, f4],
        &[s1, s2, s3, s4],
        &[r1, r2, r3, r4],
    );
    g.graph.info();

    g.inertial_flowcutter("tmp");
    // g.visualize("tmp");
}
