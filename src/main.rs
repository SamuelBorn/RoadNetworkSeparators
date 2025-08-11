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

use graph::{
    cbrt_maximal, delaunay, example::*, gabriel_graph, grid, hierachical_delaunay,
    hierachical_disks, highway, knn, nested_grid, noise, relative_neighborhood, tree, voronoi,
};
use graph::{geometric_graph::GeometricGraph, Graph};
use std::path::Path;

fn main() {
    let p = noise::get_noise_points(10_000_000);
    let g = relative_neighborhood::relative_neighborhood_points(p);
    g.inertial_flowcutter("noise");
}
