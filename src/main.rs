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
    let g = geometric_europe();
    println!("{}\teurope", Local::now());
    g.distance_overview_contracted_bins(10_000, 100, "geometric_europe_distance_overview_contracted_bins");
    println!("{}\teurope distance", Local::now());
    g.graph.hop_overview_contracted_bins(10_000, 100, "geometric_europe_hop_overview_contracted_bins");
    println!("{}\teurope hop", Local::now());

    let g = noise::noise(g.graph.get_num_nodes());
    println!("{}\tnoise", Local::now());
    g.distance_overview_contracted_bins(10_000, 100, "noise_18m_distance_overview_contracted_bins");
    println!("{}\tnoise distance", Local::now());
    g.graph.hop_overview_contracted_bins(10_000, 100, "noise_18m_hop_overview_contracted_bins");
    println!("{}\tnoise hop", Local::now());
}
