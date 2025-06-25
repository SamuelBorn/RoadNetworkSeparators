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
use graph::voronoi::{prune_graph, prune_graph_parallel};
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, gabriel_graph, grid, hierachical_delaunay, hierachical_disks, highway,
    knn, nested_grid, noise, relative_neighborhood, tree, voronoi,
};

fn main() {
    let scales = &[4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0, 2048.0, 4096.0];
    let p = noise::get_noise_points_scales(18_000_000, scales);
    let mut g = delaunay::delaunay(&p);
    prune_graph_parallel(&mut g, 2.5);
    g.graph.hop_overview_contracted_bins(1000, 100, "noise_18m_hops_custom_prune");
}
