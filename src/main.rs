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
use geo::Point;
use graph::example::{self, *};
use graph::planar::planarize;
use graph::tree::generate_random_tree;
use graph::voronoi::prune_graph;
use graph::{
    cbrt_maximal, delaunay, grid, hierachical_delaunay, hierachical_disks, highway, nested_grid,
    voronoi,
};
use graph::{geometric_graph::GeometricGraph, Graph};
use hashbrown::HashSet;
use library::{
    read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec, write_text_vec,
};
use local::{generate_local_points, generate_random_connected};
use ordered_float::Pow;
use rayon::prelude::*;
use separator::{get_ord, print_binned_statistic, Mode::*};
use std::fs;
use std::path::Path;
use std::sync::Arc;

fn main() {
    // let mut diam_walk = String::new();
    let mut diam_kruskal = String::new();
    for i in 1..=10 {
        println!("i: {}", i);
        // let g = generate_random_tree(i * 100000);
        // diam_walk.push_str(&format!("{} {}\n", g.get_num_nodes(), g.get_diameter()));

        let g = kruskal::get_mst(i * 100000);
        diam_kruskal.push_str(&format!(
            "{} {}\n",
            g.graph.get_num_nodes(),
            g.graph.get_diameter()
        ));
    }
    // fs::write("output/diameter/random_walk.txt", diam_walk).unwrap();
    fs::write("output/diameter/kruskal.txt", diam_kruskal).unwrap();

    return;
    let mut g = hierachical_delaunay::pruned_hierachical_delaunay(
        &[1.0, 0.001, 0.01, 0.3],
        &[1000, 1000, 200, 50],
        &[5000., 1000., 100., 20.],
    );
}
