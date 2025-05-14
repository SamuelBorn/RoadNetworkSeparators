pub mod bidirectional;
pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod lca;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use ordered_float::Pow;
use rayon::prelude::*;
use std::path::Path;

use cch::{compute_separator_sizes_from_order, get_top_level_separator};
use graph::example::{self, *};
use graph::geometric_graph::GeometricGraph;
use graph::hierachical_delaunay::random_pruned_hierachical_delaunay;
use graph::Graph;
use graph::{
    cbrt_maximal, delaunay, grid, hierachical_delaunay, hierachical_disks, highway, nested_grid,
    tree, voronoi,
};
use separator::Mode::*;

fn main() {
    let n = 1_000_000;
    let m = n * 5 / 4;

    (0..48)
        .into_par_iter()
        .map(|i| 2.0 + 0.1 * i as f64)
        .for_each(|p| {
            let g = local::tree_locality_bounded(n, m, |x: usize| (x as f64).powf(-p));
            g.kahip(&format!("local_graph_bounded_{}", (10. * p).round()));
        });

    // let x1 = 1000.;
    // let x2 = x1 / 50_f64.sqrt() * 2.;
    // let x3 = x2 / 50_f64.sqrt() * 1.5;
    // let x4 = x3 / 50_f64.sqrt() * 1.;
    //
    // let g = hierachical_delaunay::pruned_hierachical_delaunay(
    //     &[1.0, 0.4, 0.2, 0.1],
    //     &[50, 50, 50, 50],
    //     &[x1, x2, x3, x4],
    // );
    // g.inertial_flowcutter("hierachical_delaunay_tmp");
    // g.visualize("hierachical_delaunay_tmp");

    // for city_percentage in [0.1, 0.3, 0.4, 0.5, 0.6, 0.7, 0.9] {
    //     let g = hierachical_delaunay::random_pruned_hierachical_delaunay(
    //         &[1.0, 0.01, city_percentage, 0.5],
    //         &[500, 30, 120, 100],
    //         &[5000., 500., 350., 20.],
    //     );
    //     g.inertial_flowcutter(&format!(
    //         "hierachical_delaunay_city_{}",
    //         (city_percentage * 100.0) as u32
    //     ));
    // }
    // for points_per_level in [20, 80, 100, 120, 140, 160, 200] {
    //     let g = hierachical_delaunay::pruned_hierachical_delaunay(
    //         &[1.0, 0.01, 0.5, 0.5],
    //         &[500, 30, points_per_level, 100],
    //         &[5000., 500., 350., 20.],
    //     );
    //     g.inertial_flowcutter(&format!("hierachical_delaunay_points_{}", points_per_level));
    // }
    // for radii in [100, 200, 250, 300, 350, 400, 450, 550] {
    //     let g = hierachical_delaunay::pruned_hierachical_delaunay(
    //         &[1.0, 0.01, 0.5, 0.5],
    //         &[500, 30, 120, 100],
    //         &[5000., 500., radii as f64, 20.],
    //     );
    //     g.inertial_flowcutter(&format!("hierachical_delaunay_radii_{}", radii));
    // }
}
