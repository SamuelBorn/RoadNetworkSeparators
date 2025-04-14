pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use cch::{compute_separator_sizes_from_order, get_top_level_separator};
use graph::example::{self, *};
use graph::planar::planarize;
use graph::{
    cbrt_maximal, delaunay, grid, hierachical_delaunay, hierachical_disks, highway, nested_grid,
    voronoi,
};
use graph::{geometric_graph::GeometricGraph, Graph};
use hashbrown::HashSet;
use library::{
    read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec, write_text_vec,
};
use separator::{get_ord, print_binned_statistic, Mode::*};
use std::path::Path;

fn main() {
    let city_percentage = vec![1.0, 0.01, 0.5, 0.5];
    let points_per_level = vec![1000, 100, 20, 20];
    let radii = vec![1000., 50., 30., 3.];
    let g = hierachical_delaunay::generate_hierachical_delaunay(
        &city_percentage,
        &points_per_level,
        &radii,
    );
    g.save(Path::new("./output/graphs/hierachical_delaunay/"));

    let g = geometric_karlsruhe();
    let s = g.inertial_flowcutter("tmp");
    print_binned_statistic(s, 10);
}
