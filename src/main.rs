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
    let s1 = 70;
    let s2 = 60;
    let s3 = 50;
    let s4 = 40;

    let r1 = 1000.;
    let r2 = 1.5 * r1 / (s1 as f64).sqrt();
    let r3 = 1.5 * r2 / (s2 as f64).sqrt();
    let r4 = 1.5 * r3 / (s3 as f64).sqrt();

    let f1 = 1.0;
    let f2 = 0.5;
    let f3 = 0.4;
    let f4 = 0.3;

    println!("exp fraction: {:.1}, {:.1}, {:.1}, {:.1}", f1, f2, f3, f4);
    println!("sites: {:.1}, {:.1}, {:.1}, {:.1}", s1, s2, s3, s4);
    println!("radii: {:.1}, {:.1}, {:.1}, {:.1}", r1, r2, r3, r4);

    let t1 = s1 as f64;
    let t2 = t1 + t1 * f2 * s2 as f64;
    let t3 = t2 + t2 * f3 * s3 as f64;
    let t4 = t3 + t3 * f4 * s4 as f64;
    println!("total size: {:.1}", t4);

    let g = hierachical_delaunay::pruned_hierachical_delaunay(
        &[f1, f2, f3, f4],
        &[s1, s2, s3, s4],
        &[r1, r2, r3, r4],
    );
    g.graph.info();

    g.inertial_flowcutter("tmp");
    // g.visualize("tmp");
}
