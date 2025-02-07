pub mod graph;
pub mod library;
pub mod local;
pub mod separator;

use std::fs;
use std::path::Path;

use graph::delaunay::{length_restricted_delaunay, random_delaunay};
use graph::geometric_graph::{GeometricGraph, Position, AABB};
use graph::planar::naive_find_intersections;
use graph::{delaunay, example, grid, Graph};
use library::{read_binary_vec, read_text_vec, write_binary_vec};
use separator::{traverse_separator_tree, Mode::*};

fn main() {
    let mut g = GeometricGraph::from_file(Path::new("../Graphs/karlsruhe"))
        .unwrap()
        .graph;

    let ord = read_binary_vec::<u32>(Path::new("output/order_karlsruhe.bin"))
        .unwrap()
        .iter()
        .map(|&x| x as usize)
        .collect::<Vec<_>>();

    g.chordalize(&ord);
    let tree = g.get_lowest_neighbor_tree_top_down(&ord);
    traverse_separator_tree(&tree, *ord.last().unwrap());
}
