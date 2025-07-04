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
use graph::delaunay::{dynamic_length_restriced_delaunay, length_restricted_delaunay};
use graph::example::*;
use graph::gabriel_graph::{gabriel_graph, gabriel_graph_points};
use graph::geometric_graph::GeometricGraph;
use graph::voronoi::{
    prune_graph, prune_graph_parallel, prune_graph_spanner, prune_graph_spanner_parallel_approx,
    prune_v3, pruned_delaunay,
};
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, gabriel_graph, grid, hierachical_delaunay, hierachical_disks, highway,
    knn, nested_grid, noise, relative_neighborhood, tree, voronoi,
};
use ordered_float::OrderedFloat;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

fn main() {
    let g = GeometricGraph::from_osm_xml("/algoDaten/graphs/osm/karlsruhe-090826.osm").unwrap();
    g.inertial_flowcutter("karlsruhe-osm");

}
