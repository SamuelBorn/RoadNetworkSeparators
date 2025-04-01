pub mod cch;
pub mod graph;
pub mod kruskal;
pub mod library;
pub mod local;
pub mod random_set;
pub mod separator;

use cch::compute_separator_sizes_from_order;
use graph::example::{self, *};
use graph::planar::planarize;
use graph::{cbrt_maximal, delaunay, grid, hierachical_disks, highway, nested_grid, voronoi};
use graph::{geometric_graph::GeometricGraph, Graph};
use library::{
    read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec, write_text_vec,
};
use rayon::prelude::*;
use separator::{get_ord, Mode::*};
use std::path::Path;

fn main() {
    //let city_percentage = vec![1.0, 0.4, 0.4, 0.4];
    //let radii = vec![1500.0, 400.0, 60.0, 10.0];
    //let points_per_level = vec![1000, 270, 50, 10];

    [50, 60, 70, 80, 90, 100, 110, 120, 130, 140]
        .par_iter()
        .for_each(|&last_radius| {
            let city_percentage = vec![1.0, 0.5, 0.5, 0.5];
            let radii = vec![last_radius as f64, 20.0, 5.0, 1.0];
            let points_per_level = vec![50, 50, 50, 50];

            let g = hierachical_disks::generate_circle_center_graph_v2(
                &points_per_level,
                &city_percentage,
                &radii,
            );

            g.graph.info();
            g.save(Path::new(format!("hierachical_disks_{}", last_radius).as_str()));
            //g.inertial_flowcutter(format!("hierachical_disks_{}", last_radius).as_str());
        });

    
    [50, 60, 70, 80, 90, 100, 110, 120, 130, 140]
        .par_iter()
        .for_each(|&last_radius| {
            let g = GeometricGraph::from_file(Path::new(format!("hierachical_disks_{}", last_radius).as_str())).unwrap();
            g.inertial_flowcutter(format!("hierachical_disks_{}", last_radius).as_str());
        });
            

}
