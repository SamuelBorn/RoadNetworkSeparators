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
    let mut g = geometric_germany();
    fs::write("./output/germany_pos", g.positions.iter().map(|p| format!("{} {}\n", p.x(), p.y())).collect::<String>())
        .expect("Unable to write file");

    // g.contract_degree_2_nodes();
    // let g = g.largest_connected_component();
    // g.recurse_diameter(Some(Path::new("./output/diameter/germany_ifub")));
}
