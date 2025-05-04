pub mod bidirectional;
pub mod cch;
pub mod graph;
pub mod kruskal;
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
use local::generate_random_connected;
use ordered_float::Pow;
use rayon::prelude::*;
use separator::{get_ord, print_binned_statistic, Mode::*};
use std::fs;
use std::path::Path;
use std::sync::Arc;

fn main() {




    return;
    let city_percentage = vec![1.0, 0.001, 0.01, 0.3];
    let points_per_level = vec![1000, 1000, 200, 50];
    let radii = vec![5000., 1000., 100., 20.];
    let mut g = hierachical_delaunay::generate_hierachical_delaunay(
        &city_percentage,
        &points_per_level,
        &radii,
    );
    prune_graph(&mut g, 2.0);
    //
    //g.visualize("hierachical_delaunay");
    //g.graph.recurse_separator(Fast, None);
    //

    //let s = g.inertial_flowcutter("tmp");
    //print_binned_statistic(s, 10);
}
