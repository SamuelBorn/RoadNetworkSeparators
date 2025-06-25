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
use graph::delaunay::delaunay;
use graph::knn::knn;
use graph::relative_neighborhood::relative_neighborhood;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

use graph::example::*;
use graph::geometric_graph::GeometricGraph;
use graph::voronoi::{prune_graph, prune_graph_parallel, pruned_delaunay};
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, gabriel_graph, grid, hierachical_delaunay, hierachical_disks, highway,
    knn, nested_grid, noise, relative_neighborhood, tree, voronoi,
};

fn main() {
    let mut num_iter = 1000;

    // let mut g = europe();
    // let n = g.get_num_nodes();
    // g.contract_and_llc();
    // g.hop_overview(num_iter, "hops_europe");
    // println!("{}\tHop overview done", Local::now());
    //
    // let p = noise::get_noise_points(g.get_num_nodes());
    // let g = delaunay::delaunay(&p);
    // g.graph.hop_overview(num_iter, "hops_noise_delaunay");
    // println!("{}\tDelaunay triangulation done", Local::now());
    //
    let p = noise::get_noise_points(18_000_000);
    let mut g = pruned_delaunay(&p, 2.5);
    g.graph.contract_and_llc();
    // g.graph.hop_overview(num_iter, "hops_noise_pruned");
    dbg!(g.graph.get_num_nodes());
    println!("{}\tPruning done", Local::now());
    //
    // let mut g = relative_neighborhood::relative_neighborhood_points(p);
    // g.graph.contract_and_llc();
    // g.graph.hop_overview(num_iter, "hops_noise_rng");
    // println!("{}\tRelative neighborhood done", Local::now());
}
