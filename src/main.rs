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
    let n = 200_000;
    let m = (n as f64 * 1.55) as usize;
    let g = local::tree_locality(n, m, |x| (x as f64).powf(-3.35));

    [-2.9, -3.0, -3.1, -3.2, -3.3, -3.4, -3.5, -3.6]
        .par_iter()
        .for_each(|&pow| {
            let g = local::tree_locality(n, m, |x| (x as f64).powf(pow));
            println!("finished building {pow}");
            g.flowcutter(&format!("tree_locality_new_{pow}.json"));
            println!("finished flowcutter {pow}");
        });
}
