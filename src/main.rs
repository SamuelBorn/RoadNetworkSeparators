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
use graph::{cbrt_maximal, delaunay, grid, hierachical_disks, highway, nested_grid, voronoi};
use graph::{geometric_graph::GeometricGraph, Graph};
use hashbrown::HashSet;
use library::{
    read_binary_vec, read_text_vec, read_to_usize_vec, write_binary_vec, write_text_vec,
};
use rayon::prelude::*;
use separator::{get_ord, Mode::*};
use std::path::Path;

fn main() {
    let mut g = geometric_germany();

    while g.graph.get_num_nodes() > 100 {
        let sep = g.graph.get_separator_wrapper(Eco);

        let mut planar = g.clone();
        planarize(&mut planar);
        let subgraphs = planar.graph.get_subgraphs(&sep);
        let large_subgraphs = subgraphs.iter().filter(|s| s.get_num_nodes() > 100).count();
        println!("({},{}) {}", g.graph.get_num_nodes(), sep.len(), large_subgraphs > 1);

        for n in sep {
            g.graph.clear_vertex_edges(n);
        }
        g = g.largest_connected_component();
    }
}
