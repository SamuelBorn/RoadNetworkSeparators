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

use geo::{Point, Rect};
use graph::voronoi::prune_graph;
use ordered_float::Pow;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs;
use std::path::Path;

use cch::{compute_separator_sizes_from_order, get_top_level_separator};
use graph::example::{self, *};
use graph::geometric_graph::GeometricGraph;
use graph::hierachical_delaunay::random_pruned_hierachical_delaunay;
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, grid, hierachical_delaunay, hierachical_disks, highway, knn,
    nested_grid, noise, tree, voronoi, relative_neighborhood, gabriel_graph,
};
use separator::Mode::*;

fn main() {
    let p = library::random_points_in_circle(Point::new(100., 100.), 1., 1_000);

    let g = gabriel_graph::gabriel_graph_points(&p);
    g.visualize("gabriel_graph");

    let g = delaunay::delaunay(&p);
    g.visualize("delaunay");

    let g = relative_neighborhood::relative_neighborhood_points(&p);
    g.visualize("relative_neighborhood");
}
