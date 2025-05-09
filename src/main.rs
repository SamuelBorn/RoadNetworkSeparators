pub mod bidirectional;
pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod lca;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use cch::{compute_separator_sizes_from_order, get_top_level_separator};
use graph::example::{self, *};
use graph::geometric_graph::GeometricGraph;
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, grid, hierachical_delaunay, hierachical_disks, highway, nested_grid,
    tree, voronoi,
};
use separator::Mode::*;

fn main() {
    let mut g = hierachical_delaunay::pruned_hierachical_delaunay(
        &[1.0, 0.01, 0.5, 0.5],
        &[500, 30, 120, 100],
        &[5000., 500., 350., 20.],
    );
    g.graph.info();
    // g.visualize("hierachical_delaunay");
    // g.graph.recurse_separator(Fast, None);
    g.inertial_flowcutter("hierachical_delaunay");
}
