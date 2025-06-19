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

use chrono::Local;
use geo::Point;
use graph::knn::knn;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

use graph::example::*;
use graph::geometric_graph::GeometricGraph;
use graph::voronoi::prune_graph;
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, gabriel_graph, grid, hierachical_delaunay, hierachical_disks, highway,
    knn, nested_grid, noise, relative_neighborhood, tree, voronoi,
};

fn main() {
    let p = noise::get_noise_points_scales(350_000_000, &[8.0, 16.0, 32.0, 64.0]);
    println!("{}\tGenerated points", Local::now());
    let g = relative_neighborhood::relative_neighborhood_points(p);
    println!("{}\tGenerated relative neighborhood graph", Local::now());
    g.inertial_flowcutter("noise_low_only_350m");
    println!("{}\tInertial flowcutter", Local::now());
}
