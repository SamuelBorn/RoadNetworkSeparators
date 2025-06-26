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
use graph::delaunay::delaunay_points;
use graph::knn::knn;
use graph::noise::noise;
use graph::relative_neighborhood::relative_neighborhood;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

use graph::example::*;
use graph::geometric_graph::GeometricGraph;
use graph::voronoi::{prune_graph, prune_graph_parallel, prune_graph_spanner, pruned_delaunay};
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, gabriel_graph, grid, hierachical_delaunay, hierachical_disks, highway,
    knn, nested_grid, noise, relative_neighborhood, tree, voronoi,
};

fn main() {
    let p = noise::get_noise_points(18_000_000);
    let mut g = delaunay::delaunay_points(&p);
    prune_graph_spanner(&mut g, 10.0);
    g.graph.info(); 
    g.graph.hop_overview(1000, "hops_voronoi_spanner");
}
