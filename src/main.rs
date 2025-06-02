pub mod bidirectional;
pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod lca;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;
pub mod osm;

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
    noise, tree, voronoi,
};
use separator::Mode::*;

fn main() {
    // let g = europe();
    // let hops = g.hop_overview(1000);
    // library::write_text_vec(&hops, Path::new("output/hops_europe.txt"));

    let g = Graph::from_osm_xml(Path::new("../Graphs/karlsruhe-090826.osm")).unwrap();
    let g = g.largest_connected_component();
    g.info();
}
