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
    let p = noise::get_noise_points_scales(
        2usize.pow(28),
        &[
            2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0, 2048.0, 4096.0, 8192.0,
            16384.0, 32768.0, 65536.0,
        ],
    );
    println!("Generated points");
    let g = relative_neighborhood::relative_neighborhood_points(p);
    println!("Generated relative neighborhood graph");
    g.inertial_flowcutter("noise_2pow28");
}
